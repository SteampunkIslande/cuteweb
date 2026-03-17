use rocket::config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CutewebConfig {
    pub db: String,
    pub local: bool,
}

pub fn get_config() -> CutewebConfig {
    config::Config::figment()
        .extract::<CutewebConfig>()
        .expect("Invalid rocket configuration")
}
