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
pub struct CreateMenuRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct MenuResponse {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
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
pub struct UpdateRolePermissionsRequest {
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

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionKind {
    Group,
    Action,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuTreeNode {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub children: Vec<MenuTreeNode>,
}
