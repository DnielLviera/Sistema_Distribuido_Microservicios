mod models;
mod handlers;
mod auth;

use axum::{
    routing::{get, post},
    Router,
    middleware,
    http::Method,
};
use tower_http::cors::{CorsLayer, Any};
use handlers::{InventoryState, load_inventory};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let inventory = InventoryState::new(std::sync::Mutex::new(load_inventory()));
    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);
    
    // Rutas
    let public_routes = Router::new()
        .route("/", get(|| async { "Inventory Service funcionando" }))
        .route("/inventory", get(handlers::get_inventory))
        .route("/inventory/:product_id", get(handlers::get_product_stock));
        
    let protected_routes = Router::new()
        .route("/inventory/:product_id/increase", post(handlers::increase_stock))
        .route("/inventory/:product_id/decrease", post(handlers::decrease_stock))
        .layer(middleware::from_fn(auth::auth_middleware));
        
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(inventory)
        .layer(cors);
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8002".to_string())
        .parse()
        .unwrap_or(8002);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    println!("Inventory Service corriendo en http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}