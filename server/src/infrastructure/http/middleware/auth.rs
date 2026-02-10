use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::config::config_loader::get_jwt_env;

pub async fn authorization(mut req: Request, next: Next) -> Result<Response, StatusCode> {
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

    let brawler_id = claims.sub;

    req.extensions_mut().insert(brawler_id);

    Ok(next.run(req).await)
}
