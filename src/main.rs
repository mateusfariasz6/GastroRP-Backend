mod database;
mod services;

use actix_web::{web, App, HttpServer};

use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use crate::services::product::service::{get_products, save_product};

#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let _pool = database::start_connection().await;

    HttpServer::new(move || {App::new()
        .app_data({
            web::Data::new(AppState {
                postgres_client: _pool.clone()
            })
        })
        .configure(config)})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api")
        .route("/products", web::get().to(get_products))
        .route("/products", web::post().to(save_product)));
}
