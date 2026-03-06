use actix_web::{post, web, HttpResponse};
use reqwest::Client;
use tokio::spawn;

use crate::{
    calculation::calculate,
    models::{AsyncResultPayload, CalcRequest},
    server::Settings,
};

#[derive(Clone)]
pub struct AppState {
    settings: Settings,
    client: Client,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            client: Client::new(),
        }
    }
}

#[post("/calculate-consumption")]
async fn handle_request(req: web::Json<CalcRequest>, state: web::Data<AppState>) -> HttpResponse {
    start_calculation(req.into_inner(), state.get_ref().clone());

    HttpResponse::Ok().json(serde_json::json!({
        "status": "accepted",
        "message": "calculation started"
    }))
}

fn start_calculation(req: CalcRequest, state: AppState) {
    spawn(async move {
        let results = calculate(&req, &state.settings).await;

        if let Err(err) = send_result(req.request_id, results, &state).await {
            eprintln!("failed to send async result: {err}");
        }
    });
}

async fn send_result(
    request_id: u64,
    calculations: Vec<crate::models::CalcResult>,
    state: &AppState,
) -> Result<(), reqwest::Error> {
    let payload = AsyncResultPayload {
        request_id,
        calculations,
        token: state.settings.async_token.clone(),
    };

    state
        .client
        .post(&state.settings.backend_url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_request);
}
