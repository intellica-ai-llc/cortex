//! Cortex Adaptive Onboarding Engine (v3).
//!
//! Industry‑intelligent, role‑based onboarding paths. Based on Google
//! Gemini Enterprise onboarding agents (Feb 2026) and Enboarder's
//! role‑aware AI assistants (Feb 2026). Generates a unique onboarding
//! path for every role, preloading industry‑specific knowledge graphs,
//! regulatory calendars, and personalised dashboards on day one.

pub mod industry_router;
pub mod role_path_builder;
pub mod adaptive_checklist;
pub mod first_day_brief;

use std::sync::Arc;

pub struct AdaptiveOnboardingEngine {
    pub industry_router: Arc<industry_router::IndustryRouter>,
    pub role_path_builder: Arc<role_path_builder::RolePathBuilder>,
    pub checklist: Arc<adaptive_checklist::AdaptiveChecklist>,
    pub first_day: Arc<first_day_brief::FirstDayBrief>,
}

impl AdaptiveOnboardingEngine {
    pub fn new() -> Self {
        Self {
            industry_router: Arc::new(industry_router::IndustryRouter::new()),
            role_path_builder: Arc::new(role_path_builder::RolePathBuilder::new()),
            checklist: Arc::new(adaptive_checklist::AdaptiveChecklist::new()),
            first_day: Arc::new(first_day_brief::FirstDayBrief::new()),
        }
    }
}
