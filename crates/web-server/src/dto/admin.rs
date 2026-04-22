use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AdminUserStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAdminUserRequest {
    #[validate(length(min = 1, max = 128))]
    pub user_id: String,
    pub status: AdminUserStatus,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserResponse {
    pub user_id: String,
    pub status: AdminUserStatus,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionKind {
    Group,
    Action,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct PermissionResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: PermissionKind,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMenuRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub parent_id: Option<i64>,
    #[validate(length(min = 1, max = 128))]
    pub permission_code: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignUserRoleRequest {
    #[validate(length(min = 1, max = 128))]
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserRoleResponse {
    pub user_id: String,
    pub role_id: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GrantRolePermissionRequest {
    pub role_id: i64,
    #[validate(length(min = 1, max = 128))]
    pub permission_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RolePermissionResponse {
    pub role_id: i64,
    pub permission_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentUserPermissionsResponse {
    pub user_id: String,
    pub role_codes: Vec<String>,
    pub permission_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MenuTreeNode {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub permission_code: Option<String>,
    #[schema(no_recursion)]
    pub children: Vec<MenuTreeNode>,
}
