//! Voice Command Handler — Voice-to-Intent Pipeline
//!
//! Based on Glean Assistant (Feb 2026): real-time voice interaction,
//! speech-to-text → intent routing → dashboard response.
//! Also inspired by Chinese enterprise voice+text dual-mode systems
//! (Apr 2026) that support one-click voice input, automatic semantic
//! parsing, and multimodal data display.
//!
//! Cortex users speak naturally to the CrossSystemCommandBar, and the
//! voice handler converts speech to text, routes through the Semantic
//! Gateway, and returns results — all on-device, no cloud dependency.

pub struct VoiceCommandHandler {
    /// Whether voice input is enabled for this session.
    enabled: bool,
    /// The language code for speech recognition (default: "en-US").
    language: String,
    /// Minimum confidence threshold for speech recognition.
    min_confidence: f64,
}

/// The result of processing a voice command.
#[derive(Debug, Clone)]
pub struct VoiceCommandResult {
    /// The transcribed text.
    pub transcribed_text: String,
    /// Speech recognition confidence (0.0–1.0).
    pub speech_confidence: f64,
    /// The parsed intent from the transcribed text.
    pub parsed_intent: Option<String>,
    /// Whether the intent was successfully routed.
    pub routed: bool,
    /// Any error that occurred.
    pub error: Option<String>,
}

impl VoiceCommandHandler {
    pub fn new() -> Self {
        Self { enabled: false, language: "en-US".into(), min_confidence: 0.7 }
    }

    /// Enable voice command input.
    pub fn enable(&mut self) { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }
    pub fn is_enabled(&self) -> bool { self.enabled }

    /// Process a voice audio buffer and return a command result.
    ///
    /// In production: runs local Whisper model for STT, then passes
    /// transcribed text through the same Semantic Gateway as typed input.
    /// All processing on-device (privacy architecture).
    pub async fn process_voice(
        &self,
        _audio_data: &[f32],
    ) -> VoiceCommandResult {
        // In production: run local Whisper/Canary model.
        // The transcribed text then goes through intent_parser.
        VoiceCommandResult {
            transcribed_text: String::new(),
            speech_confidence: 0.0,
            parsed_intent: None,
            routed: false,
            error: Some("Voice processing not implemented".into()),
        }
    }

    /// Set the language for speech recognition.
    pub fn set_language(&mut self, lang: &str) { self.language = lang.to_string(); }
    pub fn language(&self) -> &str { &self.language }
}
