use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, patch, post},
};
use toolcraft_axum_kit::middleware::auth_mw::auth;
use toolcraft_jwt::VerifyJwt;

use crate::handlers::app::{
    create_app_user, create_role, current_user_permissions, delete_app_user, delete_role,
    list_app_users, list_permissions, list_role_permissions, list_roles, list_user_roles,
    update_app_user, update_role_permissions, update_user_roles,
};

pub fn app_admin_routes() -> Router {
    Router::new()
        .route("/users", post(create_app_user).get(list_app_users))
        .route(
            "/users/{user_id}",
            patch(update_app_user).delete(delete_app_user),
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
        .route_layer(from_fn(auth::<VerifyJwt>))
}

pub fn app_user_routes() -> Router {
    Router::new()
        .route("/me/permissions", get(current_user_permissions))
        .route_layer(from_fn(auth::<VerifyJwt>))
}
