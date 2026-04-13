from fastapi import APIRouter, Depends, HTTPException, status, BackgroundTasks
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, delete, insert
from pydantic import BaseModel
from typing import Optional
from collections import defaultdict

from app.core.database import get_db
from app.core import redis_client as rc
from app.core.audit import log_audit_task
from app.models.role import RoleTemplate, TemplatePermission
from app.models.permission import Permission
from app.dependencies.auth import require_permission

router = APIRouter(prefix="/role-templates", tags=["Role Templates"])

_TEMPLATES_LIST_KEY = "cache:role_templates:list"


class CreateRoleTemplateRequest(BaseModel):
    name: str
    display_name: str
    description: Optional[str] = None
    icon: Optional[str] = None
    color: Optional[str] = None
    permission_ids: list[str] = []


class UpdateRoleTemplateRequest(BaseModel):
    display_name: Optional[str] = None
    description: Optional[str] = None
    icon: Optional[str] = None
    color: Optional[str] = None
    permission_ids: Optional[list[str]] = None
    is_active: Optional[bool] = None


@router.get("/")
async def list_role_templates(
    db: AsyncSession = Depends(get_db),
    _=Depends(require_permission("admin:manage_roles")),
):
    """List all role templates with permissions. Cached in Redis."""
    cached = await rc.get_cached_response(_TEMPLATES_LIST_KEY)
    if cached is not None:
        return cached

    templates_result = await db.execute(select(RoleTemplate).order_by(RoleTemplate.created_at.desc()))
    templates = templates_result.scalars().all()
    if not templates:
        return []

    template_ids = [t.id for t in templates]
    perms_result = await db.execute(
        select(TemplatePermission.template_id, Permission.id, Permission.name, Permission.module, Permission.action)
        .join(Permission, Permission.id == TemplatePermission.permission_id)
        .where(TemplatePermission.template_id.in_(template_ids))
    )
    template_perms: dict[str, list] = defaultdict(list)
    for template_id, perm_id, perm_name, module, action in perms_result.all():
        template_perms[template_id].append({
            "id": perm_id,
            "name": perm_name,
            "module": module,
            "action": action,
        })

    data = [
        {
            "id": t.id,
            "name": t.name,
            "display_name": t.display_name,
            "description": t.description,
            "icon": t.icon,
            "color": t.color,
            "is_system_template": t.is_system_template,
            "is_active": t.is_active,
            "permissions": template_perms.get(t.id, []),
            "permission_count": len(template_perms.get(t.id, [])),
        }
        for t in templates
    ]
    await rc.cache_response(_TEMPLATES_LIST_KEY, data, ttl=300)
    return data


@router.post("/", status_code=status.HTTP_201_CREATED)
async def create_role_template(
    body: CreateRoleTemplateRequest,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
    current_user=Depends(require_permission("admin:manage_roles")),
):
    """Create a new role template."""
    existing = await db.execute(select(RoleTemplate.id).where(RoleTemplate.name == body.name))
    if existing.scalar_one_or_none():
        raise HTTPException(status_code=400, detail="Template name already exists")

    template = RoleTemplate(
        name=body.name,
        display_name=body.display_name,
        description=body.description,
        icon=body.icon,
        color=body.color,
        created_by=current_user.id,
    )
    db.add(template)
    await db.flush()

    if body.permission_ids:
        # Verify all permissions exist
        perms_result = await db.execute(
            select(Permission.id).where(Permission.id.in_(body.permission_ids))
        )
        existing_perms = set(row[0] for row in perms_result.all())
        if len(existing_perms) != len(body.permission_ids):
            await db.rollback()
            raise HTTPException(status_code=400, detail="One or more permissions not found")

        await db.execute(
            insert(TemplatePermission),
            [{"template_id": template.id, "permission_id": pid} for pid in body.permission_ids],
        )

    await db.commit()
    background_tasks.add_task(rc.invalidate_keys, _TEMPLATES_LIST_KEY)
    background_tasks.add_task(log_audit_task, current_user.id, "CREATE_ROLE_TEMPLATE", "role_templates", template.id)
    return {"id": template.id, "name": template.name, "display_name": template.display_name}


@router.get("/{template_id}")
async def get_role_template(
    template_id: str,
    db: AsyncSession = Depends(get_db),
    _=Depends(require_permission("admin:manage_roles")),
):
    """Get a specific role template with its permissions."""
    result = await db.execute(select(RoleTemplate).where(RoleTemplate.id == template_id))
    template = result.scalar_one_or_none()
    if not template:
        raise HTTPException(status_code=404, detail="Template not found")

    perms_result = await db.execute(
        select(Permission.id, Permission.name, Permission.module, Permission.action)
        .join(TemplatePermission, Permission.id == TemplatePermission.permission_id)
        .where(TemplatePermission.template_id == template_id)
    )
    permissions = [
        {
            "id": perm_id,
            "name": perm_name,
            "module": module,
            "action": action,
        }
        for perm_id, perm_name, module, action in perms_result.all()
    ]

    return {
        "id": template.id,
        "name": template.name,
        "display_name": template.display_name,
        "description": template.description,
        "icon": template.icon,
        "color": template.color,
        "is_system_template": template.is_system_template,
        "is_active": template.is_active,
        "permissions": permissions,
    }


@router.patch("/{template_id}")
async def update_role_template(
    template_id: str,
    body: UpdateRoleTemplateRequest,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
    current_user=Depends(require_permission("admin:manage_roles")),
):
    """Update a role template."""
    result = await db.execute(select(RoleTemplate).where(RoleTemplate.id == template_id))
    template = result.scalar_one_or_none()
    if not template:
        raise HTTPException(status_code=404, detail="Template not found")

    if template.is_system_template:
        raise HTTPException(status_code=400, detail="System templates cannot be modified")

    if body.display_name is not None:
        template.display_name = body.display_name
    if body.description is not None:
        template.description = body.description
    if body.icon is not None:
        template.icon = body.icon
    if body.color is not None:
        template.color = body.color
    if body.is_active is not None:
        template.is_active = body.is_active

    if body.permission_ids is not None:
        # Verify all permissions exist
        perms_result = await db.execute(
            select(Permission.id).where(Permission.id.in_(body.permission_ids))
        )
        existing_perms = set(row[0] for row in perms_result.all())
        if len(existing_perms) != len(body.permission_ids):
            await db.rollback()
            raise HTTPException(status_code=400, detail="One or more permissions not found")

        await db.execute(delete(TemplatePermission).where(TemplatePermission.template_id == template_id))
        if body.permission_ids:
            await db.execute(
                insert(TemplatePermission),
                [{"template_id": template_id, "permission_id": pid} for pid in body.permission_ids],
            )

    await db.commit()
    background_tasks.add_task(rc.invalidate_keys, _TEMPLATES_LIST_KEY)
    background_tasks.add_task(log_audit_task, current_user.id, "UPDATE_ROLE_TEMPLATE", "role_templates", template_id)
    return {"message": "Template updated"}


@router.delete("/{template_id}")
async def delete_role_template(
    template_id: str,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
    current_user=Depends(require_permission("admin:manage_roles")),
):
    """Delete a role template."""
    result = await db.execute(
        select(RoleTemplate.id, RoleTemplate.is_system_template).where(RoleTemplate.id == template_id)
    )
    template = result.one_or_none()
    if not template:
        raise HTTPException(status_code=404, detail="Template not found")
    if template.is_system_template:
        raise HTTPException(status_code=400, detail="System templates cannot be deleted")

    await db.execute(delete(RoleTemplate).where(RoleTemplate.id == template_id))
    await db.commit()
    background_tasks.add_task(rc.invalidate_keys, _TEMPLATES_LIST_KEY)
    background_tasks.add_task(log_audit_task, current_user.id, "DELETE_ROLE_TEMPLATE", "role_templates", template_id)
    return {"message": "Template deleted"}
