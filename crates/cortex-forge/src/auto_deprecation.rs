use serde::{Deserialize, Serialize};

pub struct AutoDeprecation {
    threshold: f64,
}

impl AutoDeprecation {
    pub fn new() -> Self { Self { threshold: 0.7 } }
    pub fn should_deprecate(&self, success_rate: f64) -> bool {
        success_rate < self.threshold
    }
}
