"""Advanced search and filtering service."""
from typing import Optional, List, Dict, Any
from datetime import datetime
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, and_, or_, func, desc, asc, text
from sqlalchemy.orm import selectinload
import re

from app.models.student import Student
from app.models.saved_search import SavedSearch
from app.core.cache_helpers import get_cache_key, cache_get, cache_set


class SearchFilter:
    """Helper class for building search filters."""
    
    def __init__(self):
        self.filters = []
    
    def add_text_search(self, query: str):
        """Add full-text search on name, email, admission_no, roll_no."""
        if not query or not query.strip():
            return self
        
        search_term = f"%{query.strip()}%"
        self.filters.append(
            or_(
                Student.first_name.ilike(search_term),
                Student.last_name.ilike(search_term),
                Student.email.ilike(search_term),
                Student.admission_no.ilike(search_term),
                Student.roll_no.ilike(search_term),
            )
        )
        return self
    
    def add_filter(self, field_name: str, value: Any, operator: str = "eq"):
        """Add a filter by field name."""
        if value is None:
            return self
        
        field = getattr(Student, field_name, None)
        if field is None:
            return self
        
        if operator == "eq":
            self.filters.append(field == value)
        elif operator == "in":
            self.filters.append(field.in_(value if isinstance(value, list) else [value]))
        elif operator == "like":
            self.filters.append(field.ilike(f"%{value}%"))
        elif operator == "gte":
            self.filters.append(field >= value)
        elif operator == "lte":
            self.filters.append(field <= value)
        elif operator == "gt":
            self.filters.append(field > value)
        elif operator == "lt":
            self.filters.append(field < value)
        
        return self
    
    def add_date_range(self, field_name: str, start_date: Optional[datetime], end_date: Optional[datetime]):
        """Add date range filter."""
        field = getattr(Student, field_name, None)
        if field is None:
            return self
        
        if start_date:
            self.filters.append(field >= start_date)
        if end_date:
            self.filters.append(field <= end_date)
        
        return self
    
    def add_active_status(self, is_active: Optional[bool]):
        """Filter by active status."""
        if is_active is not None:
            self.filters.append(Student.is_active == is_active)
        return self
    
    def build(self):
        """Build the combined filter."""
        if not self.filters:
            return None
        return and_(*self.filters) if len(self.filters) > 1 else self.filters[0]


class AdvancedSearchService:
    """Service for advanced student search and filtering."""
    
    @staticmethod
    async def search_students(
        db: AsyncSession,
        query: Optional[str] = None,
        filters: Optional[Dict[str, Any]] = None,
        sort_by: str = "first_name",
        sort_order: str = "asc",
        page: int = 1,
        page_size: int = 20,
        use_cache: bool = True,
    ) -> Dict[str, Any]:
        """
        Search students with advanced filtering and sorting.
        
        Args:
            db: Database session
            query: Full-text search query
            filters: Dictionary of filters {field: value}
            sort_by: Field to sort by
            sort_order: "asc" or "desc"
            page: Page number (1-indexed)
            page_size: Results per page
            use_cache: Whether to use cache for count
        
        Returns:
            Dictionary with students, total count, and metadata
        """
        # Build search filter
        search_filter = SearchFilter()
        
        if query:
            search_filter.add_text_search(query)
        
        if filters:
            # Handle common filters
            if "class_name" in filters and filters["class_name"]:
                search_filter.add_filter("class_name", filters["class_name"])
            
            if "city" in filters and filters["city"]:
                search_filter.add_filter("city", filters["city"])
            
            if "state" in filters and filters["state"]:
                search_filter.add_filter("state", filters["state"])
            
            if "is_active" in filters and filters["is_active"] is not None:
                search_filter.add_active_status(filters["is_active"])
            
            if "enrollment_start" in filters and "enrollment_end" in filters:
                search_filter.add_date_range(
                    "enrollment_date",
                    filters.get("enrollment_start"),
                    filters.get("enrollment_end")
                )
            
            if "dob_start" in filters and "dob_end" in filters:
                search_filter.add_date_range(
                    "date_of_birth",
                    filters.get("dob_start"),
                    filters.get("dob_end")
                )
        
        filter_clause = search_filter.build()
        
        # Build query
        base_query = select(Student)
        if filter_clause is not None:
            base_query = base_query.where(filter_clause)
        
        # Add sorting
        sort_field = getattr(Student, sort_by, Student.first_name)
        if sort_order.lower() == "desc":
            base_query = base_query.order_by(desc(sort_field))
        else:
            base_query = base_query.order_by(asc(sort_field))
        
        # Get total count
        count_query = select(func.count()).select_from(Student)
        if filter_clause is not None:
            count_query = count_query.where(filter_clause)
        
        total = await db.scalar(count_query)
        
        # Paginate
        offset = (page - 1) * page_size
        paginated_query = base_query.offset(offset).limit(page_size)
        
        result = await db.execute(paginated_query)
        students = result.scalars().all()
        
        total_pages = (total + page_size - 1) // page_size
        
        return {
            "students": students,
            "total": total,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
            "has_next": page < total_pages,
            "has_previous": page > 1,
        }
    
    @staticmethod
    async def get_search_suggestions(
        db: AsyncSession,
        field: str,
        query: str,
        limit: int = 10,
    ) -> List[str]:
        """
        Get autocomplete suggestions for a field.
        
        Args:
            db: Database session
            field: Field name (city, class_name, etc.)
            query: Search query
            limit: Max suggestions
        
        Returns:
            List of unique suggestions
        """
        if not query:
            return []
        
        # Cache key for suggestions
        cache_key = get_cache_key(f"search_suggestions:{field}:{query}", namespace="search")
        
        # Try cache
        cached = await cache_get(cache_key)
        if cached:
            return cached
        
        # Get field
        field_obj = getattr(Student, field, None)
        if field_obj is None:
            return []
        
        # Query
        search_term = f"{query}%"
        suggestion_query = (
            select(field_obj)
            .distinct()
            .where(field_obj.ilike(search_term))
            .order_by(field_obj)
            .limit(limit)
        )
        
        result = await db.execute(suggestion_query)
        suggestions = [row[0] for row in result.all() if row[0]]
        
        # Cache for 1 hour
        await cache_set(cache_key, suggestions, ttl=3600)
        
        return suggestions
    
    @staticmethod
    async def create_saved_search(
        db: AsyncSession,
        user_id: str,
        name: str,
        filters: Dict[str, Any],
        sort_by: str = "first_name",
        sort_order: str = "asc",
        description: Optional[str] = None,
        is_default: bool = False,
    ) -> SavedSearch:
        """Create a saved search."""
        saved_search = SavedSearch(
            user_id=user_id,
            name=name,
            description=description,
            filters=filters,
            sort_by=sort_by,
            sort_order=sort_order,
            is_default=is_default,
        )
        db.add(saved_search)
        await db.flush()
        return saved_search
    
    @staticmethod
    async def get_user_saved_searches(
        db: AsyncSession,
        user_id: str,
    ) -> List[SavedSearch]:
        """Get all saved searches for a user."""
        query = (
            select(SavedSearch)
            .where(SavedSearch.user_id == user_id)
            .order_by(SavedSearch.is_default.desc(), SavedSearch.created_at.desc())
        )
        result = await db.execute(query)
        return result.scalars().all()
    
    @staticmethod
    async def get_saved_search(
        db: AsyncSession,
        search_id: str,
        user_id: str,
    ) -> Optional[SavedSearch]:
        """Get a specific saved search."""
        query = (
            select(SavedSearch)
            .where(
                and_(
                    SavedSearch.id == search_id,
                    SavedSearch.user_id == user_id,
                )
            )
        )
        result = await db.execute(query)
        return result.scalar_one_or_none()
    
    @staticmethod
    async def update_saved_search(
        db: AsyncSession,
        search_id: str,
        user_id: str,
        name: Optional[str] = None,
        filters: Optional[Dict[str, Any]] = None,
        sort_by: Optional[str] = None,
        sort_order: Optional[str] = None,
        description: Optional[str] = None,
        is_default: Optional[bool] = None,
    ) -> Optional[SavedSearch]:
        """Update a saved search."""
        saved_search = await AdvancedSearchService.get_saved_search(db, search_id, user_id)
        if not saved_search:
            return None
        
        if name is not None:
            saved_search.name = name
        if filters is not None:
            saved_search.filters = filters
        if sort_by is not None:
            saved_search.sort_by = sort_by
        if sort_order is not None:
            saved_search.sort_order = sort_order
        if description is not None:
            saved_search.description = description
        if is_default is not None:
            saved_search.is_default = is_default
        
        saved_search.updated_at = datetime.utcnow()
        await db.flush()
        return saved_search
    
    @staticmethod
    async def delete_saved_search(
        db: AsyncSession,
        search_id: str,
        user_id: str,
    ) -> bool:
        """Delete a saved search."""
        saved_search = await AdvancedSearchService.get_saved_search(db, search_id, user_id)
        if not saved_search:
            return False
        
        await db.delete(saved_search)
        await db.flush()
        return True
