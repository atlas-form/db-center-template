use axum::{Extension, Json, extract::Path};
use service::api::admin::AdminApi;
use toolcraft_axum_kit::{
    CommonResponse, IntoCommonResponse, ResponseResult, middleware::auth_mw::AuthUser,
};
use validator::Validate;

use crate::{
    dto::admin::*,
    error::{Error, from_biz_error},
    statics::db_manager::get_default_ctx,
};

fn map_admin_user_status(status: AdminUserStatus) -> service::dto::admin::AdminUserStatus {
    match status {
        AdminUserStatus::Enabled => service::dto::admin::AdminUserStatus::Enabled,
        AdminUserStatus::Disabled => service::dto::admin::AdminUserStatus::Disabled,
    }
}

fn map_permission_kind(kind: PermissionKind) -> service::dto::admin::PermissionKind {
    match kind {
        PermissionKind::Group => service::dto::admin::PermissionKind::Group,
        PermissionKind::Action => service::dto::admin::PermissionKind::Action,
    }
}

#[utoipa::path(
    post,
    path = "/admin-users",
    tag = "admin",
    security(("Bearer" = [])),
    request_body = CreateAdminUserRequest,
    responses(
        (status = 200, description = "Create admin user", body = CommonResponse<AdminUserResponse>),
        (status = 400, description = "Validation or business error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_admin_user(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateAdminUserRequest>,
) -> ResponseResult<AdminUserResponse> {
    req.validate().map_err(Error::from)?;
    let api = AdminApi::new(get_default_ctx());
    let admin_user = api
        .create_admin_user(
            auth_user.user_id,
            service::dto::admin::CreateAdminUserRequest {
                user_id: req.user_id,
                status: map_admin_user_status(req.status),
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(AdminUserResponse {
        user_id: admin_user.user_id,
        status: match admin_user.status {
            service::dto::admin::AdminUserStatus::Enabled => AdminUserStatus::Enabled,
            service::dto::admin::AdminUserStatus::Disabled => AdminUserStatus::Disabled,
        },
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/admin-users",
    tag = "admin",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "List admin users", body = CommonResponse<Vec<AdminUserResponse>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_admin_users(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<Vec<AdminUserResponse>> {
    let api = AdminApi::new(get_default_ctx());
    let admin_users = api
        .list_admin_users(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(admin_users
        .into_iter()
        .map(|admin_user| AdminUserResponse {
            user_id: admin_user.user_id,
            status: match admin_user.status {
                service::dto::admin::AdminUserStatus::Enabled => AdminUserStatus::Enabled,
                service::dto::admin::AdminUserStatus::Disabled => AdminUserStatus::Disabled,
            },
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

#[utoipa::path(
    post,
    path = "/roles",
    tag = "admin",
    security(("Bearer" = [])),
    request_body = CreateRoleRequest,
    responses(
        (status = 200, description = "Create role", body = CommonResponse<RoleResponse>),
        (status = 400, description = "Validation or business error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_role(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateRoleRequest>,
) -> ResponseResult<RoleResponse> {
    req.validate().map_err(Error::from)?;
    let api = AdminApi::new(get_default_ctx());
    let role = api
        .create_role(
            auth_user.user_id,
            service::dto::admin::CreateRoleRequest {
                name: req.name,
                code: req.code,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(RoleResponse {
        id: role.id,
        name: role.name,
        code: role.code,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/roles",
    tag = "admin",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "List roles", body = CommonResponse<Vec<RoleResponse>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_roles(Extension(auth_user): Extension<AuthUser>) -> ResponseResult<Vec<RoleResponse>> {
    let api = AdminApi::new(get_default_ctx());
    let roles = api
        .list_roles(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(roles
        .into_iter()
        .map(|role| RoleResponse {
            id: role.id,
            name: role.name,
            code: role.code,
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

#[utoipa::path(
    post,
    path = "/permissions",
    tag = "admin",
    security(("Bearer" = [])),
    request_body = CreatePermissionRequest,
    responses(
        (status = 200, description = "Create permission", body = CommonResponse<PermissionResponse>),
        (status = 400, description = "Validation or business error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_permission(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreatePermissionRequest>,
) -> ResponseResult<PermissionResponse> {
    req.validate().map_err(Error::from)?;
    let api = AdminApi::new(get_default_ctx());
    let permission = api
        .create_permission(
            auth_user.user_id,
            service::dto::admin::CreatePermissionRequest {
                code: req.code,
                name: req.name,
                parent_code: req.parent_code,
                sort: req.sort,
                kind: map_permission_kind(req.kind),
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(PermissionResponse {
        id: permission.id,
        code: permission.code,
        name: permission.name,
        parent_code: permission.parent_code,
        sort: permission.sort,
        kind: match permission.kind {
            service::dto::admin::PermissionKind::Group => PermissionKind::Group,
            service::dto::admin::PermissionKind::Action => PermissionKind::Action,
        },
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/permissions",
    tag = "admin",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "List permissions", body = CommonResponse<Vec<PermissionResponse>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_permissions(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<Vec<PermissionResponse>> {
    let api = AdminApi::new(get_default_ctx());
    let permissions = api
        .list_permissions(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(permissions
        .into_iter()
        .map(|permission| PermissionResponse {
            id: permission.id,
            code: permission.code,
            name: permission.name,
            parent_code: permission.parent_code,
            sort: permission.sort,
            kind: match permission.kind {
                service::dto::admin::PermissionKind::Group => PermissionKind::Group,
                service::dto::admin::PermissionKind::Action => PermissionKind::Action,
            },
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

#[utoipa::path(
    post,
    path = "/menus",
    tag = "admin",
    security(("Bearer" = [])),
    request_body = CreateMenuRequest,
    responses(
        (status = 200, description = "Create menu", body = CommonResponse<MenuResponse>),
        (status = 400, description = "Validation or business error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_menu(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateMenuRequest>,
) -> ResponseResult<MenuResponse> {
    req.validate().map_err(Error::from)?;
    let api = AdminApi::new(get_default_ctx());
    let menu = api
        .create_menu(
            auth_user.user_id,
            service::dto::admin::CreateMenuRequest {
                name: req.name,
                parent_id: req.parent_id,
                permission_code: req.permission_code,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(MenuResponse {
        id: menu.id,
        name: menu.name,
        parent_id: menu.parent_id,
        permission_code: menu.permission_code,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/menus",
    tag = "admin",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "List menus", body = CommonResponse<Vec<MenuResponse>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_menus(Extension(auth_user): Extension<AuthUser>) -> ResponseResult<Vec<MenuResponse>> {
    let api = AdminApi::new(get_default_ctx());
    let menus = api
        .list_menus(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(menus
        .into_iter()
        .map(|menu| MenuResponse {
            id: menu.id,
            name: menu.name,
            parent_id: menu.parent_id,
            permission_code: menu.permission_code,
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

#[utoipa::path(
    post,
    path = "/user-roles",
    tag = "admin",
    security(("Bearer" = [])),
    request_body = AssignUserRoleRequest,
    responses(
        (status = 200, description = "Assign role to user", body = CommonResponse<UserRoleResponse>),
        (status = 400, description = "Validation or business error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn assign_user_role(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<AssignUserRoleRequest>,
) -> ResponseResult<UserRoleResponse> {
    req.validate().map_err(Error::from)?;
    let api = AdminApi::new(get_default_ctx());
    let user_role = api
        .assign_user_role(
            auth_user.user_id,
            service::dto::admin::AssignUserRoleRequest {
                user_id: req.user_id,
                role_id: req.role_id,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(UserRoleResponse {
        user_id: user_role.user_id,
        role_id: user_role.role_id,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/users/{user_id}/roles",
    tag = "admin",
    security(("Bearer" = [])),
    params(
        ("user_id" = String, Path, description = "Auth user id")
    ),
    responses(
        (status = 200, description = "List roles of a user", body = CommonResponse<Vec<RoleResponse>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_user_roles(
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<String>,
) -> ResponseResult<Vec<RoleResponse>> {
    let api = AdminApi::new(get_default_ctx());
    let roles = api
        .list_user_roles(auth_user.user_id, user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(roles
        .into_iter()
        .map(|role| RoleResponse {
            id: role.id,
            name: role.name,
            code: role.code,
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

#[utoipa::path(
    post,
    path = "/role-permissions",
    tag = "admin",
    security(("Bearer" = [])),
    request_body = GrantRolePermissionRequest,
    responses(
        (status = 200, description = "Grant permission to role", body = CommonResponse<RolePermissionResponse>),
        (status = 400, description = "Validation or business error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn grant_role_permission(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<GrantRolePermissionRequest>,
) -> ResponseResult<RolePermissionResponse> {
    req.validate().map_err(Error::from)?;
    let api = AdminApi::new(get_default_ctx());
    let role_permission = api
        .grant_role_permission(
            auth_user.user_id,
            service::dto::admin::GrantRolePermissionRequest {
                role_id: req.role_id,
                permission_code: req.permission_code,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(RolePermissionResponse {
        role_id: role_permission.role_id,
        permission_code: role_permission.permission_code,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/me/permissions",
    tag = "admin",
    security(("Bearer" = [])),
    responses(
        (
            status = 200,
            description = "Current user permissions",
            body = CommonResponse<CurrentUserPermissionsResponse>
        ),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn current_user_permissions(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<CurrentUserPermissionsResponse> {
    let api = AdminApi::new(get_default_ctx());
    let resp = api
        .get_current_user_permissions(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(CurrentUserPermissionsResponse {
        user_id: resp.user_id,
        role_codes: resp.role_codes,
        permission_codes: resp.permission_codes,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    get,
    path = "/me/menus",
    tag = "admin",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "Current user menu tree", body = CommonResponse<Vec<MenuTreeNode>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn current_user_menus(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<Vec<MenuTreeNode>> {
    let api = AdminApi::new(get_default_ctx());
    let menus = api
        .get_current_user_menus(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(menus
        .into_iter()
        .map(map_menu_tree)
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

fn map_menu_tree(node: service::dto::admin::MenuTreeNode) -> MenuTreeNode {
    MenuTreeNode {
        id: node.id,
        name: node.name,
        parent_id: node.parent_id,
        permission_code: node.permission_code,
        children: node.children.into_iter().map(map_menu_tree).collect(),
    }
}
