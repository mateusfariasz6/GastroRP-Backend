use actix_web::{web, HttpResponse, Responder};
use crate::AppState;
use super::model::{Order, OrderItem, SaveOrder};
use apistos::api_operation;


#[api_operation(
    summary = "Lista pedidos",
    tag = "orders",
    error_code = 500
)]
pub async fn get_orders(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Order, r#"SELECT * FROM orders_table"#)
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[api_operation(
    summary = "Cista pedido",
    tag = "orders",
    error_code = 500
)]
pub async fn create_order(app_state: web::Data<AppState>, order: web::Json<Vec<SaveOrder>>) -> impl Responder {
    let total_price = calc_total_price(&order.0);
    let order_saved = sqlx::query_as!(Order, r#"INSERT INTO orders_table (total_price) VALUES ($1) RETURNING *"#, total_price)
        .fetch_one(&app_state.postgres_client)
        .await;

    match order_saved {
        Ok(order_sa) => {
            for prod in order.0 {
                sqlx::query!(r#"INSERT INTO order_items (order_id, product_id, quantity) VALUES ($1, $2, $3)"#, order_sa.id, prod.product.id, prod.quantity)
                    .execute(&app_state.postgres_client)
                    .await.expect("Error trying to save an order item");
            }
            HttpResponse::Ok().json(order_sa)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[api_operation(
    summary = "Retorna pedido pelo id",
    tag = "orders",
    error_code = 500
)]
pub async fn get_order_by_id(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();
    let result_order = sqlx::query_as!(Order, r#"SELECT * FROM orders_table WHERE id = $1"#, id)
        .fetch_one(&app_state.postgres_client)
        .await;

    let result_items = sqlx::query_as!(OrderItem, r#"SELECT * FROM order_items WHERE order_id = $1"#, id)
        .fetch_all(&app_state.postgres_client)
        .await;

    if result_order.is_ok() && result_items.is_ok() {
        HttpResponse::Ok().json((result_order.unwrap(), result_items.unwrap()))
    } else {
        HttpResponse::InternalServerError().body("Error")
    }

}

fn calc_total_price(itens: &Vec<SaveOrder>) -> f64 {
    let mut total_price = 0.0;
    for x in itens {
        total_price += x.product.price * x.quantity as f64;
    }
    total_price
}