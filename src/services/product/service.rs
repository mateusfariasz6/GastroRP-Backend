use actix_web::{web, HttpResponse, Responder};
use crate::AppState;
use super::models::{AllProducts, SaveProduct};

pub async fn get_products(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        AllProducts,
        r#"
        SELECT id, name, price FROM products_table
        "#)
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(products) => {
            HttpResponse::Ok().json(
                products
                    .iter()
                    .map(|product| AllProducts {
                    id: product.id,
                    name: product.name.clone(),
                    price: product.price.clone(),
                }).collect::<Vec<AllProducts>>()
            )
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error trying to get products")
        }
    }
}

pub async fn save_product(app_state: web::Data<AppState>, product: web::Json<SaveProduct>) -> impl Responder {
    let result = sqlx::query!("INSERT INTO products_table (name, price) VALUES ($1, $2) RETURNING id, name, price;", product.name, product.price)
        .fetch_one(&app_state.postgres_client)
        .await;

    match result {
        Ok(product) => {
            HttpResponse::Ok().json(AllProducts {
                id: product.id,
                name: product.name.clone(),
                price: product.price.clone(),
            })
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error trying to get products")
        }
    }
}