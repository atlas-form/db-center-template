use db_core::PaginationParams;
pub use repo::table::{app_permissions::PermissionKind, app_users::AppUserStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ListAppUsersRequest {
    pub pagination: PaginationParams,
    pub keyword: Option<String>,
    pub status: Option<AppUserStatus>,
    pub created_at_from: Option<String>,
    pub created_at_to: Option<String>,
    pub updated_at_from: Option<String>,
    pub updated_at_to: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAppUserRequest {
    pub user_id: String,
    pub remark: Option<String>,
    pub status: AppUserStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppUserResponse {
    pub user_id: String,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AppUserStatus,
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
pub struct UpdateRolePermissionsRequest {
    pub role_id: i64,
    pub permission_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserRolesRequest {
    pub user_id: String,
    pub role_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserRoleOptionResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub checked: bool,
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
