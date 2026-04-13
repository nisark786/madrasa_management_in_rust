"""Saved Search model for advanced filtering."""
from datetime import datetime
from sqlalchemy import Column, String, Integer, DateTime, JSON, ForeignKey, Index, Boolean
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import UUID
import uuid

from app.core.database import Base


class SavedSearch(Base):
    """User's saved search filters."""
    __tablename__ = "saved_searches"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    user_id = Column(UUID(as_uuid=True), ForeignKey("users.id"), nullable=False, index=True)
    name = Column(String(255), nullable=False)
    description = Column(String(500))
    
    # Store filter criteria as JSON
    filters = Column(JSON, nullable=False, default={})
    sort_by = Column(String(50), default="first_name")
    sort_order = Column(String(10), default="asc")  # asc or desc
    
    # Metadata
    is_default = Column(Boolean, default=False, index=True)
    is_shared = Column(Boolean, default=False)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow, nullable=False, index=True)
    updated_at = Column(DateTime(timezone=True), default=datetime.utcnow, onupdate=datetime.utcnow, nullable=False)

    # Relationships
    user = relationship("User", back_populates="saved_searches")

    __table_args__ = (
        Index("idx_user_created", "user_id", "created_at"),
        Index("idx_user_default", "user_id", "is_default"),
    )

    def __repr__(self):
        return f"<SavedSearch(id={self.id}, name={self.name}, user_id={self.user_id})>"
