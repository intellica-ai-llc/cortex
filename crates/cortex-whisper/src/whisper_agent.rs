use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Primary voice journaling orchestrator.
///
/// Implements KRIYA's co‑interpretive engagement model (Zhu et al.,
/// CHI 2026): Comfort Zone, Detective Mode, and What-If Planning
/// are the three interaction modes. Users explore their data with
/// curiosity rather than being judged by performance metrics.
pub struct WhisperAgent {
    /// Per‑user journaling history for pattern tracking.
    history: tokio::sync::RwLock<HashMap<String, Vec<JournalEntry>>>,
}

/// A single voice journal entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: String,
    pub user_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transcript: String,
    pub voice_features: crate::voice_capture::CapturedVoiceSample,
    pub pulse_score: Option<f64>,
    pub reflection: Option<String>,
    pub mode: JournalMode,
}

/// KRIYA co‑interpretive engagement mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JournalMode {
    /// Safe, reassuring data presentation — no judgment.
    ComfortZone,
    /// User actively investigates patterns and correlations.
    DetectiveMode,
    /// Projects future scenarios based on behavioural changes.
    WhatIfPlanning,
    /// Quick daily check‑in, minimal interaction.
    QuickCheckIn,
}

/// The morning journaling flow: greet → capture → analyse → reflect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningRoutine {
    pub greeting: String,
    pub capture_duration_secs: u32,     // 15‑45 seconds
    pub voice_result: Option<crate::voice_capture::CapturedVoiceSample>,
    pub reflection: Option<String>,
    pub pulse_score_update: Option<f64>,
}

impl WhisperAgent {
    pub fn new() -> Self {
        Self { history: tokio::sync::RwLock::new(HashMap::new()) }
    }

    /// Run the morning voice check‑in ritual.
    ///
    /// thymia extracts 30+ biomarkers from 15 seconds of speech;
    /// Canary Speech uses 45 seconds for a broader wellness screen.
    /// Cortex Whisper defaults to 30 seconds for the daily check‑in.
    pub async fn morning_check_in(
        &self,
        user_id: &str,
        audio: &[f32],
    ) -> MorningRoutine {
        let capture = crate::voice_capture::VoiceCaptureEngine::process(audio);
        let transcript = crate::transcriber::WhisperTranscriber::transcribe(audio);

        // Build a KRIYA‑style reflection.
        // "Your voice sounds a bit fatigued today — your speech rate
        //  is 15% slower than your baseline. Combined with your
        //  elevated pallor score, it might be worth taking a short
        //  break."
        let reflection = if capture.fatigue_index > 0.6 {
            Some(format!(
                "I hear you. Your voice sounds a bit fatigued today — \
                 your speech rate is {:.0}% slower than your baseline. \
                 Want me to suggest some ways to recharge?",
                capture.fatigue_index * 100.0
            ))
        } else {
            Some("Good morning! Your voice sounds energetic today. Ready to take on the day?".into())
        };

        let entry = JournalEntry {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
            transcript,
            voice_features: capture.clone(),
            pulse_score: None,
            reflection: reflection.clone(),
            mode: JournalMode::QuickCheckIn,
        };

        self.history.write().await
            .entry(user_id.to_string())
            .or_default()
            .push(entry);

        MorningRoutine {
            greeting: "Good morning. How are you feeling today?".into(),
            capture_duration_secs: 30,
            voice_result: Some(capture),
            reflection,
            pulse_score_update: None,
        }
    }

    /// Get journaling history for a user.
    pub async fn get_history(&self, user_id: &str) -> Vec<JournalEntry> {
        self.history.read().await.get(user_id).cloned().unwrap_or_default()
    }
}
