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
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret_env = get_jwt_env().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = crate::infrastructure::jwt::verify_token(secret_env.secret, token.to_string())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let brawler_id = claims
        .sub;

    req.extensions_mut().insert(brawler_id);

    Ok(next.run(req).await)
}
