use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct InventoryItem {
    pub product_id: i32,
    pub stock: i32,
    pub product_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockUpdate {
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}