use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, HeaderMap},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::Claims;

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let token = extract_token(headers).ok_or(StatusCode::UNAUTHORIZED)?;
    
    let secret = std::env::var("SECRET_KEY")
        .unwrap_or_else(|_| "default-secret-key".to_string());
    
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    
    match decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(_token_data) => {
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("authorization")?.to_str().ok()?;
    
    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}