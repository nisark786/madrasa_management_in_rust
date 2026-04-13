"""
Compatibility layer for old dependency imports.
This module re-exports dependencies from their new locations for backward compatibility.
"""

from app.core.database import get_db
from app.dependencies.auth import get_current_user, get_user_permissions

__all__ = [
    'get_db',
    'get_current_user',
    'get_user_permissions',
]
