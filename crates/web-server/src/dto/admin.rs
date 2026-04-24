use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminUserStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAdminUserRequest {
    #[validate(length(min = 1, max = 128))]
    pub identifier: String,
    #[validate(length(min = 1, max = 255))]
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAdminUserRequest {
    #[validate(length(min = 1, max = 255))]
    pub remark: Option<String>,
    pub status: AdminUserStatus,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub user_id: String,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AdminUserStatus,
    pub roles: Vec<RoleResponse>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionKind {
    Group,
    Action,
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
pub struct CreatePermissionRequest {
    #[validate(length(min = 1, max = 128))]
    pub code: String,
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(length(min = 1, max = 128))]
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: PermissionKind,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: PermissionKind,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMenuRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub parent_id: Option<i64>,
    #[validate(length(min = 1, max = 128))]
    pub permission_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AssignUserRoleRequest {
    #[validate(length(min = 1, max = 128))]
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Debug, Serialize)]
pub struct UserRoleResponse {
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GrantRolePermissionRequest {
    pub role_id: i64,
    #[validate(length(min = 1, max = 128))]
    pub permission_code: String,
}

#[derive(Debug, Serialize)]
pub struct RolePermissionResponse {
    pub role_id: i64,
    pub permission_code: String,
}

#[derive(Debug, Serialize)]
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
