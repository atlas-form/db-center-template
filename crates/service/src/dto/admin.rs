use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePermissionRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionResponse {
    pub id: i64,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMenuRequest {
    pub name: String,
    pub path: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssignUserRoleRequest {
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserRoleResponse {
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrantRolePermissionRequest {
    pub role_id: i64,
    pub permission_code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionResponse {
    pub role_id: i64,
    pub permission_code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrentUserPermissionsResponse {
    pub user_id: String,
    pub role_codes: Vec<String>,
    pub permission_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuTreeNode {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
    pub children: Vec<MenuTreeNode>,
}
