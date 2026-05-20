use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Background vocal analysis during meetings (consent‑gated).
///
/// When the user speaks during video calls, Cortex Whisper analyses
/// vocal biomarkers in the background. No audio leaves the device.
/// Only the extracted feature vector (12‑20 floats) is stored.
/// Users explicitly opt in; off by default.
///
/// Japanese startups can detect stress from just 3 seconds of
/// speech; ten seconds is sufficient for a meaningful signal with
/// higher confidence.
pub struct PassiveMonitor {
    /// Users who have explicitly opted in to passive monitoring.
    opted_in_users: tokio::sync::RwLock<HashSet<String>>,
    /// Minimum duration (seconds) of continuous speech to trigger analysis.
    min_speech_duration_secs: f64,
    /// Cooldown period between passive analyses.
    cooldown_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveSession {
    pub user_id: String,
    pub session_id: String,
    pub meeting_title: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub speech_segments: Vec<SpeechSegment>,
    pub aggregated_features: Option<crate::voice_capture::CapturedVoiceSample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechSegment {
    pub offset_secs: f64,
    pub duration_secs: f64,
    pub features: crate::voice_capture::CapturedVoiceSample,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveAnalysisResult {
    pub user_id: String,
    pub meeting_id: String,
    pub feature_vector: crate::voice_capture::CapturedVoiceSample,
    pub stress_trend: Option<String>,   // "rising", "stable", "declining"
    pub alert: Option<String>,
}

impl PassiveMonitor {
    pub fn new() -> Self {
        Self {
            opted_in_users: tokio::sync::RwLock::new(HashSet::new()),
            min_speech_duration_secs: 3.0,
            cooldown_seconds: 300, // 5 minutes
        }
    }

    /// Opt a user into passive monitoring.
    pub async fn opt_in(&self, user_id: &str) {
        self.opted_in_users.write().await.insert(user_id.to_string());
    }

    /// Opt a user out of passive monitoring.
    pub async fn opt_out(&self, user_id: &str) {
        self.opted_in_users.write().await.remove(user_id);
    }

    /// Check if a user has consented to passive monitoring.
    pub async fn is_opted_in(&self, user_id: &str) -> bool {
        self.opted_in_users.read().await.contains(user_id)
    }

    /// Process a speech segment detected during a meeting.
    /// Only processes if user is opted in and segment meets minimum
    /// duration. Returns None if skipped.
    pub async fn process_segment(
        &self,
        user_id: &str,
        audio: &[f32],
    ) -> Option<PassiveAnalysisResult> {
        if !self.is_opted_in(user_id).await {
            return None;
        }

        let duration = audio.len() as f64 / 16000.0;
        if duration < self.min_speech_duration_secs {
            return None;
        }

        let features = crate::voice_capture::VoiceCaptureEngine::process_passive(audio);
        Some(PassiveAnalysisResult {
            user_id: user_id.to_string(),
            meeting_id: String::new(),
            feature_vector: features,
            stress_trend: None,
            alert: None,
        })
    }
}
