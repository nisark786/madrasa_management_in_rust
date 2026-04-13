"""Report Template model for custom student data exports."""
from datetime import datetime
from sqlalchemy import Column, String, Integer, DateTime, JSON, ForeignKey, Index, Boolean
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import UUID, ARRAY
import uuid

from app.core.database import Base


class ReportTemplate(Base):
    """User's custom report templates."""
    __tablename__ = "report_templates"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    user_id = Column(UUID(as_uuid=True), ForeignKey("users.id"), nullable=False, index=True)
    name = Column(String(255), nullable=False)
    description = Column(String(500))
    
    # Report configuration
    # List of fields to include: ['first_name', 'last_name', 'email', 'class_name', ...]
    fields = Column(ARRAY(String), nullable=False, default=list)
    
    # Export format: 'pdf', 'excel', 'csv'
    export_format = Column(String(10), default="pdf")
    
    # Filters to apply before export (JSON)
    filters = Column(JSON, nullable=False, default={})
    
    # Grouping field (e.g., 'class_name', 'city', None)
    group_by = Column(String(50), nullable=True)
    
    # Include summary/statistics
    include_summary = Column(Boolean, default=False)
    
    # Sorting configuration
    sort_by = Column(String(50), default="first_name")
    sort_order = Column(String(10), default="asc")  # asc or desc
    
    # Is system default template
    is_default = Column(Boolean, default=False, index=True)
    
    # Metadata
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow, nullable=False, index=True)
    updated_at = Column(DateTime(timezone=True), default=datetime.utcnow, onupdate=datetime.utcnow, nullable=False)

    # Relationships
    user = relationship("User", back_populates="report_templates")
    generated_reports = relationship("GeneratedReport", back_populates="template", cascade="all, delete-orphan")

    __table_args__ = (
        Index("idx_user_created", "user_id", "created_at"),
        Index("idx_user_default", "user_id", "is_default"),
    )

    def __repr__(self):
        return f"<ReportTemplate(id={self.id}, name={self.name}, format={self.export_format})>"


class GeneratedReport(Base):
    """Record of generated reports."""
    __tablename__ = "generated_reports"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    template_id = Column(UUID(as_uuid=True), ForeignKey("report_templates.id"), nullable=False, index=True)
    user_id = Column(UUID(as_uuid=True), ForeignKey("users.id"), nullable=False, index=True)
    
    # Report metadata
    title = Column(String(255), nullable=False)
    export_format = Column(String(10), nullable=False)  # 'pdf', 'excel', 'csv'
    
    # File path or storage location
    file_path = Column(String(500), nullable=True)
    file_size = Column(Integer, nullable=True)  # In bytes
    
    # Report statistics
    total_records = Column(Integer, default=0)
    
    # Generation status: 'pending', 'completed', 'failed'
    status = Column(String(20), default="pending", index=True)
    error_message = Column(String(500), nullable=True)
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow, nullable=False, index=True)
    completed_at = Column(DateTime(timezone=True), nullable=True)
    expires_at = Column(DateTime(timezone=True), nullable=True)  # For auto-cleanup

    # Relationships
    template = relationship("ReportTemplate", back_populates="generated_reports")
    user = relationship("User", back_populates="generated_reports")

    __table_args__ = (
        Index("idx_template_created", "template_id", "created_at"),
        Index("idx_user_created", "user_id", "created_at"),
        Index("idx_status", "status"),
    )

    def __repr__(self):
        return f"<GeneratedReport(id={self.id}, status={self.status}, format={self.export_format})>"
