use axum::{
    Router,
    routing::{get, post},
    middleware,
};
use crate::handlers;
use crate::auth;

pub fn create_routes() -> Router<crate::database::DbPool> {
    let public_routes = Router::new()
        .route("/", get(root_handler))
        .route("/inventory", get(handlers::get_inventory))
        .route("/inventory/:product_id", get(handlers::get_product_stock));
    
    let protected_routes = Router::new()
        .route("/inventory/:product_id/increase", post(handlers::increase_stock))
        .route("/inventory/:product_id/decrease", post(handlers::decrease_stock))
        .layer(middleware::from_fn(auth::auth_middleware));
    
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}

async fn root_handler() -> &'static str {
    "Inventory Service funcionando"
}