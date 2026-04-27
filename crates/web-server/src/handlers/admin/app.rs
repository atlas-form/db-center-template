use axum::{Extension, Json, extract::Path};
use db_core::error::BizError;
use error_code::admin as admin_error;
use service::api::app::AppApi;
use toolcraft_axum_kit::{IntoCommonResponse, ResponseResult, middleware::auth_mw::AuthUser};
use validator::Validate;

use crate::{
    clients::auth_client,
    dto::app::*,
    error::{Error, from_biz_error},
    statics::db_manager::get_app_ctx,
};

fn map_role_response(role: service::dto::app::RoleResponse) -> RoleResponse {
    RoleResponse {
        id: role.id,
        name: role.name,
        code: role.code,
    }
}

fn map_app_user_response(app_user: service::dto::app::AppUserResponse) -> AppUserResponse {
    AppUserResponse {
        user_id: app_user.user_id,
        display_id: app_user.display_id,
        display_name: app_user.display_name,
        remark: app_user.remark,
        status: match app_user.status {
            service::dto::app::AppUserStatus::Enabled => AppUserStatus::Enabled,
            service::dto::app::AppUserStatus::Disabled => AppUserStatus::Disabled,
        },
        roles: app_user.roles.into_iter().map(map_role_response).collect(),
    }
}

fn map_permission_kind(kind: service::dto::app::PermissionKind) -> PermissionKind {
    match kind {
        service::dto::app::PermissionKind::Group => PermissionKind::Group,
        service::dto::app::PermissionKind::Action => PermissionKind::Action,
    }
}

fn map_permission_tree(node: service::dto::app::PermissionTreeNode) -> PermissionTreeNode {
    PermissionTreeNode {
        id: node.id,
        name: node.name,
        kind: map_permission_kind(node.kind),
        children: node.children.into_iter().map(map_permission_tree).collect(),
    }
}

fn map_role_permission_tree(
    node: service::dto::app::RolePermissionTreeNode,
) -> RolePermissionTreeNode {
    RolePermissionTreeNode {
        id: node.id,
        name: node.name,
        kind: map_permission_kind(node.kind),
        checked: node.checked,
        children: node
            .children
            .into_iter()
            .map(map_role_permission_tree)
            .collect(),
    }
}

pub async fn create_app_user(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateAppUserRequest>,
) -> ResponseResult<AppUserResponse> {
    req.validate().map_err(Error::from)?;
    let target_user = auth_client::get_user_by_identifier(&req.identifier)
        .await?
        .ok_or_else(|| {
            from_biz_error(BizError::new(
                admin_error::ADMIN_AUTH_USER_NOT_FOUND,
                format!("auth user not found: {}", req.identifier),
            ))
        })?;
    let api = AppApi::new(get_app_ctx());
    let app_user = api
        .create_app_user(
            auth_user.user_id,
            service::dto::app::CreateAppUserRequest {
                user_id: target_user.id,
                display_id: target_user
                    .display_user_id
                    .unwrap_or_else(|| target_user.username.clone()),
                display_name: target_user
                    .display_name
                    .unwrap_or_else(|| target_user.username.clone()),
                remark: req.remark,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(map_app_user_response(app_user)
        .into_common_response()
        .to_json())
}

pub async fn list_app_users(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<Vec<AppUserResponse>> {
    let api = AppApi::new(get_app_ctx());
    let app_users = api
        .list_app_users(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(app_users
        .into_iter()
        .map(map_app_user_response)
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

pub async fn update_app_user(
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateAppUserRequest>,
) -> ResponseResult<AppUserResponse> {
    req.validate().map_err(Error::from)?;
    let api = AppApi::new(get_app_ctx());
    let app_user = api
        .update_app_user(
            auth_user.user_id,
            service::dto::app::UpdateAppUserRequest {
                user_id,
                remark: req.remark,
                status: match req.status {
                    AppUserStatus::Enabled => service::dto::app::AppUserStatus::Enabled,
                    AppUserStatus::Disabled => service::dto::app::AppUserStatus::Disabled,
                },
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(map_app_user_response(app_user)
        .into_common_response()
        .to_json())
}

pub async fn delete_app_user(
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<String>,
) -> ResponseResult<()> {
    let api = AppApi::new(get_app_ctx());
    api.delete_app_user(auth_user.user_id, user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(().into_common_response().to_json())
}

pub async fn create_role(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateRoleRequest>,
) -> ResponseResult<RoleResponse> {
    req.validate().map_err(Error::from)?;
    let api = AppApi::new(get_app_ctx());
    let role = api
        .create_role(
            auth_user.user_id,
            service::dto::app::CreateRoleRequest {
                name: req.name,
                code: req.code,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(map_role_response(role).into_common_response().to_json())
}

pub async fn list_roles(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<Vec<RoleResponse>> {
    let api = AppApi::new(get_app_ctx());
    let roles = api
        .list_roles(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(roles
        .into_iter()
        .map(map_role_response)
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

pub async fn delete_role(
    Extension(auth_user): Extension<AuthUser>,
    Path(role_id): Path<i64>,
) -> ResponseResult<()> {
    let api = AppApi::new(get_app_ctx());
    api.delete_role(auth_user.user_id, role_id)
        .await
        .map_err(from_biz_error)?;

    Ok(().into_common_response().to_json())
}

pub async fn list_user_roles(
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<String>,
) -> ResponseResult<Vec<UserRoleOptionResponse>> {
    let api = AppApi::new(get_app_ctx());
    let roles = api
        .list_user_roles(auth_user.user_id, user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(roles
        .into_iter()
        .map(|role| UserRoleOptionResponse {
            id: role.id,
            name: role.name,
            code: role.code,
            checked: role.checked,
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

pub async fn update_user_roles(
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateUserRolesRequest>,
) -> ResponseResult<Vec<UserRoleOptionResponse>> {
    req.validate().map_err(Error::from)?;
    let api = AppApi::new(get_app_ctx());
    let roles = api
        .update_user_roles(
            auth_user.user_id,
            service::dto::app::UpdateUserRolesRequest {
                user_id,
                role_ids: req.role_ids,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(roles
        .into_iter()
        .map(|role| UserRoleOptionResponse {
            id: role.id,
            name: role.name,
            code: role.code,
            checked: role.checked,
        })
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

pub async fn list_permissions(
    Extension(auth_user): Extension<AuthUser>,
) -> ResponseResult<Vec<PermissionTreeNode>> {
    let api = AppApi::new(get_app_ctx());
    let tree = api
        .list_permissions(auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(tree
        .into_iter()
        .map(map_permission_tree)
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

pub async fn list_role_permissions(
    Extension(auth_user): Extension<AuthUser>,
    Path(role_id): Path<i64>,
) -> ResponseResult<Vec<RolePermissionTreeNode>> {
    let api = AppApi::new(get_app_ctx());
    let tree = api
        .list_role_permissions(auth_user.user_id, role_id)
        .await
        .map_err(from_biz_error)?;

    Ok(tree
        .into_iter()
        .map(map_role_permission_tree)
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}

pub async fn update_role_permissions(
    Extension(auth_user): Extension<AuthUser>,
    Path(role_id): Path<i64>,
    Json(req): Json<UpdateRolePermissionsRequest>,
) -> ResponseResult<Vec<RolePermissionTreeNode>> {
    req.validate().map_err(Error::from)?;
    let api = AppApi::new(get_app_ctx());
    let tree = api
        .update_role_permissions(
            auth_user.user_id,
            service::dto::app::UpdateRolePermissionsRequest {
                role_id,
                permission_ids: req.permission_ids,
            },
        )
        .await
        .map_err(from_biz_error)?;

    Ok(tree
        .into_iter()
        .map(map_role_permission_tree)
        .collect::<Vec<_>>()
        .into_common_response()
        .to_json())
}
