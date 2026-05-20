use serde::{Deserialize, Serialize};

/// Journaling Reflector — KRIYA co‑interpretive engagement model.
///
/// KRIYA (Zhu et al., CHI 2026, arXiv:2601.14589) found that users
/// "framed engaging with wellbeing data as interpretation rather
/// than performance, experienced reflection as supportive or
/// pressuring depending on emotional framing, and developed trust
/// through transparency."
///
/// The reflector never shames. Missing a day triggers:
/// "No worries — life happens. Want to do a quick voice check‑in
///  now? It takes 15 seconds."
pub struct JournalingReflector;

/// The three KRIYA interaction modes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReflectionMode {
    /// "Your Pulse Score is 82 — in the top 15% for your age group."
    ComfortZone,
    /// "Let's investigate: your Monday voice is consistently lower‑energy than Friday."
    DetectiveMode,
    /// "If you maintain your current sleep and exercise pattern, your Pulse Score projects to reach 82 by next month."
    WhatIfPlanning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reflection {
    pub mode: ReflectionMode,
    pub message: String,
    pub tone: ReflectionTone,
    pub actionable_suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReflectionTone {
    Supportive,
    Curious,
    Encouraging,
    Gentle,
}

impl JournalingReflector {
    pub fn new() -> Self { Self }

    /// Generate a reflection based on voice features and mode.
    ///
    /// KRIYA principle: "supportive or pressuring depending on
    /// emotional framing." If the user's voice shows elevated
    /// stress, the reflector frames it as an observation, not
    /// a judgment: "Your voice sounds a bit fatigued today"
    /// rather than "Your stress score is high."
    pub fn reflect(
        &self,
        features: &crate::voice_capture::CapturedVoiceSample,
        mode: ReflectionMode,
    ) -> Reflection {
        let (message, tone, suggestion) = match mode {
            ReflectionMode::ComfortZone => {
                let pulse_estimate = 100.0 - (features.stress_index + features.fatigue_index) * 50.0;
                (
                    format!(
                        "Your voice suggests you're well‑rested today. \
                         Estimated Pulse Score is {:.0}/100.",
                        pulse_estimate.max(0.0).min(100.0)
                    ),
                    ReflectionTone::Encouraging,
                    Some("Want to do a quick eye scan to complete your full Pulse Score?".into()),
                )
            }
            ReflectionMode::DetectiveMode => {
                (
                    "I notice your speech rate is slower than your baseline. \
                     Would you like to explore what patterns might be contributing?".into(),
                    ReflectionTone::Curious,
                    Some("Let's look at your sleep, exercise, and stress tags over the past week.".into()),
                )
            }
            ReflectionMode::WhatIfPlanning => {
                (
                    "If you maintain your current sleep and exercise pattern, \
                     your Pulse Score projects to improve by 8 points next month.".into(),
                    ReflectionTone::Supportive,
                    Some("Shall we set a gentle goal for this week?".into()),
                )
            }
        };

        Reflection { mode, message, tone, actionable_suggestion: suggestion }
    }

    /// Generate a gentle nudge for a missed day.
    /// "No worries — life happens."
    pub fn missed_day_nudge() -> String {
        "No worries — life happens. Want to do a quick voice check‑in now? It takes 15 seconds.".into()
    }

    /// Generate a streak celebration.
    pub fn streak_celebration(days: u32) -> String {
        format!(
            "You've checked in for {} days straight! That's building irreplaceable health \
             data — your baselines are getting more accurate every day.",
            days
        )
    }
}
