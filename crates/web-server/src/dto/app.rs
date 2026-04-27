use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AppUserStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListAppUsersQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    #[validate(length(max = 255))]
    pub keyword: Option<String>,
    pub status: Option<AppUserStatus>,
    #[validate(length(max = 64))]
    pub created_at_from: Option<String>,
    #[validate(length(max = 64))]
    pub created_at_to: Option<String>,
    #[validate(length(max = 64))]
    pub updated_at_from: Option<String>,
    #[validate(length(max = 64))]
    pub updated_at_to: Option<String>,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAppUserRequest {
    #[validate(length(min = 1, max = 255))]
    pub remark: Option<String>,
    pub status: AppUserStatus,
}

#[derive(Debug, Serialize)]
pub struct AppUserResponse {
    pub user_id: String,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AppUserStatus,
    pub roles: Vec<RoleResponse>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRolePermissionsRequest {
    pub permission_ids: Vec<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRolesRequest {
    pub role_ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionKind {
    Group,
    Action,
}

#[derive(Debug, Serialize)]
pub struct CurrentUserPermissionsResponse {
    pub user_id: String,
    pub role_codes: Vec<String>,
    pub permission_codes: Vec<String>,
}
