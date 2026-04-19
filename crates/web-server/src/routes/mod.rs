mod admin;
mod auth_example;

use std::sync::Arc;

use admin::{AdminApiDoc, admin_routes};
use auth_example::{AuthExampleApiDoc, auth_example_routes};
use axum::{Extension, Router};
use toolcraft_axum_kit::middleware::cors::create_cors;
use toolcraft_jwt::VerifyJwt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/admin", api = AdminApiDoc),
        (path = "/example", api = AuthExampleApiDoc),
    )
)]
struct ApiDoc;

pub fn create_routes(jwt: Arc<VerifyJwt>) -> Router {
    let cors = create_cors();
    let doc = ApiDoc::openapi();

    Router::new()
        .nest("/admin", admin_routes())
        .nest("/example", auth_example_routes())
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
