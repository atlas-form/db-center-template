use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: i64,
    pub permission_id: i64,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRolePermission {
    pub role_id: i64,
    pub permission_id: i64,
}
