use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: i64,
    pub permission_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRolePermission {
    pub role_id: i64,
    pub permission_code: String,
}
