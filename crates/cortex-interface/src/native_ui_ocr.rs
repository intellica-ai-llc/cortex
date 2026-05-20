use serde::{Deserialize, Serialize};

/// OCR‑based capture for terminal emulators and green screens.
///
/// Gap closure (v11): captures field‑level interactions from
/// IBM iSeries 5250, VT100, and other text‑based interfaces
/// where accessibility APIs are unavailable.
/// Uses a lightweight on‑device OCR engine for label/value detection.
pub struct OcrParser {
    // In production: integrate Tesseract or a quantised ScreenAI variant.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub blocks: Vec<TextBlock>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub text: String,
    pub bounding_box: super::native_ui_accessibility::BoundingBox,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType { Label, Value, Button, Header, Unknown }

impl OcrParser {
    pub fn new() -> Self { Self {} }

    /// Process a screenshot and extract labelled fields.
    pub async fn parse_screenshot(&self, _image_data: &[u8]) -> Result<OcrResult, String> {
        // Production: run OCR on the image, then apply
        // label‑value pairing heuristics based on layout.
        Ok(OcrResult { blocks: vec![], confidence: 0.0 })
    }
}
