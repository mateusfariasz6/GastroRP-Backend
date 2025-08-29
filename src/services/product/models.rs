use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AllProducts {
    pub id: i32,
    pub name: String,
    pub price: f64
}

#[derive(Serialize, Deserialize)]
pub struct SaveProduct {
    pub name: String,
    pub price: f64
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64
}