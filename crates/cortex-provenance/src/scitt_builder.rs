/// SCITT (Supply Chain Integrity, Transparency, and Trust) receipt builder.
pub struct SCITTReceiptBuilder;

impl SCITTReceiptBuilder {
    pub fn new() -> Self { Self }

    pub fn build_receipt(&self, merkle_root: &str) -> String {
        format!("SCITT:{}:{}", chrono::Utc::now().to_rfc3339(), merkle_root)
    }
}
