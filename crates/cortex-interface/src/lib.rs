//! Cortex InterfaceEngine — the Interface of One.
//!
//! Updated for v12: includes the invisibility wrapper (Strangler Fig
//! façade, dual‑write propagator, activity camouflage) and the
//! adoption bridge sequencer for crossing Moore’s Chasm.

pub mod personalized_dashboard;
pub mod cross_system_bar;
pub mod widget_generator;
pub mod notification_manager;
pub mod weaning_engine;
pub mod observational_capture;

// batch 6b modules
pub mod native_ui_accessibility;
pub mod native_ui_ocr;
pub mod native_ui_terminal;
pub mod cross_device_sync;
pub mod adaptive_ui_renderer;
pub mod agui_adapter;
pub mod a2ui_adapter;
pub mod component_catalog;

// batch 6c modules (invisibility wrapper & adoption)
pub mod facade;
pub mod dual_write_propagator;
pub mod activity_camouflage;
pub mod adoption_bridge;
pub mod role_dashboard;
pub mod morning_brief;
pub mod absorption_engine;
pub mod command_center;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top‑level interface orchestrator.
pub struct InterfaceEngine {
    pub dashboard: personalized_dashboard::PersonalizedDashboard,
    pub command_bar: cross_system_bar::CrossSystemCommandBar,
    pub widget_gen: widget_generator::WidgetGenerator,
    pub notifier: notification_manager::NotificationManager,
    pub weaning: weaning_engine::WeaningEngine,
    pub obs_capture: observational_capture::ObservationalCapture,

    // batch 6b – adaptive UI
    pub cross_device: cross_device_sync::CrossDeviceSessionManager,
    pub renderer: adaptive_ui_renderer::AdaptiveUIRenderer,
    pub agui: agui_adapter::AGUIAdapter,
    pub a2ui: a2ui_adapter::A2UIAdapter,
    pub component_catalog: component_catalog::ComponentCatalog,

    // batch 6c – invisibility + adoption
    pub facade: facade::StranglerFigFacade,
    pub dual_write: dual_write_propagator::DualWritePropagator,
    pub camouflage: activity_camouflage::ActivityCamouflageController,
    pub adoption_bridge: adoption_bridge::AdoptionBridgeSequencer,
    pub role_dashboard: role_dashboard::RoleAdaptiveDashboard,
    pub morning_brief: morning_brief::MorningBrief,
    pub absorption_engine: absorption_engine::AbsorptionEngine,
    pub command_center: command_center::AgenticCommandCenter,
}

impl InterfaceEngine {
    pub fn new() -> Self {
        Self {
            dashboard: personalized_dashboard::PersonalizedDashboard::new(),
            command_bar: cross_system_bar::CrossSystemCommandBar::new(),
            widget_gen: widget_generator::WidgetGenerator::new(),
            notifier: notification_manager::NotificationManager::new(),
            weaning: weaning_engine::WeaningEngine::new(),
            obs_capture: observational_capture::ObservationalCapture::new(),

            cross_device: cross_device_sync::CrossDeviceSessionManager::new(),
            renderer: adaptive_ui_renderer::AdaptiveUIRenderer::new(),
            agui: agui_adapter::AGUIAdapter::new(),
            a2ui: a2ui_adapter::A2UIAdapter::new(),
            component_catalog: component_catalog::ComponentCatalog::new(),

            facade: facade::StranglerFigFacade::new(),
            dual_write: dual_write_propagator::DualWritePropagator::new(),
            camouflage: activity_camouflage::ActivityCamouflageController::new(),
            adoption_bridge: adoption_bridge::AdoptionBridgeSequencer::new(),
            role_dashboard: role_dashboard::RoleAdaptiveDashboard::new(),
            morning_brief: morning_brief::MorningBrief::new(),
            absorption_engine: absorption_engine::AbsorptionEngine::new(),
            command_center: command_center::AgenticCommandCenter::new(),
        }
    }
}
