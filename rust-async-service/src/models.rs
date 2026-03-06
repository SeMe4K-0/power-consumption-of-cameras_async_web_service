use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CalcRequest {
    pub request_id: u64,
    pub calculations: Vec<CalcInput>,
}

#[derive(Debug, Deserialize)]
pub struct CalcInput {
    pub id: u64,
    pub power: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CalcResult {
    pub calculation_id: u64,
    pub monthly_cost: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct AsyncResultPayload {
    pub request_id: u64,
    pub calculations: Vec<CalcResult>,
    pub token: String,
}
