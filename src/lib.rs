use rocket::config;
use serde::Deserialize;

#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: String::from("Success"),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

#[derive(Deserialize)]
pub struct CutewebConfig {
    pub db: String,
}

pub fn get_config() -> CutewebConfig {
    config::Config::figment()
        .extract::<CutewebConfig>()
        .expect("Invalid rocket configuration")
}
