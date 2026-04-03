use std::sync::Arc;

use axum::{Extension, Router};
use toolcraft_axum_kit::middleware::cors::create_cors;
use toolcraft_jwt::VerifyJwt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(nest())]
struct ApiDoc;

pub fn create_routes(jwt: Arc<VerifyJwt>) -> Router {
    let cors = create_cors();
    let doc = ApiDoc::openapi();

    Router::new()
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
