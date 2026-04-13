from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks, UploadFile, File, Query
from fastapi.responses import StreamingResponse
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, insert, update, delete
from sqlalchemy.exc import IntegrityError
from pydantic import BaseModel
from typing import Optional, List
import io
from datetime import datetime, timezone

from app.core.database import get_db
from app.core import redis_client as rc
from app.core.audit import log_audit_task
from app.core.bulk_operations import BulkOperationService
from app.models.student import Student
from app.dependencies.auth import require_permission, get_current_user

router = APIRouter(prefix="/bulk", tags=["Bulk Operations"])

# Cache key
_STUDENTS_LIST_KEY = "cache:students:list"

# ═══════════════════════════════════════════════════════════════
# HIGH PRIORITY FIX #5: Bounded Export Limits
# ═══════════════════════════════════════════════════════════════
MAX_EXPORT_ROWS = 50000  # Maximum rows per export (prevents OOM)
MAX_PAGE_SIZE = 5000     # Maximum rows per paginated fetch


class BulkImportResponse(BaseModel):
    """Response for bulk import operations."""
    total_rows: int
    successful: int
    failed: int
    errors: List[str] = []
    details: Optional[List[dict]] = None


class BulkDeleteRequest(BaseModel):
    """Request to bulk delete students."""
    student_ids: List[str]


class BulkUpdateRequest(BaseModel):
    """Request to bulk update students."""
    student_ids: List[str]
    updates: dict  # Fields to update


@router.get("/students/export/csv")
async def export_students_csv(
    db: AsyncSession = Depends(get_db),
    _=Depends(require_permission("students:read")),
):
    """
    Export students as CSV file with streaming and limits.
    
    HIGH PRIORITY FIX #5: Prevents OOM by:
    - Limiting total export size (50k rows max)
    - Streaming response (yields chunks, not full load)
    - Paginated database queries (5k per page)
    """
    async def generate_csv():
        """Generate CSV by streaming chunks from database."""
        offset = 0
        total_rows = 0
        is_first_page = True
        
        while total_rows < MAX_EXPORT_ROWS:
            # Fetch page of students
            result = await db.execute(
                select(Student)
                .order_by(Student.created_at.desc())
                .limit(MAX_PAGE_SIZE)
                .offset(offset)
            )
            students = result.scalars().all()
            
            if not students:
                break  # No more data
            
            # Convert to list of dicts
            students_data = [
                {
                    'first_name': s.first_name,
                    'last_name': s.last_name,
                    'email': s.email,
                    'class_name': s.class_name,
                    'roll_no': s.roll_no,
                    'admission_no': s.admission_no,
                    'mobile_numbers': s.mobile_numbers,
                    'address': s.address,
                    'city': s.city,
                    'state': s.state,
                    'zip_code': s.zip_code,
                    'date_of_birth': s.date_of_birth,
                    'enrollment_date': s.enrollment_date,
                    'is_active': s.is_active,
                    'notes': s.notes,
                }
                for s in students
            ]
            
            # Generate CSV for this page (with header on first page)
            csv_content = BulkOperationService.students_to_csv(students_data)
            
            if is_first_page:
                # First page: include everything (header + data)
                yield csv_content
                is_first_page = False
            else:
                # Subsequent pages: strip header and yield body only
                lines = csv_content.strip().split('\n')
                if len(lines) > 1:
                    yield '\n'.join(lines[1:]) + '\n'
            
            total_rows += len(students)
            offset += MAX_PAGE_SIZE
        
        # Note: If export was truncated
        if total_rows >= MAX_EXPORT_ROWS:
            msg = f"\n# Export limited to {MAX_EXPORT_ROWS} rows. Use filters or pagination for larger exports.\n"
            yield msg
    
    try:
        return StreamingResponse(
            generate_csv(),
            media_type="text/csv",
            headers={"Content-Disposition": "attachment; filename=students_export.csv"}
        )
    
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to export students: {str(e)}")


@router.get("/students/import-template")
async def get_import_template(
    _=Depends(require_permission("students:write")),
):
    """Download CSV template for importing students."""
    try:
        csv_template = BulkOperationService.generate_csv_template()
        
        return StreamingResponse(
            iter([csv_template]),
            media_type="text/csv",
            headers={"Content-Disposition": "attachment; filename=students_template.csv"}
        )
    
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to generate template: {str(e)}")


@router.post("/students/import/csv")
async def import_students_csv(
    file: UploadFile = File(...),
    background_tasks: BackgroundTasks = None,
    db: AsyncSession = Depends(get_db),
    current_user=Depends(require_permission("students:write")),
):
    """
    Import students from CSV file.
    
    Returns summary of successful/failed imports with error details.
    """
    if not file.filename.endswith('.csv'):
        raise HTTPException(status_code=400, detail="Only CSV files are allowed")
    
    try:
        # Read file content
        content = await file.read()
        csv_content = content.decode('utf-8')
        
        # Parse CSV
        students_data, parse_errors = BulkOperationService.csv_to_students(csv_content)
        
        if parse_errors and not students_data:
            return BulkImportResponse(
                total_rows=0,
                successful=0,
                failed=0,
                errors=parse_errors,
            )
        
        # Import students into database
        successful = 0
        failed = 0
        import_errors = []
        imported_student_ids = []
        
        for student_data in students_data:
            try:
                # Check if email already exists
                existing = await db.execute(
                    select(Student.id).where(Student.email == student_data['email'])
                )
                if existing.scalar_one_or_none():
                    failed += 1
                    import_errors.append(f"Email already exists: {student_data['email']}")
                    continue
                
                # Create new student
                new_student = Student(
                    first_name=student_data['first_name'],
                    last_name=student_data['last_name'],
                    email=student_data['email'],
                    class_name=student_data.get('class_name'),
                    roll_no=student_data.get('roll_no'),
                    admission_no=student_data.get('admission_no'),
                    mobile_numbers=student_data.get('mobile_numbers', []),
                    address=student_data.get('address'),
                    city=student_data.get('city'),
                    state=student_data.get('state'),
                    zip_code=student_data.get('zip_code'),
                    date_of_birth=student_data.get('date_of_birth'),
                    is_active=student_data.get('is_active', True),
                    notes=student_data.get('notes'),
                )
                
                # Handle enrollment_date if provided
                if student_data.get('enrollment_date'):
                    try:
                        enrollment_date = datetime.fromisoformat(student_data['enrollment_date'])
                        if enrollment_date.tzinfo is None:
                            enrollment_date = enrollment_date.replace(tzinfo=timezone.utc)
                        new_student.enrollment_date = enrollment_date
                    except ValueError:
                        pass  # Skip invalid dates
                
                db.add(new_student)
                await db.flush()
                imported_student_ids.append(new_student.id)
                successful += 1
            
            except IntegrityError as e:
                await db.rollback()
                failed += 1
                import_errors.append(f"Database error for {student_data['email']}: Duplicate entry or constraint violation")
            except Exception as e:
                await db.rollback()
                failed += 1
                import_errors.append(f"Error importing {student_data.get('email', 'unknown')}: {str(e)}")
        
        # Commit all successful imports
        try:
            await db.commit()
        except Exception as e:
            await db.rollback()
            raise HTTPException(status_code=500, detail=f"Failed to save imports: {str(e)}")
        
        # Invalidate cache and log
        if background_tasks:
            background_tasks.add_task(rc.invalidate_keys, _STUDENTS_LIST_KEY)
            background_tasks.add_task(
                log_audit_task,
                current_user.id,
                "BULK_IMPORT_STUDENTS",
                "students",
                f"imported_{successful}_students"
            )
        
        return BulkImportResponse(
            total_rows=len(students_data) + len(parse_errors),
            successful=successful,
            failed=failed,
            errors=import_errors[:50],  # Limit to first 50 errors
        )
    
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to process CSV: {str(e)}")


@router.post("/students/delete")
async def bulk_delete_students(
    body: BulkDeleteRequest,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
    current_user=Depends(require_permission("students:delete")),
):
    """Bulk delete students by IDs."""
    if not body.student_ids:
        raise HTTPException(status_code=400, detail="No student IDs provided")
    
    if len(body.student_ids) > 1000:
        raise HTTPException(status_code=400, detail="Cannot delete more than 1000 students at once")
    
    try:
        # Delete students
        result = await db.execute(
            delete(Student).where(Student.id.in_(body.student_ids))
        )
        deleted_count = result.rowcount
        
        await db.commit()
        
        # Invalidate cache and log
        background_tasks.add_task(rc.invalidate_keys, _STUDENTS_LIST_KEY)
        background_tasks.add_task(
            log_audit_task,
            current_user.id,
            "BULK_DELETE_STUDENTS",
            "students",
            f"deleted_{deleted_count}_students"
        )
        
        return {
            "message": f"Successfully deleted {deleted_count} students",
            "deleted_count": deleted_count,
        }
    
    except Exception as e:
        await db.rollback()
        raise HTTPException(status_code=500, detail=f"Failed to delete students: {str(e)}")


@router.post("/students/update")
async def bulk_update_students(
    body: BulkUpdateRequest,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
    current_user=Depends(require_permission("students:write")),
):
    """Bulk update students by IDs."""
    if not body.student_ids:
        raise HTTPException(status_code=400, detail="No student IDs provided")
    
    if not body.updates:
        raise HTTPException(status_code=400, detail="No updates provided")
    
    if len(body.student_ids) > 1000:
        raise HTTPException(status_code=400, detail="Cannot update more than 1000 students at once")
    
    # Whitelist allowed fields to update
    allowed_fields = {
        'class_name', 'roll_no', 'admission_no', 'address',
        'city', 'state', 'zip_code', 'is_active', 'notes'
    }
    
    # Filter updates to only allowed fields
    safe_updates = {k: v for k, v in body.updates.items() if k in allowed_fields}
    
    if not safe_updates:
        raise HTTPException(status_code=400, detail="No valid fields to update")
    
    try:
        # Add updated_at timestamp
        safe_updates['updated_at'] = datetime.now(timezone.utc)
        
        # Update students
        result = await db.execute(
            update(Student)
            .where(Student.id.in_(body.student_ids))
            .values(**safe_updates)
        )
        updated_count = result.rowcount
        
        await db.commit()
        
        # Invalidate cache and log
        background_tasks.add_task(rc.invalidate_keys, _STUDENTS_LIST_KEY)
        background_tasks.add_task(
            log_audit_task,
            current_user.id,
            "BULK_UPDATE_STUDENTS",
            "students",
            f"updated_{updated_count}_students"
        )
        
        return {
            "message": f"Successfully updated {updated_count} students",
            "updated_count": updated_count,
        }
    
    except Exception as e:
        await db.rollback()
        raise HTTPException(status_code=500, detail=f"Failed to update students: {str(e)}")
