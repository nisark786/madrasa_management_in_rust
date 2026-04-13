"""Advanced search and filtering API endpoints."""
from typing import Optional, Dict, Any, List
from fastapi import APIRouter, Depends, HTTPException, status, Query
from sqlalchemy.ext.asyncio import AsyncSession
from pydantic import BaseModel, Field

from app.core.database import get_db
from app.core.search_service import AdvancedSearchService
from app.dependencies.auth import get_current_user, require_permission
from app.models.user import User
from app.models.student import Student

router = APIRouter(prefix="/search", tags=["Advanced Search"])


# ─────── Request/Response Models ─────────────────────────────────────────


class SearchRequest(BaseModel):
    """Advanced search request."""
    query: Optional[str] = Field(None, description="Full-text search query")
    filters: Optional[Dict[str, Any]] = Field(
        default_factory=dict,
        description="Filter criteria (class_name, city, state, is_active, etc.)"
    )
    sort_by: str = Field(default="first_name", description="Field to sort by")
    sort_order: str = Field(default="asc", description="Sort order: asc or desc")
    page: int = Field(default=1, ge=1, description="Page number")
    page_size: int = Field(default=20, ge=1, le=100, description="Results per page")


class StudentSearchResult(BaseModel):
    """Single search result."""
    id: str
    first_name: str
    last_name: str
    email: str
    class_name: str
    roll_no: Optional[str]
    admission_no: Optional[str]
    city: Optional[str]
    state: Optional[str]
    is_active: bool


class SearchResponse(BaseModel):
    """Advanced search response."""
    students: List[StudentSearchResult]
    total: int
    page: int
    page_size: int
    total_pages: int
    has_next: bool
    has_previous: bool


class SavedSearchCreate(BaseModel):
    """Create saved search request."""
    name: str = Field(..., min_length=1, max_length=255)
    description: Optional[str] = Field(None, max_length=500)
    filters: Dict[str, Any] = Field(default_factory=dict)
    sort_by: str = Field(default="first_name")
    sort_order: str = Field(default="asc")
    is_default: bool = Field(default=False)


class SavedSearchUpdate(BaseModel):
    """Update saved search request."""
    name: Optional[str] = Field(None, min_length=1, max_length=255)
    description: Optional[str] = Field(None, max_length=500)
    filters: Optional[Dict[str, Any]] = None
    sort_by: Optional[str] = None
    sort_order: Optional[str] = None
    is_default: Optional[bool] = None


class SavedSearchResponse(BaseModel):
    """Saved search response."""
    id: str
    name: str
    description: Optional[str]
    filters: Dict[str, Any]
    sort_by: str
    sort_order: str
    is_default: bool
    is_shared: bool
    created_at: str
    updated_at: str

    class Config:
        from_attributes = True


# ─────── Search Endpoints ────────────────────────────────────────────────


@router.post("/students", response_model=SearchResponse)
async def search_students(
    request: SearchRequest,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
    _: None = Depends(require_permission("students:read")),
):
    """
    Advanced search for students with filtering and sorting.
    
    ### Filters supported:
    - `class_name`: Filter by class
    - `city`: Filter by city
    - `state`: Filter by state
    - `is_active`: Filter by active status (true/false)
    - `enrollment_start`: Enrollment date start
    - `enrollment_end`: Enrollment date end
    - `dob_start`: Date of birth start
    - `dob_end`: Date of birth end
    
    ### Example:
    ```json
    {
        "query": "John Doe",
        "filters": {
            "class_name": "10A",
            "city": "New York",
            "is_active": true
        },
        "sort_by": "first_name",
        "sort_order": "asc",
        "page": 1,
        "page_size": 20
    }
    ```
    """
    result = await AdvancedSearchService.search_students(
        db,
        query=request.query,
        filters=request.filters,
        sort_by=request.sort_by,
        sort_order=request.sort_order,
        page=request.page,
        page_size=request.page_size,
    )
    
    return SearchResponse(
        students=[
            StudentSearchResult(
                id=str(s.id),
                first_name=s.first_name,
                last_name=s.last_name,
                email=s.email,
                class_name=s.class_name,
                roll_no=s.roll_no,
                admission_no=s.admission_no,
                city=s.city,
                state=s.state,
                is_active=s.is_active,
            )
            for s in result["students"]
        ],
        total=result["total"],
        page=result["page"],
        page_size=result["page_size"],
        total_pages=result["total_pages"],
        has_next=result["has_next"],
        has_previous=result["has_previous"],
    )


@router.get("/suggestions/{field}")
async def get_search_suggestions(
    field: str = Query(..., description="Field name: city, class_name, state, etc."),
    query: str = Query(..., min_length=1, description="Search query"),
    limit: int = Query(10, ge=1, le=50),
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
    _: None = Depends(require_permission("students:read")),
):
    """
    Get autocomplete suggestions for a field.
    
    Supported fields: `city`, `class_name`, `state`
    """
    allowed_fields = ["city", "class_name", "state", "roll_no"]
    if field not in allowed_fields:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Field must be one of: {', '.join(allowed_fields)}"
        )
    
    suggestions = await AdvancedSearchService.get_search_suggestions(
        db,
        field=field,
        query=query,
        limit=limit,
    )
    
    return {"suggestions": suggestions}


# ─────── Saved Searches Endpoints ────────────────────────────────────────


@router.post("/saved", response_model=SavedSearchResponse, status_code=status.HTTP_201_CREATED)
async def create_saved_search(
    request: SavedSearchCreate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Create a new saved search."""
    saved_search = await AdvancedSearchService.create_saved_search(
        db,
        user_id=current_user.id,
        name=request.name,
        filters=request.filters,
        sort_by=request.sort_by,
        sort_order=request.sort_order,
        description=request.description,
        is_default=request.is_default,
    )
    
    await db.commit()
    
    return SavedSearchResponse(
        id=str(saved_search.id),
        name=saved_search.name,
        description=saved_search.description,
        filters=saved_search.filters,
        sort_by=saved_search.sort_by,
        sort_order=saved_search.sort_order,
        is_default=saved_search.is_default,
        is_shared=saved_search.is_shared,
        created_at=saved_search.created_at.isoformat(),
        updated_at=saved_search.updated_at.isoformat(),
    )


@router.get("/saved", response_model=List[SavedSearchResponse])
async def list_saved_searches(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """List all saved searches for the current user."""
    saved_searches = await AdvancedSearchService.get_user_saved_searches(
        db,
        user_id=current_user.id,
    )
    
    return [
        SavedSearchResponse(
            id=str(ss.id),
            name=ss.name,
            description=ss.description,
            filters=ss.filters,
            sort_by=ss.sort_by,
            sort_order=ss.sort_order,
            is_default=ss.is_default,
            is_shared=ss.is_shared,
            created_at=ss.created_at.isoformat(),
            updated_at=ss.updated_at.isoformat(),
        )
        for ss in saved_searches
    ]


@router.get("/saved/{search_id}", response_model=SavedSearchResponse)
async def get_saved_search(
    search_id: str,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Get a specific saved search."""
    saved_search = await AdvancedSearchService.get_saved_search(
        db,
        search_id=search_id,
        user_id=current_user.id,
    )
    
    if not saved_search:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Saved search not found"
        )
    
    return SavedSearchResponse(
        id=str(saved_search.id),
        name=saved_search.name,
        description=saved_search.description,
        filters=saved_search.filters,
        sort_by=saved_search.sort_by,
        sort_order=saved_search.sort_order,
        is_default=saved_search.is_default,
        is_shared=saved_search.is_shared,
        created_at=saved_search.created_at.isoformat(),
        updated_at=saved_search.updated_at.isoformat(),
    )


@router.put("/saved/{search_id}", response_model=SavedSearchResponse)
async def update_saved_search(
    search_id: str,
    request: SavedSearchUpdate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Update a saved search."""
    saved_search = await AdvancedSearchService.update_saved_search(
        db,
        search_id=search_id,
        user_id=current_user.id,
        name=request.name,
        filters=request.filters,
        sort_by=request.sort_by,
        sort_order=request.sort_order,
        description=request.description,
        is_default=request.is_default,
    )
    
    if not saved_search:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Saved search not found"
        )
    
    await db.commit()
    
    return SavedSearchResponse(
        id=str(saved_search.id),
        name=saved_search.name,
        description=saved_search.description,
        filters=saved_search.filters,
        sort_by=saved_search.sort_by,
        sort_order=saved_search.sort_order,
        is_default=saved_search.is_default,
        is_shared=saved_search.is_shared,
        created_at=saved_search.created_at.isoformat(),
        updated_at=saved_search.updated_at.isoformat(),
    )


@router.delete("/saved/{search_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_saved_search(
    search_id: str,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Delete a saved search."""
    success = await AdvancedSearchService.delete_saved_search(
        db,
        search_id=search_id,
        user_id=current_user.id,
    )
    
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Saved search not found"
        )
    
    await db.commit()
