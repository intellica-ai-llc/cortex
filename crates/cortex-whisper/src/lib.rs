//! Cortex Whisper™ — Voice Journaling & Vocal Biomarker Agent (v5).
//!
//! Based on thymia (30+ health signals from 15s speech) and Canary
//! Speech (45-second check-in). Uses KRIYA's co‑interpretive
//! engagement model: users explore their data with curiosity rather
//! than being judged by it. All processing is on‑device; only
//! extracted feature vectors (12‑20 floats) are stored.
//!
//! Subsystems:
//!   whisper_agent        — orchestrator, journaling loop
//!   voice_capture        — 15‑45 second speech capture
//!   transcriber          — local transcription (no cloud)
//!   journaling_reflector — KRIYA co‑interpretive engagement
//!   pattern_detector     — longitudinal voice pattern discovery
//!   passive_monitor      — background analysis during meetings

pub mod whisper_agent;
pub mod voice_capture;
pub mod transcriber;
pub mod journaling_reflector;
pub mod pattern_detector;
pub mod passive_monitor;

use std::sync::Arc;

pub struct CortexWhisper {
    pub agent: Arc<whisper_agent::WhisperAgent>,
    pub capture: Arc<voice_capture::VoiceCaptureEngine>,
    pub transcriber: Arc<transcriber::WhisperTranscriber>,
    pub reflector: Arc<journaling_reflector::JournalingReflector>,
    pub pattern_detector: Arc<pattern_detector::PatternDetector>,
    pub passive_monitor: Arc<passive_monitor::PassiveMonitor>,
}

impl CortexWhisper {
    pub fn new() -> Self {
        Self {
            agent: Arc::new(whisper_agent::WhisperAgent::new()),
            capture: Arc::new(voice_capture::VoiceCaptureEngine::new()),
            transcriber: Arc::new(transcriber::WhisperTranscriber::new()),
            reflector: Arc::new(journaling_reflector::JournalingReflector::new()),
            pattern_detector: Arc::new(pattern_detector::PatternDetector::new()),
            passive_monitor: Arc::new(passive_monitor::PassiveMonitor::new()),
        }
    }
}
