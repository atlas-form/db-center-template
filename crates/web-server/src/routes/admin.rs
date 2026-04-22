use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use toolcraft_axum_kit::middleware::auth_mw::auth;
use toolcraft_jwt::VerifyJwt;
use utoipa::OpenApi;

use crate::handlers::admin::{
    assign_user_role, create_admin_user, create_menu, create_permission, create_role,
    current_user_menus, current_user_permissions, grant_role_permission, list_admin_users,
    list_menus, list_permissions, list_roles, list_user_roles,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::admin::create_admin_user,
        crate::handlers::admin::list_admin_users,
        crate::handlers::admin::create_role,
        crate::handlers::admin::list_roles,
        crate::handlers::admin::create_permission,
        crate::handlers::admin::list_permissions,
        crate::handlers::admin::create_menu,
        crate::handlers::admin::list_menus,
        crate::handlers::admin::assign_user_role,
        crate::handlers::admin::list_user_roles,
        crate::handlers::admin::grant_role_permission,
        crate::handlers::admin::current_user_permissions,
        crate::handlers::admin::current_user_menus,
    ),
    tags(
        (name = "admin", description = "RBAC and admin menu endpoints")
    ),
)]
pub struct AdminApiDoc;

pub fn admin_routes() -> Router {
    Router::new()
        .route("/admin-users", post(create_admin_user).get(list_admin_users))
        .route("/roles", post(create_role).get(list_roles))
        .route(
            "/permissions",
            post(create_permission).get(list_permissions),
        )
        .route("/menus", post(create_menu).get(list_menus))
        .route("/user-roles", post(assign_user_role))
        .route("/users/{user_id}/roles", get(list_user_roles))
        .route("/role-permissions", post(grant_role_permission))
        .route("/me/permissions", get(current_user_permissions))
        .route("/me/menus", get(current_user_menus))
        .route_layer(from_fn(auth::<VerifyJwt>))
}
