use tokio::time::{sleep, Duration};
use rand::Rng;

use crate::{
    models::{CalcRequest, CalcResult},
    server::Settings,
};

const HOURS_PER_DAY: f64 = 24.0;
const DAYS_PER_MONTH: f64 = 30.0;

pub async fn calculate(req: &CalcRequest, settings: &Settings) -> Vec<CalcResult> {
    let (min_delay, max_delay) = normalize_delay_bounds(settings.min_delay_secs, settings.max_delay_secs);
    let delay_secs = rand::thread_rng().gen_range(min_delay..=max_delay);
    sleep(Duration::from_secs(delay_secs)).await;

    req.calculations
        .iter()
        .map(|item| CalcResult {
            calculation_id: item.id,
            monthly_cost: calculate_monthly_cost(item.power, settings.tariff),
        })
        .collect()
}

pub fn calculate_monthly_cost(power_watts: f64, tariff: f64) -> Option<f64> {
    if power_watts <= 0.0 || tariff < 0.0 {
        return None;
    }

    let monthly_cost = (power_watts * HOURS_PER_DAY * DAYS_PER_MONTH * tariff) / 1000.0;
    Some(monthly_cost)
}

fn normalize_delay_bounds(min_delay: u64, max_delay: u64) -> (u64, u64) {
    if min_delay <= max_delay {
        (min_delay, max_delay)
    } else {
        (max_delay, min_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CalcInput, CalcRequest};
    use crate::server::Settings;

    #[test]
    fn test_calculate_monthly_cost() {
        let result = calculate_monthly_cost(100.0, 5.5).unwrap();
        assert_eq!(result, 396.0);
    }

    #[tokio::test]
    async fn test_calculate_async_result() {
        let req = CalcRequest {
            request_id: 1,
            calculations: vec![
                CalcInput { id: 10, power: 100.0 },
                CalcInput { id: 11, power: 0.0 },
            ],
        };

        let settings = Settings {
            host: "127.0.0.1".to_string(),
            port: 8001,
            backend_url: "http://localhost:8080/api/request_cameras_calculations/update-consumption".to_string(),
            async_token: "token".to_string(),
            tariff: 5.5,
            min_delay_secs: 0,
            max_delay_secs: 0,
        };

        let results = calculate(&req, &settings).await;

        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0],
            CalcResult {
                calculation_id: 10,
                monthly_cost: Some(396.0)
            }
        );
        assert_eq!(
            results[1],
            CalcResult {
                calculation_id: 11,
                monthly_cost: None
            }
        );
    }
}
