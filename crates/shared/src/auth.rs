use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum Role {
    Student,
    Staff,
    Manager,
    PlatformAdmin,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Student => "student",
            Role::Staff => "staff",
            Role::Manager => "manager",
            Role::PlatformAdmin => "platform_admin",
        }
    }
}
