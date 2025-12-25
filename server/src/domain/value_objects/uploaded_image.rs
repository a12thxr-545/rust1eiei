use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedImage {
    pub url: String,
    pub public_id: String,
}

impl UploadedImage {
    pub fn new(url: String, public_id: String) -> Self {
        Self { url, public_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedAvartar {
    pub base64_string: String,
}
