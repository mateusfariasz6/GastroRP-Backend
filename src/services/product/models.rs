use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Serialize, Deserialize)]
pub struct AllProducts {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub category: ProductCategory
}

#[derive(Serialize, Deserialize)]
pub struct SaveProduct {
    pub name: String,
    pub price: f64,
    pub category: ProductCategory
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64,
    product_category: ProductCategory
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "product_category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductCategory {
    DRINK,
    FOOD,
    UNDEFINED,
}