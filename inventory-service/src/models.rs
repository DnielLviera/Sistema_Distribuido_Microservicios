use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub product_id: u32,
    pub stock: i32,
    pub min_stock: i32,
    pub max_stock: i32,
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