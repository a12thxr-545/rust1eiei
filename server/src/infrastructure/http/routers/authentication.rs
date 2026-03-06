use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::cookie::Cookie;
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

use axum::extract::Query;
use axum::response::Redirect;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LineCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawlers_repository = BrawlerPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(Arc::new(brawlers_repository));

    let auth_routes = Router::new()
        .route("/me", axum::routing::get(get_me))
        .layer(axum::middleware::from_fn(
            crate::infrastructure::http::middleware::auth::authorization,
        ));

    Router::new()
        .route("/login", post(login))
        .route("/line/login", axum::routing::get(line_login_redirect))
        .route("/line/callback", axum::routing::get(line_callback))
        .merge(auth_routes)
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

pub async fn get_me<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    axum::extract::Extension(brawler_id): axum::extract::Extension<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match authentication_use_case.get_me(brawler_id).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn line_login_redirect() -> impl IntoResponse {
    let line_env = match crate::config::config_loader::get_line_env() {
        Ok(env) => env,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Config error").into_response(),
    };

    let authorize_url = format!(
        "https://access.line.me/oauth2/v2.1/authorize?response_type=code&client_id={}&redirect_uri={}&state={}&scope=profile%20openid",
        line_env.channel_id,
        urlencoding::encode(&line_env.callback_url),
        uuid::Uuid::new_v4()
    );

    Redirect::temporary(&authorize_url).into_response()
}

pub async fn line_callback<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Query(query): Query<LineCallbackQuery>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    if let Some(err) = query.error {
        return (
            StatusCode::BAD_REQUEST,
            format!("LINE Auth Error: {} - {:?}", err, query.error_description),
        )
            .into_response();
    }

    let code = match query.code {
        Some(code) => code,
        None => return (StatusCode::BAD_REQUEST, "No code provided").into_response(),
    };

    match authentication_use_case.line_login(&code).await {
        Ok(passport) => {
            let line_env = crate::config::config_loader::get_line_env().unwrap();
            let mut redirect_url = line_env.frontend_url.clone();

            // Handle trailing slash
            if !redirect_url.ends_with('/') {
                redirect_url.push('/');
            }

            // Redirect to frontend with token in query string
            redirect_url.push_str(&format!("login?token={}", passport.access_token));

            Redirect::temporary(&redirect_url).into_response()
        }
        Err(e) => {
            tracing::error!("LINE login error: {:?}", e);
            let line_env = crate::config::config_loader::get_line_env().unwrap();
            let mut redirect_url = line_env.frontend_url.clone();
            if !redirect_url.ends_with('/') {
                redirect_url.push('/');
            }
            redirect_url.push_str(&format!(
                "login?error=line_auth_failed&detail={}",
                urlencoding::encode(&e.to_string())
            ));
            Redirect::temporary(&redirect_url).into_response()
        }
    }
}
