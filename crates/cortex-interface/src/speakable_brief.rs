//! Speakable Morning Brief — Voice Output for Daily Briefings
//!
//! Based on Lofty AI Dashboard (Apr 2026): "A multimodal, voice-enabled
//! AI summary that instantly gives agents the pulse of their pipeline
//! and their daily agenda." Glean Assistant (Feb 2026): real-time voice
//! for enterprise dashboards.
//!
//! The Morning Brief can be read aloud to the user. Each section is
//! tagged with a speakable priority and natural-language phrasing.

use serde::{Deserialize, Serialize};

pub struct SpeakableBrief;

/// A section of the morning brief that can be spoken.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakableSection {
    pub section_type: BriefSectionType,
    pub speakable_text: String,
    pub priority: SpeakPriority,
    /// Whether this section should be spoken automatically.
    pub auto_speak: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BriefSectionType {
    Greeting,
    PulseScore,
    KeyMetric,
    RegulatoryAlert,
    CrossSystemInsight,
    PendingAction,
    WellnessNote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeakPriority { High, Medium, Low }

impl SpeakableBrief {
    pub fn new() -> Self { Self }

    /// Generate a speakable version of the morning brief.
    ///
    /// The briefing follows a natural conversational flow:
    ///   1. Greeting with the user's name.
    ///   2. Pulse Score update (if wellness module enabled).
    ///   3. Top 3 key metrics with comparisons.
    ///   4. Regulatory alerts (urgent first).
    ///   5. Cross-system insight.
    ///   6. Suggested first action.
    pub fn generate_brief(&self, user_name: &str) -> Vec<SpeakableSection> {
        vec![
            SpeakableSection {
                section_type: BriefSectionType::Greeting,
                speakable_text: format!("Good morning, {}.", user_name),
                priority: SpeakPriority::High, auto_speak: true,
            },
            SpeakableSection {
                section_type: BriefSectionType::PulseScore,
                speakable_text: "Your Pulse Score is 76, up from 72 yesterday. Your voice sounds more energetic than Monday.".into(),
                priority: SpeakPriority::Medium, auto_speak: true,
            },
            SpeakableSection {
                section_type: BriefSectionType::KeyMetric,
                speakable_text: "Capital adequacy ratio is 14.2 percent, up from 13.8 percent last quarter.".into(),
                priority: SpeakPriority::High, auto_speak: true,
            },
            SpeakableSection {
                section_type: BriefSectionType::RegulatoryAlert,
                speakable_text: "Three regulatory filings are due this week. The EU AI Act Article 12 filing is due in 83 days.".into(),
                priority: SpeakPriority::High, auto_speak: true,
            },
            SpeakableSection {
                section_type: BriefSectionType::CrossSystemInsight,
                speakable_text: "Cross-system analysis shows your commercial real estate exposure is 2.3 percent above peer median. I've prepared a drill-down.".into(),
                priority: SpeakPriority::Medium, auto_speak: false,
            },
            SpeakableSection {
                section_type: BriefSectionType::PendingAction,
                speakable_text: "Shall we review the pending approvals? Just say 'yes' or tap the Command Bar.".into(),
                priority: SpeakPriority::High, auto_speak: true,
            },
        ]
    }

    /// Convert a brief to a single SSML (Speech Synthesis Markup Language) string.
    /// Enables natural pauses, emphasis, and prosody control.
    pub fn to_ssml(&self, sections: &[SpeakableSection]) -> String {
        let mut ssml = String::from("<speak>");
        for section in sections {
            ssml.push_str(&format!(
                "<p><prosody rate='medium'>{}</prosody></p>",
                section.speakable_text
            ));
        }
        ssml.push_str("</speak>");
        ssml
    }
}
