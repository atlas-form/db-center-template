use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use toolcraft_axum_kit::middleware::auth_mw::auth;
use toolcraft_jwt::VerifyJwt;

use crate::handlers::admin::{
    assign_user_role, create_admin_user, create_menu, create_permission, create_role,
    current_user_menus, current_user_permissions, grant_role_permission, list_admin_users,
    list_menus, list_permissions, list_roles, list_user_roles,
};

pub fn admin_routes() -> Router {
    Router::new()
        .route(
            "/admin-users",
            post(create_admin_user).get(list_admin_users),
        )
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
