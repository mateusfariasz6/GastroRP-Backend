use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::services::product::models::AllProducts;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct Order {
    pub id: i32,
    pub total_price: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SaveOrder {
    pub quantity: i32,
    pub product: AllProducts
}