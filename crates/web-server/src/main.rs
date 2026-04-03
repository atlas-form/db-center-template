mod dto;
mod error;
mod handlers;
mod logging;
mod routes;
mod settings;
mod statics;

use settings::Settings;
use toolcraft_axum_kit::http_server;
use toolcraft_jwt::VerifyJwt;

use crate::{logging::init_tracing_to_file, statics::db_manager::init_db};

#[tokio::main]
async fn main() {
    init_tracing_to_file();
    let settings = Settings::load("config/services.toml").unwrap();
    init_db(settings.db.clone())
        .await
        .expect("DatabaseManager initialization failed");

    let jwt = VerifyJwt::new(settings.jwt).unwrap();
    let router = routes::create_routes(jwt.into());
    let http_task = http_server::start(settings.http.port, router);

    let _ = tokio::join!(http_task);
}
