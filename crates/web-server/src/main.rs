mod clients;
mod dto;
mod error;
mod handlers;
mod logging;
mod routes;
mod settings;
mod statics;

use std::sync::Arc;

use clients::auth_client::fetch_verify_jwt;
use settings::Settings;
use toolcraft_axum_kit::http_server;

use crate::{
    logging::init_tracing_to_file,
    statics::{
        db_manager::init_db,
        request_client::init_request_client,
    },
};

#[tokio::main]
async fn main() {
    init_tracing_to_file();
    let settings = Settings::load("config/services.toml").unwrap();
    init_request_client(
        settings.jwt_verify.header.clone(),
        settings.jwt_verify.token.clone(),
    )
    .expect("Request client initialization failed");
    init_db(settings.db.clone())
        .await
        .expect("DatabaseManager initialization failed");
    let jwt = Arc::new(
        fetch_verify_jwt(&settings.jwt_verify)
            .await
            .expect("VerifyJwt initialization failed"),
    );

    let router = routes::create_routes(jwt);
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
