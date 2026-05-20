use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct MetricCollector {
    gauges: RwLock<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub name: String,
    pub value: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MetricCollector {
    pub fn new() -> Self { Self { gauges: RwLock::new(HashMap::new()) } }
    pub async fn record(&self, name: &str, value: f64) {
        self.gauges.write().await.insert(name.to_string(), value);
    }
    pub async fn snapshot(&self, name: &str) -> Option<MetricSnapshot> {
        self.gauges.read().await.get(name).map(|&v| MetricSnapshot {
            name: name.to_string(),
            value: v,
            timestamp: chrono::Utc::now(),
        })
    }
}
