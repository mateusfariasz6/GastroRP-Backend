use serde::{Serialize, Deserialize};
use sqlx::Type;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AllProducts {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub category: ProductCategory
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SaveProduct {
    pub name: String,
    pub price: f64,
    pub category: ProductCategory
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64,
    product_category: ProductCategory
}

#[derive(Debug, Serialize, Deserialize, Type, JsonSchema, ApiComponent)]
#[sqlx(type_name = "product_category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductCategory {
    DRINK,
    FOOD,
    UNDEFINED,
}