use axum::{
    Router,
    middleware::from_fn,
    routing::{get, patch, post},
};
use toolcraft_axum_kit::middleware::auth_mw::auth;
use toolcraft_jwt::VerifyJwt;

use crate::handlers::admin::{
    assign_user_role, create_admin_user, create_menu, create_role, current_user_menus,
    delete_admin_user, delete_role, list_admin_users, list_menus, list_roles, list_user_roles,
    update_admin_user,
};

pub fn admin_routes() -> Router {
    Router::new()
        .route(
            "/admin-users",
            post(create_admin_user).get(list_admin_users),
        )
        .route(
            "/admin-users/{user_id}",
            patch(update_admin_user).delete(delete_admin_user),
        )
        .route("/roles", post(create_role).get(list_roles))
        .route("/roles/{role_id}", axum::routing::delete(delete_role))
        .route("/menus", post(create_menu).get(list_menus))
        .route("/user-roles", post(assign_user_role))
        .route("/users/{user_id}/roles", get(list_user_roles))
        .route("/me/menus", get(current_user_menus))
        .route_layer(from_fn(auth::<VerifyJwt>))
}
