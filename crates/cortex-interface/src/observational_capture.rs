use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Observational Capture Engine (v2/v3/v8).
///
/// Records user interactions with legacy applications via browser
/// extension, accessibility API, OCR, and terminal emulation.
/// Converts observed workflows into reusable agent skills.
pub struct ObservationalCapture {
    sessions: RwLock<Vec<CaptureSession>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureSession {
    pub session_id: String,
    pub user_id: String,
    pub application: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub events: Vec<CaptureEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureEvent {
    pub event_type: CaptureEventType,
    pub field_id: Option<String>,
    pub value: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptureEventType {
    PageNavigation,
    FieldFocus,
    FieldChange,
    FormSubmit,
    ButtonClick,
    ErrorDisplayed,
}

impl ObservationalCapture {
    pub fn new() -> Self {
        Self { sessions: RwLock::new(Vec::new()) }
    }

    /// Start a new capture session for a user.
    pub async fn start_session(&self, user_id: &str, application: &str) -> CaptureSession {
        let session = CaptureSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            application: application.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            events: Vec::new(),
        };
        self.sessions.write().await.push(session.clone());
        session
    }

    /// Record an event within an active session.
    pub async fn record_event(&self, session_id: &str, event: CaptureEvent) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|s| s.session_id == session_id) {
            session.events.push(event);
        }
    }

    /// End a capture session.
    pub async fn end_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|s| s.session_id == session_id) {
            session.end_time = Some(chrono::Utc::now());
        }
    }

    /// Convert an observed workflow into a skill draft (for Forge).
    pub async fn convert_to_skill_draft(&self, session_id: &str) -> Option<SkillDraft> {
        let sessions = self.sessions.read().await;
        let session = sessions.iter().find(|s| s.session_id == session_id)?;
        if session.events.len() < 3 { return None; }

        let tokens: Vec<String> = session.events.iter().map(|e| format!("{:?}", e.event_type)).collect();
        Some(SkillDraft {
            source_session: session_id.to_string(),
            tokens,
            confidence: 0.8,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDraft {
    pub source_session: String,
    pub tokens: Vec<String>,
    pub confidence: f64,
}
