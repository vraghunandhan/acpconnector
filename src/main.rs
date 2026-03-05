mod errors;
mod models;
mod routes;
mod storage;
mod validation;

use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use storage::redis_store::RedisStorage;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<dyn storage::Storage>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    info!("Connecting to Redis at {}", redis_url);
    let storage = RedisStorage::new(&redis_url)
        .expect("Failed to connect to Redis");

    let app_state = AppState {
        storage: Arc::new(storage),
    };

    info!("Starting server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::Logger::default())
            .configure(routes::configure)
    })
    .bind(&bind_address)?
    .run()
    .await
}
