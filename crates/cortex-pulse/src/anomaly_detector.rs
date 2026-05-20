use serde::{Deserialize, Serialize};

/// Detects deviations from multi‑modal baselines.
pub struct AnomalyDetector {
    threshold_sigma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyAlert {
    pub user_id: String,
    pub metric: String,
    pub current_value: f64,
    pub baseline_mean: f64,
    pub sigma_deviation: f64,
}

impl AnomalyDetector {
    pub fn new(threshold_sigma: f64) -> Self { Self { threshold_sigma } }

    pub fn detect(&self, value: f64, mean: f64, std: f64) -> Option<AnomalyAlert> {
        if std == 0.0 { return None; }
        let deviation = (value - mean).abs() / std;
        if deviation > self.threshold_sigma {
            Some(AnomalyAlert {
                user_id: String::new(),
                metric: String::new(),
                current_value: value,
                baseline_mean: mean,
                sigma_deviation: deviation,
            })
        } else { None }
    }
}
