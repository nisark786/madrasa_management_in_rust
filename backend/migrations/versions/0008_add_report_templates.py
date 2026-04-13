"""Add ReportTemplate and GeneratedReport models

Revision ID: 0008_add_report_templates
Revises: 0007_add_google_drive_to_backups
Create Date: 2026-04-13 00:00:00.000000

"""
from alembic import op
import sqlalchemy as sa
from sqlalchemy.dialects import postgresql

# revision identifiers, used by Alembic.
revision = '0008_add_report_templates'
down_revision = '0007_add_google_drive_to_backups'
branch_labels = None
depends_on = None


def upgrade() -> None:
    # Create report_templates table
    op.create_table(
        'report_templates',
        sa.Column('id', sa.String(36), nullable=False),
        sa.Column('user_id', sa.String(36), nullable=False),
        sa.Column('name', sa.String(255), nullable=False),
        sa.Column('description', sa.Text(), nullable=True),
        sa.Column('fields', postgresql.JSONB(), nullable=False),
        sa.Column('format', sa.String(50), nullable=False),
        sa.Column('filters', postgresql.JSONB(), nullable=True),
        sa.Column('sort_field', sa.String(100), nullable=True),
        sa.Column('sort_order', sa.String(10), nullable=False, server_default='asc'),
        sa.Column('group_by', sa.String(100), nullable=True),
        sa.Column('is_default', sa.Boolean(), nullable=False, server_default='false'),
        sa.Column('created_at', sa.DateTime(timezone=True), nullable=False, server_default=sa.func.now()),
        sa.Column('updated_at', sa.DateTime(timezone=True), nullable=False, server_default=sa.func.now()),
        sa.ForeignKeyConstraint(['user_id'], ['users.id'], ondelete='CASCADE'),
        sa.PrimaryKeyConstraint('id'),
        sa.Index('ix_report_templates_user_id', 'user_id'),
    )
    
    # Create generated_reports table
    op.create_table(
        'generated_reports',
        sa.Column('id', sa.String(36), nullable=False),
        sa.Column('user_id', sa.String(36), nullable=False),
        sa.Column('template_id', sa.String(36), nullable=True),
        sa.Column('report_name', sa.String(255), nullable=False),
        sa.Column('format', sa.String(50), nullable=False),
        sa.Column('status', sa.String(50), nullable=False, server_default='pending'),
        sa.Column('file_path', sa.String(500), nullable=True),
        sa.Column('file_size', sa.Integer(), nullable=True),
        sa.Column('total_records', sa.Integer(), nullable=False, server_default='0'),
        sa.Column('generated_at', sa.DateTime(timezone=True), nullable=True),
        sa.Column('expires_at', sa.DateTime(timezone=True), nullable=True),
        sa.Column('error_message', sa.Text(), nullable=True),
        sa.Column('created_at', sa.DateTime(timezone=True), nullable=False, server_default=sa.func.now()),
        sa.ForeignKeyConstraint(['user_id'], ['users.id'], ondelete='CASCADE'),
        sa.ForeignKeyConstraint(['template_id'], ['report_templates.id'], ondelete='SET NULL'),
        sa.PrimaryKeyConstraint('id'),
        sa.Index('ix_generated_reports_user_id', 'user_id'),
        sa.Index('ix_generated_reports_template_id', 'template_id'),
        sa.Index('ix_generated_reports_status', 'status'),
    )


def downgrade() -> None:
    op.drop_index('ix_generated_reports_status', table_name='generated_reports')
    op.drop_index('ix_generated_reports_template_id', table_name='generated_reports')
    op.drop_index('ix_generated_reports_user_id', table_name='generated_reports')
    op.drop_table('generated_reports')
    op.drop_index('ix_report_templates_user_id', table_name='report_templates')
    op.drop_table('report_templates')
