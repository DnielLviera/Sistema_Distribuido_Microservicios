use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use std::sync::{Arc, Mutex};
use crate::models::{InventoryItem, StockUpdate};

pub type InventoryState = Arc<Mutex<Vec<InventoryItem>>>;

pub fn load_inventory() -> Vec<InventoryItem> {
    if let Ok(file_content) = std::fs::read_to_string("inventory.json") {
        match serde_json::from_str::<Vec<InventoryItem>>(&file_content) {
            Ok(inventory) => {
                println!("JSON parseado correctamente, {} items", inventory.len());
                inventory
            },
            Err(e) => {
                println!("Error parseando JSON: {}", e);
                vec![]
            }
        }
    } else {
        println!("Archivo inventory.json no encontrado");
        vec![]
    }
}

pub fn save_inventory(inventory: &[InventoryItem]) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(inventory)?;
    std::fs::write("inventory.json", json)
}

// GET /inventory - Obtener todo el inventario
pub async fn get_inventory(
    State(inventory): State<InventoryState>
) -> Json<Vec<InventoryItem>> {
    let inventory = inventory.lock().unwrap();
    Json(inventory.clone())
}

// GET /inventory/{product_id} - Obtener stock de un producto
pub async fn get_product_stock(
    Path(product_id): Path<u32>,
    State(inventory): State<InventoryState>
) -> Result<Json<InventoryItem>, StatusCode> {
    let inventory = inventory.lock().unwrap();
    
    inventory
        .iter()
        .find(|item| item.product_id == product_id)
        .map(|item| Json(item.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /inventory/{product_id}/increase - Aumentar stock
pub async fn increase_stock(
    Path(product_id): Path<u32>,
    State(inventory): State<InventoryState>,
    Json(update): Json<StockUpdate>,
) -> Result<Json<InventoryItem>, StatusCode> {
    let item_clone = {
        let mut inventory_guard = inventory.lock().unwrap();
        
        if let Some(item) = inventory_guard.iter_mut().find(|item| item.product_id == product_id) {
            item.stock += update.quantity;
            item.clone()
        } else {
            return Err(StatusCode::NOT_FOUND);
        }
    };
    
    let inventory_guard = inventory.lock().unwrap();
    save_inventory(&inventory_guard).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(item_clone))
}

// POST /inventory/{product_id}/decrease - Disminuir stock
pub async fn decrease_stock(
    Path(product_id): Path<u32>,
    State(inventory): State<InventoryState>,
    Json(update): Json<StockUpdate>,
) -> Result<Json<InventoryItem>, StatusCode> {
    let item_clone = {
        let mut inventory_guard = inventory.lock().unwrap();
        
        if let Some(item) = inventory_guard.iter_mut().find(|item| item.product_id == product_id) {
            item.stock -= update.quantity;
            if item.stock < 0 {
                item.stock = 0;
            }
            item.clone()
        } else {
            return Err(StatusCode::NOT_FOUND);
        }
    };
    
    let inventory_guard = inventory.lock().unwrap();
    save_inventory(&inventory_guard).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(item_clone))
}