pub use repo::table::{admin_users::AdminUserStatus, permissions::PermissionKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAdminUserRequest {
    pub user_id: String,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAdminUserRequest {
    pub user_id: String,
    pub remark: Option<String>,
    pub status: AdminUserStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminUserResponse {
    pub user_id: String,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AdminUserStatus,
    pub roles: Vec<RoleResponse>,
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
pub struct CreateMenuRequest {
    pub name: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
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
pub struct UpdateRolePermissionsRequest {
    pub role_id: i64,
    pub permission_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionTreeNode {
    pub id: i64,
    pub name: String,
    pub kind: PermissionKind,
    pub children: Vec<PermissionTreeNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionTreeNode {
    pub id: i64,
    pub name: String,
    pub kind: PermissionKind,
    pub checked: bool,
    pub children: Vec<RolePermissionTreeNode>,
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
    pub children: Vec<MenuTreeNode>,
}
