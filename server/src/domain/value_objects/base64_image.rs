use anyhow::Result;
use base64::{Engine, engine::general_purpose};



#[derive(Debug, Clone)]
pub struct Base64Image(String);

impl Base64Image {
    pub fn new(data: &String) -> Result<Self> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("Base64Image is empty !!"));
        }

        let bytes = match general_purpose::STANDARD.decode(data) {
            Ok(bs) => bs,
            Err(_) => return Err(anyhow::anyhow!("Invalid base64 image data.")),
        };
        let file_type = match infer::get(&bytes) {
            Some(t) => if t.mime_type() == "image/png" || t.mime_type() == "image/jpeg" {
                t.mime_type()
            } else {
                return Err(anyhow::anyhow!("Invalid base64 image data."));
            },
            _ => return Err(anyhow::anyhow!("Invalid base64 image data.")),
        };

        Ok(Self(format!("data:{};base64,{}", file_type, &data)))
    }   
    pub fn into_inner(self) -> String {
        self.0
    }
}

