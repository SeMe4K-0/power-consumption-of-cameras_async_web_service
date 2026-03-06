use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use crate::handlers::{init, AppState};

#[derive(Clone, Debug)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub backend_url: String,
    pub async_token: String,
    pub tariff: f64,
    pub min_delay_secs: u64,
    pub max_delay_secs: u64,
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("LISTEN_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8001);
        let backend_url = env::var("BACKEND_CALLBACK_URL").unwrap_or_else(|_| {
            "http://localhost:8080/api/request_cameras_calculations/update-consumption".to_string()
        });
        let async_token = env::var("ASYNC_TOKEN").unwrap_or_else(|_| {
            "a3f8b2c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1".to_string()
        });
        let tariff = env::var("TARIFF")
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(5.5);
        let min_delay_secs = env::var("MIN_DELAY_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(5);
        let max_delay_secs = env::var("MAX_DELAY_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(10);

        Self {
            host,
            port,
            backend_url,
            async_token,
            tariff,
            min_delay_secs,
            max_delay_secs,
        }
    }
}

pub async fn start_server(settings: Settings) -> std::io::Result<()> {
    let bind_host = settings.host.clone();
    let bind_port = settings.port;
    let app_state = web::Data::new(AppState::new(settings));

    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(init))
        .bind((bind_host, bind_port))?
        .run()
        .await
}
