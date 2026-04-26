use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, patch, post},
};
use toolcraft_axum_kit::middleware::auth_mw::auth;
use toolcraft_jwt::VerifyJwt;

use crate::handlers::admin::{
    create_admin_user, create_menu, create_role, current_user_menus, current_user_permissions,
    delete_admin_user, delete_role, list_admin_users, list_menus, list_permissions,
    list_role_permissions, list_roles, list_user_roles, update_admin_user, update_role_permissions,
    update_user_roles,
};

pub fn admin_routes() -> Router {
    Router::new()
        .route("/users", post(create_admin_user).get(list_admin_users))
        .route(
            "/users/{user_id}",
            patch(update_admin_user).delete(delete_admin_user),
        )
        .route(
            "/users/{user_id}/roles",
            get(list_user_roles).put(update_user_roles),
        )
        .route("/roles", post(create_role).get(list_roles))
        .route("/roles/{role_id}", delete(delete_role))
        .route(
            "/roles/{role_id}/permissions",
            get(list_role_permissions).put(update_role_permissions),
        )
        .route("/permissions", get(list_permissions))
        .route("/menus", post(create_menu).get(list_menus))
        .route("/me/permissions", get(current_user_permissions))
        .route("/me/menus", get(current_user_menus))
        .route_layer(from_fn(auth::<VerifyJwt>))
}
