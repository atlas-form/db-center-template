pub use repo::table::admin_users::AdminUserStatus;
pub use repo::table::permissions::PermissionKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAdminUserRequest {
    pub user_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AdminUserStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminUserResponse {
    pub user_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AdminUserStatus,
}

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
    pub name: String,
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: PermissionKind,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: PermissionKind,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMenuRequest {
    pub name: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
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
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
    pub children: Vec<MenuTreeNode>,
}
