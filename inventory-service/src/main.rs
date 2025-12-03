mod models;
mod handlers;
mod auth;
mod database;

use axum::{
    routing::{get, post},
    Router,
    middleware,
    http::Method,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use database::create_pool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool = create_pool().await.expect("Failed to create pool");
   
    init_db(&pool).await;

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

async fn init_db(pool: &sqlx::PgPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            product_id SERIAL PRIMARY KEY,
            product_name VARCHAR(255) NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create table");
    
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await
        .expect("Failed to count products");
    
    if count.0 == 0 {
        sqlx::query(
            r#"
            INSERT INTO products (product_name, stock) VALUES
            ('Laptop', 10),
            ('Mouse', 50),
            ('Teclado', 30),
            ('Monitor', 15)
            "#
        )
        .execute(pool)
        .await
        .expect("Failed to insert sample data");
    }
}