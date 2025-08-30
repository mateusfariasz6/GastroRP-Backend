mod database;
mod services;

use actix_web::{web, App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::info::Info;


use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use crate::services::order::service::{get_orders, create_order, get_order_by_id};
use crate::services::product::service::{get_products, save_product, get_product, delete_product};

#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let _pool = database::start_connection().await;

    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "An API".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        App::new().document(spec)
        .app_data({
            web::Data::new(AppState {
                postgres_client: _pool.clone()
            })
        })
        .configure(config).build("/openapi.json")})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn config(config: &mut apistos::web::ServiceConfig) {
    config.service(apistos::web::scope("/api")
        .route("/products", apistos::web::get().to(get_products))
        .route("/products", apistos::web::post().to(save_product))
        .route("/products/{id}", apistos::web::get().to(get_product))
        .route("/products/{id}", apistos::web::delete().to(delete_product))
        .route("/orders", apistos::web::get().to(get_orders))
        .route("/orders", apistos::web::post().to(create_order))
        .route("/orders/{id}", apistos::web::get().to(get_order_by_id)));
}
