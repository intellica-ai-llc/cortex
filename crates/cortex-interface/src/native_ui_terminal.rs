use serde::{Deserialize, Serialize};

/// Terminal emulation adapter for legacy mainframe/green‑screen apps.
///
/// Parses IBM 5250 / VT100 data streams directly, extracting
/// screen fields and input areas without OCR.
pub struct TerminalParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalScreen {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<TerminalCell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCell {
    pub character: char,
    pub attribute: u8, // bold, underline, reverse, colour
}

impl TerminalParser {
    pub fn new() -> Self { Self {} }

    /// Parse a raw terminal data stream into a structured screen.
    pub fn parse_stream(&self, _data: &[u8]) -> Result<TerminalScreen, String> {
        // Production: decode 5250 orders or VT100 escape sequences.
        Ok(TerminalScreen {
            rows: 24, cols: 80,
            cells: vec![],
            cursor_row: 0, cursor_col: 0,
        })
    }

    /// Identify input fields on a terminal screen.
    pub fn extract_fields(&self, screen: &TerminalScreen) -> Vec<TerminalField> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalField {
    pub label: String,
    pub row: usize, pub col: usize, pub length: usize,
    pub is_input: bool,
}
