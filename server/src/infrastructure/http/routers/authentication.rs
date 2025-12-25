use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::cookie::{Cookie};
use cookie::time::Duration;

use crate::{
    application::use_cases::authentication::AuthenticationUseCase,
    config::{config_loader::get_stage, stage::Stage},
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        jwt::authentication_model::LoginModel,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawlers_repository = BrawlerPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(Arc::new(brawlers_repository));

    Router::new()
        .route("/login", post(login))
        // .route("/refresh-token", post(refresh_token))
        .with_state(Arc::new(authentication_use_case))
}

pub async fn login<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match authentication_use_case.login(login_model).await {
        Ok(passport) => {
            let mut token = Cookie::build(("token", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(7));

            let mut refresh_token = Cookie::build(("refresh_token", passport.token_type.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(7));

            if get_stage() == Stage::Production {
                refresh_token = refresh_token.secure(true);
                token = token.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&token.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&refresh_token.to_string()).unwrap(),
            );

            (
                StatusCode::OK,
                headers,
                Json(serde_json::json!({
                    "access_token": passport.access_token,
                    "token_type": passport.token_type,
                    "expires_in": passport.expires_in,
                    "message": "Login successfully"
                })),
            )
                .into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

// pub async fn refresh_token<T>(
//     State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
//     jar: CookieJar,
// ) -> impl IntoResponse
// where
//     T: BrawlerRepository + Send + Sync,
// {
//     if let Some(rft) = jar.get("refresh_token") {
//         let refresh_token = rft.value().to_string();

//         let response = match authentication_use_case.refresh_token(refresh_token).await {
//             Ok(passport) => {
//                 let mut token = Cookie::build(("token", passport.access_token.clone()))
//                     .path("/")
//                     .same_site(cookie::SameSite::Lax)
//                     .http_only(true)
//                     .max_age(Duration::days(7));

//                 let mut refresh_token =
//                     Cookie::build(("refresh_token", passport.token_type.clone()))
//                         .path("/")
//                         .same_site(cookie::SameSite::Lax)
//                         .http_only(true)
//                         .max_age(Duration::days(7));

//                 if get_stage() == Stage::Production {
//                     token = token.secure(true);
//                     refresh_token = refresh_token.secure(true);
//                 }

//                 let mut headers = HeaderMap::new();
//                 headers.append(
//                     header::SET_COOKIE,
//                     HeaderValue::from_str(&token.to_string()).unwrap(),
//                 );
//                 headers.append(
//                     header::SET_COOKIE,
//                     HeaderValue::from_str(&refresh_token.to_string()).unwrap(),
//                 );

//                 (StatusCode::OK, headers, "Refresh token successfully").into_response()
//             }
//             Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
//         };
//         return response;
//     }

//     (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
// }
