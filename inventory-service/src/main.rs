mod models;
mod handlers;
mod auth;
mod database;
mod routes;
mod init_db;

use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use database::create_pool;
use routes::create_routes;
use init_db::init_db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool = create_pool().await.expect("Failed to create pool");
   
    init_db(&pool).await;

    let cors = CorsLayer::new()
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            ])
        .allow_origin(Any)
        .allow_headers(Any);
        
    let app = Router::new()
        .merge(create_routes())
        .with_state(pool)
        .layer(cors);
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8002".to_string())
        .parse()
        .unwrap_or(8002);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("Inventory Service corriendo en http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}