use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Longitudinal voice pattern discovery.
///
/// Detects recurring patterns in vocal biomarkers over days,
/// weeks, and months. Unlocks at Day 14: "Your Monday voice is
/// consistently lower‑energy than Friday." (Progressive Insight
/// Architecture, v5.)
pub struct PatternDetector {
    /// Per‑user pattern store.
    patterns: tokio::sync::RwLock<HashMap<String, Vec<VoicePattern>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicePattern {
    pub pattern_type: PatternType,
    pub description: String,
    pub confidence: f64,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    pub supporting_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// "Your Monday voice is consistently lower‑energy than Friday."
    DayOfWeekEffect,
    /// "On days you tagged 'exercised', your Pulse Score averages 12 points higher."
    ContextCorrelation,
    /// "Your vocal markers have shown elevated stress for 11 consecutive days."
    SustainedElevation,
    /// "Your speech rate has been steadily increasing over the past month."
    Trend,
    /// "This pattern matches your pre‑burnout profile from Q4 2025."
    HistoricalMatch,
}

impl PatternDetector {
    pub fn new() -> Self {
        Self { patterns: tokio::sync::RwLock::new(HashMap::new()) }
    }

    /// Analyse journaling history for patterns.
    /// Unlocked after 14 days of data (Progressive Insight Architecture).
    pub async fn detect_patterns(
        &self,
        user_id: &str,
        entries: &[super::whisper_agent::JournalEntry],
    ) -> Vec<VoicePattern> {
        if entries.len() < 14 { return vec![]; }

        let mut discovered = Vec::new();
        let mut existing = self.patterns.write().await;
        let user_patterns = existing.entry(user_id.to_string()).or_default();

        // Check day‑of‑week effect
        if let Some(pattern) = self.day_of_week_effect(entries) {
            discovered.push(pattern);
        }

        // Check for sustained elevation
        if let Some(pattern) = self.sustained_elevation(entries) {
            discovered.push(pattern);
        }

        user_patterns.extend(discovered.clone());
        discovered
    }

    fn day_of_week_effect(
        &self,
        entries: &[super::whisper_agent::JournalEntry],
    ) -> Option<VoicePattern> {
        // Simplified: compare Monday vs Friday energy.
        let monday: Vec<_> = entries.iter()
            .filter(|e| e.timestamp.format("%A").to_string() == "Monday")
            .collect();
        let friday: Vec<_> = entries.iter()
            .filter(|e| e.timestamp.format("%A").to_string() == "Friday")
            .collect();

        if monday.len() >= 2 && friday.len() >= 2 {
            let mon_energy: f64 = monday.iter()
                .map(|e| 1.0 - e.voice_features.fatigue_index)
                .sum::<f64>() / monday.len() as f64;
            let fri_energy: f64 = friday.iter()
                .map(|e| 1.0 - e.voice_features.fatigue_index)
                .sum::<f64>() / friday.len() as f64;

            if (fri_energy - mon_energy).abs() > 0.1 {
                return Some(VoicePattern {
                    pattern_type: PatternType::DayOfWeekEffect,
                    description: format!(
                        "Your Monday voice is consistently {}-energy than Friday.",
                        if mon_energy < fri_energy { "lower" } else { "higher" }
                    ),
                    confidence: 0.75,
                    discovered_at: chrono::Utc::now(),
                    supporting_evidence: vec![
                        format!("Monday avg energy: {:.2}", mon_energy),
                        format!("Friday avg energy: {:.2}", fri_energy),
                    ],
                });
            }
        }
        None
    }

    fn sustained_elevation(
        &self,
        entries: &[super::whisper_agent::JournalEntry],
    ) -> Option<VoicePattern> {
        // Check last 7 entries for consistent high stress.
        let recent: Vec<_> = entries.iter().rev().take(7).collect();
        if recent.len() < 7 { return None; }

        let elevated_count = recent.iter()
            .filter(|e| e.voice_features.stress_index > 0.6)
            .count();

        if elevated_count >= 7 {
            Some(VoicePattern {
                pattern_type: PatternType::SustainedElevation,
                description: format!(
                    "Your vocal markers have shown elevated stress for {} consecutive days.",
                    elevated_count
                ),
                confidence: 0.85,
                discovered_at: chrono::Utc::now(),
                supporting_evidence: vec![
                    format!("{} of {} days above stress threshold", elevated_count, recent.len()),
                ],
            })
        } else {
            None
        }
    }
}
