use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::config::config_loader::get_jwt_env;

pub async fn authorization(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    match extract_brawler_id(&req) {
        Ok(brawler_id) => {
            tracing::info!("Authorized user: {} for {}", brawler_id, req.uri());
            req.extensions_mut().insert(brawler_id);
            // Log that it was inserted
            if req.extensions().get::<i32>().is_some() {
                tracing::info!("Extension i32 successfully inserted");
            } else {
                tracing::error!("Extension i32 insertion FAILED!");
            }
            Ok(next.run(req).await)
        }
        Err(status) => {
            tracing::error!("Authorization failed for {}: {}", req.uri(), status);
            Err(status)
        }
    }
}

pub async fn optional_authorization(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Ok(brawler_id) = extract_brawler_id(&req) {
        req.extensions_mut().insert(brawler_id);
    }
    Ok(next.run(req).await)
}

fn extract_brawler_id(req: &Request) -> Result<i32, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let query_token = req
        .uri()
        .query()
        .and_then(|q| q.split('&').find(|p| p.starts_with("token=")))
        .and_then(|p| p.strip_prefix("token="));

    let token = auth_header
        .or(query_token)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret_env = get_jwt_env().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = crate::infrastructure::jwt::verify_token(secret_env.secret, token.to_string())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(claims.sub)
}
