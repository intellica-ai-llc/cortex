/// Anchors receipts via SCITT.
pub struct SCITTAnchoringService;

impl SCITTAnchoringService {
    pub fn new() -> Self { Self }
    pub fn anchor(&self, receipt: &str) -> String {
        format!("scitt:{}", receipt)
    }
}
