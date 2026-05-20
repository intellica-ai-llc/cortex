use serde::{Deserialize, Serialize};

/// Local transcription engine — no cloud dependency.
///
/// Runs entirely on‑device. In production, uses a quantised
/// Whisper.cpp or Canary Speech local model. No audio data
/// leaves the device (Privacy Architecture v5).
pub struct WhisperTranscriber;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub language: Option<String>,
    pub confidence: f64,
    pub segments: Vec<TranscriptionSegment>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start_secs: f64,
    pub end_secs: f64,
    pub text: String,
}

impl WhisperTranscriber {
    pub fn new() -> Self { Self }

    /// Transcribe a raw audio buffer locally.
    /// Placeholder — in production runs a local Whisper model.
    pub fn transcribe(_audio: &[f32]) -> String {
        // In production: run quantised Whisper.cpp model.
        // Return the transcribed text for journaling and reflection.
        String::new()
    }

    /// Transcribe with full metadata (segments, confidence).
    pub fn transcribe_detailed(audio: &[f32]) -> TranscriptionResult {
        let text = Self::transcribe(audio);
        TranscriptionResult {
            text,
            language: Some("en".into()),
            confidence: 0.92,
            segments: vec![],
            duration_ms: (audio.len() as f64 / 16000.0 * 1000.0) as u64,
        }
    }
}
