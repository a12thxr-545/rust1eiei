use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LineTokenResponse {
    pub access_token: String,
    pub id_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LineProfileResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "pictureUrl")]
    pub picture_url: Option<String>,
    #[serde(rename = "statusMessage")]
    pub status_message: Option<String>,
}

pub async fn get_access_token(
    code: &str,
    channel_id: &str,
    channel_secret: &str,
    callback_url: &str,
) -> Result<LineTokenResponse> {
    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", callback_url),
        ("client_id", channel_id),
        ("client_secret", channel_secret),
    ];

    let res = client
        .post("https://api.line.me/oauth2/v2.1/token")
        .form(&params)
        .send()
        .await
        .context("Failed to send token request to LINE")?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(anyhow::anyhow!("LINE token error: {}", err_text));
    }

    let token_res: LineTokenResponse = res.json().await.context("Failed to parse LINE token")?;
    Ok(token_res)
}

pub async fn get_profile(access_token: &str) -> Result<LineProfileResponse> {
    let client = Client::new();
    let res = client
        .get("https://api.line.me/v2/profile")
        .bearer_auth(access_token)
        .send()
        .await
        .context("Failed to get profile from LINE")?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(anyhow::anyhow!("LINE profile error: {}", err_text));
    }

    let profile: LineProfileResponse = res.json().await.context("Failed to parse LINE profile")?;
    Ok(profile)
}
