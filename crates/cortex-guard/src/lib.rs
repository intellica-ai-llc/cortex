//! CortexGuard — cryptographic kill switch for enterprise AI agents.
//!
//! Three-factor, offline-capable, dead-man's switch.
//! Based on the JumpCloud finding that 55% of organisations lack
//! any centralised kill switch (May 5, 2026).

pub mod kill_switch;
pub mod behavioral_baseline;
pub mod heartbeat_monitor;
pub mod forensic_mode;
pub mod recovery_workflow;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CortexGuard {
    pub kill_switch: kill_switch::KillSwitch,
    pub baseline: behavioral_baseline::BehavioralBaseline,
    pub heartbeat: heartbeat_monitor::HeartbeatMonitor,
    pub forensic: forensic_mode::ForensicMode,
    pub recovery: recovery_workflow::RecoveryWorkflow,
    pub state: RwLock<GuardState>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuardState {
    Normal,
    Throttled,
    SafePark,
    Frozen,
    Forensic,
}

impl CortexGuard {
    pub fn new() -> Self {
        Self {
            kill_switch: kill_switch::KillSwitch::new(),
            baseline: behavioral_baseline::BehavioralBaseline::new(),
            heartbeat: heartbeat_monitor::HeartbeatMonitor::new(),
            forensic: forensic_mode::ForensicMode::new(),
            recovery: recovery_workflow::RecoveryWorkflow::new(),
            state: RwLock::new(GuardState::Normal),
        }
    }

    /// Activate the kill switch and enter forensic mode.
    pub async fn activate(&self, trigger: KillTrigger) {
        let mut state = self.state.write().await;
        *state = GuardState::Frozen;

        tracing::error!(
            trigger = ?trigger,
            "CortexGuard kill switch activated — all agents frozen"
        );

        self.forensic.capture_snapshot().await;
    }

    /// Check all three factors. Returns true if any factor triggers.
    pub async fn evaluate(&self) -> GuardState {
        // Factor 1: Token presence
        if !self.kill_switch.is_token_present().await {
            return GuardState::Frozen;
        }

        // Factor 2: Behavioural baseline
        if self.baseline.is_deviating().await {
            return GuardState::Throttled;
        }

        // Factor 3: Heartbeat
        if !self.heartbeat.is_alive().await {
            return GuardState::SafePark;
        }

        GuardState::Normal
    }
}

#[derive(Debug)]
pub enum KillTrigger {
    TokenRemoved,
    BehavioralDeviation { sigma: f64 },
    HeartbeatLost { seconds: u64 },
    ManualActivation { by: String },
}
