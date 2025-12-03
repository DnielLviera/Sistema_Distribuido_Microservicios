use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::{InventoryItem, StockUpdate};

pub type DbState = PgPool;

// GET /inventory - Obtener todo el inventario
pub async fn get_inventory(
    State(pool): State<DbState>
) -> Result<Json<Vec<InventoryItem>>, StatusCode> {
    let items = sqlx::query_as::<_, InventoryItem>("SELECT product_id, stock, product_name FROM products ORDER BY product_id")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(items))
}

// GET /inventory/{product_id} - Obtener stock de un producto
pub async fn get_product_stock(
    Path(product_id): Path<i32>,
    State(pool): State<DbState>
) -> Result<Json<InventoryItem>, StatusCode> {
    let item = sqlx::query_as::<_, InventoryItem>(
        "SELECT product_id, stock, product_name FROM products WHERE product_id = $1"
    )
    .bind(product_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    item.map(Json).ok_or(StatusCode::NOT_FOUND)
}

// POST /inventory/{product_id}/increase - Aumentar stock
pub async fn increase_stock(
    Path(product_id): Path<i32>,
    State(pool): State<DbState>,
    Json(update): Json<StockUpdate>,
) -> Result<Json<InventoryItem>, StatusCode> {
    let item = sqlx::query_as::<_, InventoryItem>(
        "UPDATE products SET stock = stock + $1 WHERE product_id = $2 RETURNING product_id, stock, product_name"
    )
    .bind(update.quantity)
    .bind(product_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    item.map(Json).ok_or(StatusCode::NOT_FOUND)
}

// POST /inventory/{product_id}/decrease - Disminuir stock
pub async fn decrease_stock(
    Path(product_id): Path<i32>,
    State(pool): State<DbState>,
    Json(update): Json<StockUpdate>,
) -> Result<Json<InventoryItem>, StatusCode> {
    let item = sqlx::query_as::<_, InventoryItem>(
        "UPDATE products SET stock = GREATEST(stock - $1, 0) WHERE product_id = $2 RETURNING product_id, stock, product_name"
    )
    .bind(update.quantity)
    .bind(product_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    item.map(Json).ok_or(StatusCode::NOT_FOUND)
}