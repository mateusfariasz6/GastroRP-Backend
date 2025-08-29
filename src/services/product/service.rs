use crate::services::product::models::ProductCategory;
use actix_web::{web, HttpResponse, Responder};
use crate::AppState;
use super::models::{AllProducts, SaveProduct};

pub async fn get_products(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(AllProducts,
        r#"
        SELECT id, name, price, category AS "category:ProductCategory"  FROM products_table ORDER BY id
        "#)
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(products) => {
            HttpResponse::Ok().json(products)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error trying to get products")
        }
    }
}

pub async fn save_product(app_state: web::Data<AppState>, product: web::Json<SaveProduct>) -> impl Responder {
    let result = sqlx::query!(r#"INSERT INTO products_table (name, price, category) VALUES ($1, $2, $3::text::product_category) RETURNING id, name, price, category AS "category:ProductCategory";"#, product.name, product.price, product.category as _)
        .fetch_one(&app_state.postgres_client)
        .await;

    match result {
        Ok(product) => {
            HttpResponse::Ok().json(AllProducts {
                id: product.id,
                name: product.name.clone(),
                price: product.price.clone(),
                category: product.category,
            })
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error trying to get products")
        }
    }
}

pub async fn get_product(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as!(AllProducts, r#"SELECT id, name, price, category AS "category:ProductCategory" FROM public.products_table WHERE id=$1"#, *id)
        .fetch_optional(&app_state.postgres_client)
        .await;

    if result.is_err() {
        HttpResponse::InternalServerError().body("Error trying to get products")
    } else {
        let product_opt = result.unwrap();
        if product_opt.is_none() {
            HttpResponse::NotFound().body("Product not found")
        } else {
            let product = product_opt.unwrap();

            HttpResponse::Ok().json(AllProducts {
                id: product.id,
                name: product.name.clone(),
                price: product.price.clone(),
                category: product.category
            })
        }
    }
}

pub async fn delete_product(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM products_table WHERE id=$1", *id)
        .execute(&app_state.postgres_client)
        .await;

    if result.is_err() {
        println!("{:?}", result);
        HttpResponse::InternalServerError().body("Error trying to delete product")
    } else {
        let pg_result = result.unwrap();
        if pg_result.rows_affected() == 0 {
            HttpResponse::NotFound().body("Product not found")
        } else {
            HttpResponse::NoContent().finish()
        }
    }
}