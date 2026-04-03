use axum::{Router, middleware::from_fn, routing::get};
use toolcraft_axum_kit::middleware::auth_mw::auth;
use toolcraft_jwt::VerifyJwt;
use utoipa::OpenApi;

use crate::handlers::auth_example::current_user;

#[derive(OpenApi)]
#[openapi(
    paths(crate::handlers::auth_example::current_user),
    tags(
        (name = "auth-example", description = "JWT protected example endpoints")
    ),
)]
pub struct AuthExampleApiDoc;

pub fn auth_example_routes() -> Router {
    Router::new()
        .route("/me", get(current_user))
        .route_layer(from_fn(auth::<VerifyJwt>))
}
