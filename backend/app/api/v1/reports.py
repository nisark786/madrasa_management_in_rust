"""Report generation and management API endpoints."""
from typing import Optional, List, Dict, Any
from fastapi import APIRouter, Depends, HTTPException, status, Query, BackgroundTasks
from fastapi.responses import FileResponse, StreamingResponse
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select
from pydantic import BaseModel, Field
from datetime import datetime

from app.core.database import get_db
from app.core.search_service import AdvancedSearchService
from app.core.export_service import ExportService
from app.dependencies.auth import get_current_user, require_permission
from app.models.user import User
from app.models.student import Student
from app.models.report_template import ReportTemplate, GeneratedReport

router = APIRouter(prefix="/reports", tags=["Reports & Export"])


# ─────── Request/Response Models ─────────────────────────────────────────


class ReportTemplateCreate(BaseModel):
    """Create report template request."""
    name: str = Field(..., min_length=1, max_length=255)
    description: Optional[str] = Field(None, max_length=500)
    fields: List[str] = Field(..., min_items=1, description="List of student fields to include")
    export_format: str = Field(default="csv", regex="^(csv|excel|pdf)$")
    filters: Dict[str, Any] = Field(default_factory=dict, description="Search filters")
    group_by: Optional[str] = Field(None, description="Field to group by")
    include_summary: bool = Field(default=False)
    sort_by: str = Field(default="first_name")
    sort_order: str = Field(default="asc", regex="^(asc|desc)$")
    is_default: bool = Field(default=False)


class ReportTemplateUpdate(BaseModel):
    """Update report template request."""
    name: Optional[str] = Field(None, min_length=1, max_length=255)
    description: Optional[str] = Field(None, max_length=500)
    fields: Optional[List[str]] = None
    export_format: Optional[str] = Field(None, regex="^(csv|excel|pdf)$")
    filters: Optional[Dict[str, Any]] = None
    group_by: Optional[str] = None
    include_summary: Optional[bool] = None
    sort_by: Optional[str] = None
    sort_order: Optional[str] = Field(None, regex="^(asc|desc)$")
    is_default: Optional[bool] = None


class GenerateReportRequest(BaseModel):
    """Generate report request."""
    template_id: str = Field(...)
    title: Optional[str] = Field(None, description="Custom report title")
    filename: Optional[str] = Field(None, description="Custom filename")


class ReportTemplateResponse(BaseModel):
    """Report template response."""
    id: str
    name: str
    description: Optional[str]
    fields: List[str]
    export_format: str
    filters: Dict[str, Any]
    group_by: Optional[str]
    include_summary: bool
    sort_by: str
    sort_order: str
    is_default: bool
    created_at: str
    updated_at: str

    class Config:
        from_attributes = True


class GeneratedReportResponse(BaseModel):
    """Generated report response."""
    id: str
    template_id: str
    title: str
    export_format: str
    status: str
    total_records: int
    file_size: Optional[int]
    created_at: str
    completed_at: Optional[str]
    error_message: Optional[str]

    class Config:
        from_attributes = True


# ─────── Quick Export Endpoints ──────────────────────────────────────────


@router.post("/export/quick")
async def quick_export(
    export_format: str = Query(..., regex="^(csv|excel|pdf)$"),
    fields: List[str] = Query(...),
    title: str = Query("Student Export"),
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
    _: None = Depends(require_permission("students:read")),
):
    """
    Quick export all students without saving template.
    
    Example:
    ```
    POST /api/v1/reports/export/quick?export_format=excel&fields=first_name&fields=email&title=All%20Students
    ```
    """
    if not fields:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="At least one field is required"
        )
    
    # Get all students
    result = await db.execute(select(Student).order_by(Student.first_name))
    students = result.scalars().all()
    
    # Export
    try:
        file_bytes, filename = ExportService.export_students(
            students,
            fields,
            export_format=export_format,
            title=title,
        )
        
        return StreamingResponse(
            iter([file_bytes.getvalue()]),
            media_type="application/octet-stream",
            headers={"Content-Disposition": f"attachment; filename={filename}"}
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Export failed: {str(e)}"
        )


# ─────── Report Template Endpoints ───────────────────────────────────────


@router.post("/templates", response_model=ReportTemplateResponse, status_code=status.HTTP_201_CREATED)
async def create_report_template(
    request: ReportTemplateCreate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Create a new report template."""
    template = ReportTemplate(
        user_id=current_user.id,
        name=request.name,
        description=request.description,
        fields=request.fields,
        export_format=request.export_format,
        filters=request.filters,
        group_by=request.group_by,
        include_summary=request.include_summary,
        sort_by=request.sort_by,
        sort_order=request.sort_order,
        is_default=request.is_default,
    )
    
    db.add(template)
    await db.flush()
    await db.commit()
    
    return ReportTemplateResponse(
        id=str(template.id),
        name=template.name,
        description=template.description,
        fields=template.fields,
        export_format=template.export_format,
        filters=template.filters,
        group_by=template.group_by,
        include_summary=template.include_summary,
        sort_by=template.sort_by,
        sort_order=template.sort_order,
        is_default=template.is_default,
        created_at=template.created_at.isoformat(),
        updated_at=template.updated_at.isoformat(),
    )


@router.get("/templates", response_model=List[ReportTemplateResponse])
async def list_report_templates(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """List all report templates for the current user."""
    query = (
        select(ReportTemplate)
        .where(ReportTemplate.user_id == current_user.id)
        .order_by(ReportTemplate.is_default.desc(), ReportTemplate.created_at.desc())
    )
    result = await db.execute(query)
    templates = result.scalars().all()
    
    return [
        ReportTemplateResponse(
            id=str(t.id),
            name=t.name,
            description=t.description,
            fields=t.fields,
            export_format=t.export_format,
            filters=t.filters,
            group_by=t.group_by,
            include_summary=t.include_summary,
            sort_by=t.sort_by,
            sort_order=t.sort_order,
            is_default=t.is_default,
            created_at=t.created_at.isoformat(),
            updated_at=t.updated_at.isoformat(),
        )
        for t in templates
    ]


@router.get("/templates/{template_id}", response_model=ReportTemplateResponse)
async def get_report_template(
    template_id: str,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Get a specific report template."""
    query = (
        select(ReportTemplate)
        .where(
            ReportTemplate.id == template_id,
            ReportTemplate.user_id == current_user.id,
        )
    )
    result = await db.execute(query)
    template = result.scalar_one_or_none()
    
    if not template:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Report template not found"
        )
    
    return ReportTemplateResponse(
        id=str(template.id),
        name=template.name,
        description=template.description,
        fields=template.fields,
        export_format=template.export_format,
        filters=template.filters,
        group_by=template.group_by,
        include_summary=template.include_summary,
        sort_by=template.sort_by,
        sort_order=template.sort_order,
        is_default=template.is_default,
        created_at=template.created_at.isoformat(),
        updated_at=template.updated_at.isoformat(),
    )


@router.put("/templates/{template_id}", response_model=ReportTemplateResponse)
async def update_report_template(
    template_id: str,
    request: ReportTemplateUpdate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Update a report template."""
    query = (
        select(ReportTemplate)
        .where(
            ReportTemplate.id == template_id,
            ReportTemplate.user_id == current_user.id,
        )
    )
    result = await db.execute(query)
    template = result.scalar_one_or_none()
    
    if not template:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Report template not found"
        )
    
    # Update fields
    if request.name is not None:
        template.name = request.name
    if request.description is not None:
        template.description = request.description
    if request.fields is not None:
        template.fields = request.fields
    if request.export_format is not None:
        template.export_format = request.export_format
    if request.filters is not None:
        template.filters = request.filters
    if request.group_by is not None:
        template.group_by = request.group_by
    if request.include_summary is not None:
        template.include_summary = request.include_summary
    if request.sort_by is not None:
        template.sort_by = request.sort_by
    if request.sort_order is not None:
        template.sort_order = request.sort_order
    if request.is_default is not None:
        template.is_default = request.is_default
    
    template.updated_at = datetime.utcnow()
    await db.commit()
    
    return ReportTemplateResponse(
        id=str(template.id),
        name=template.name,
        description=template.description,
        fields=template.fields,
        export_format=template.export_format,
        filters=template.filters,
        group_by=template.group_by,
        include_summary=template.include_summary,
        sort_by=template.sort_by,
        sort_order=template.sort_order,
        is_default=template.is_default,
        created_at=template.created_at.isoformat(),
        updated_at=template.updated_at.isoformat(),
    )


@router.delete("/templates/{template_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_report_template(
    template_id: str,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Delete a report template."""
    query = (
        select(ReportTemplate)
        .where(
            ReportTemplate.id == template_id,
            ReportTemplate.user_id == current_user.id,
        )
    )
    result = await db.execute(query)
    template = result.scalar_one_or_none()
    
    if not template:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Report template not found"
        )
    
    await db.delete(template)
    await db.commit()


# ─────── Report Generation Endpoints ─────────────────────────────────────


@router.post("/templates/{template_id}/generate")
async def generate_report(
    template_id: str,
    request: GenerateReportRequest = None,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
    _: None = Depends(require_permission("students:read")),
):
    """
    Generate report from template.
    Returns the file as a download or error message.
    """
    if not request:
        request = GenerateReportRequest(template_id=template_id)
    
    # Get template
    query = (
        select(ReportTemplate)
        .where(
            ReportTemplate.id == template_id,
            ReportTemplate.user_id == current_user.id,
        )
    )
    result = await db.execute(query)
    template = result.scalar_one_or_none()
    
    if not template:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Report template not found"
        )
    
    # Search for students using template filters
    search_result = await AdvancedSearchService.search_students(
        db,
        filters=template.filters,
        sort_by=template.sort_by,
        sort_order=template.sort_order,
        page=1,
        page_size=100000,  # Get all
    )
    
    students = search_result["students"]
    
    # Generate export
    try:
        file_bytes, filename = ExportService.export_students(
            students,
            template.fields,
            export_format=template.export_format,
            title=request.title or f"{template.name} - {datetime.now().strftime('%Y-%m-%d')}",
            group_by=template.group_by,
            filename=request.filename,
        )
        
        return StreamingResponse(
            iter([file_bytes.getvalue()]),
            media_type="application/octet-stream",
            headers={"Content-Disposition": f"attachment; filename={filename}"}
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Report generation failed: {str(e)}"
        )


# ─────── Available Fields Endpoint ───────────────────────────────────────


@router.get("/available-fields")
async def get_available_fields(
    current_user: User = Depends(get_current_user),
    _: None = Depends(require_permission("students:read")),
):
    """Get list of available fields for reports."""
    from app.core.export_service import ExportService
    
    fields = [
        {"name": key, "label": value}
        for key, value in ExportService.FIELD_NAMES.items()
    ]
    
    return {
        "fields": fields,
        "total": len(fields),
    }
