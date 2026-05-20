use serde::{Deserialize, Serialize};

pub struct SecureAggregation;

impl SecureAggregation {
    pub fn new() -> Self { Self }
    pub fn aggregate(&self, shares: &[Vec<u8>]) -> Vec<u8> {
        shares.iter().fold(vec![], |mut acc, s| { acc.extend(s); acc })
    }
}
