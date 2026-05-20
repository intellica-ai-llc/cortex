use serde::{Deserialize, Serialize};

/// Hierarchical Controller — ClawMobile pattern.
///
/// LFAB's S‑HAI core handles high‑level probabilistic reasoning and
/// planning, while the ClawMobile deterministic control layer executes
/// structured system interfaces (native UI parsing, system APIs, local
/// TraceDB queries). This separation ensures that critical operations
/// (e.g., field‑level writes) never pass through a probabilistic layer
/// without deterministic validation.
///
/// Based on LFM2.5-1.2B-Thinking (Liquid AI): "enabling on‑device agents
/// to orchestrate tools, extract data, and execute local workflows
/// without cloud compute." The model runs entirely on‑device, using
/// ~900 MB RAM, and supports a full agentic loop: perceive → reason →
/// act → observe.
pub struct HierarchicalController {
    /// Whether the on‑device model is loaded and ready.
    model_loaded: tokio::sync::RwLock<bool>,
    /// Tasks currently delegated to the cloud.
    cloud_delegations: tokio::sync::RwLock<Vec<CloudDelegation>>,
}

/// The two layers of the hierarchical architecture.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ControlLayer {
    /// LFAB S‑HAI Core — probabilistic reasoning, intent understanding.
    Strategic,
    /// ClawMobile deterministic control — structured execution.
    Deterministic,
}

/// A task delegated to the Cortex server for cloud processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDelegation {
    pub task_id: String,
    pub task_description: String,
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub status: DelegationStatus,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DelegationStatus { Pending, InProgress, Complete, Failed }

/// Decision about where to execute a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub execute_on_device: bool,
    pub control_layer: ControlLayer,
    pub reason: String,
    pub estimated_latency_ms: u64,
    pub requires_connectivity: bool,
}

impl HierarchicalController {
    pub fn new() -> Self {
        Self {
            model_loaded: tokio::sync::RwLock::new(false),
            cloud_delegations: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    /// Decide whether a task should execute on‑device or escalate to cloud.
    ///
    /// OpenPhone device‑cloud collaboration model: simple tasks (field
    /// lookup, tokenization, local search) run on‑device via LFAB.
    /// Complex tasks (cross‑system joins, deep research, report generation)
    /// escalate to the Cortex server.
    ///
    /// Decision criteria:
    ///   1. Task complexity (number of tool calls, data volume).
    ///   2. Model capability (is the on‑device model loaded?).
    ///   3. Connectivity (is the server reachable?).
    ///   4. Data sensitivity (PII must not leave the device).
    ///   5. Latency requirements (sub‑second must be local).
    pub async fn route_task(
        &self,
        task_description: &str,
        estimated_tool_calls: u32,
        contains_pii: bool,
        latency_requirement_ms: u64,
        server_reachable: bool,
    ) -> RoutingDecision {
        let model_ready = *self.model_loaded.read().await;

        // PII never leaves the device — always deterministic local.
        if contains_pii {
            return RoutingDecision {
                execute_on_device: true,
                control_layer: ControlLayer::Deterministic,
                reason: "PII data — must remain on device".into(),
                estimated_latency_ms: 5,
                requires_connectivity: false,
            };
        }

        // Sub‑second latency must be local.
        if latency_requirement_ms < 100 {
            return RoutingDecision {
                execute_on_device: true,
                control_layer: if model_ready { ControlLayer::Strategic }
                              else { ControlLayer::Deterministic },
                reason: format!("Latency requirement {}ms — must execute locally", latency_requirement_ms),
                estimated_latency_ms: if model_ready { 50 } else { 10 },
                requires_connectivity: false,
            };
        }

        // Complex tasks escalate to server when reachable.
        if estimated_tool_calls > 5 && server_reachable {
            return RoutingDecision {
                execute_on_device: false,
                control_layer: ControlLayer::Strategic,
                reason: format!("{} tool calls exceeds local threshold — escalate to server", estimated_tool_calls),
                estimated_latency_ms: 500,
                requires_connectivity: true,
            };
        }

        // Default: execute on‑device if model ready; otherwise escalate.
        if model_ready {
            RoutingDecision {
                execute_on_device: true,
                control_layer: ControlLayer::Strategic,
                reason: "On‑device model available — executing locally".into(),
                estimated_latency_ms: 80,
                requires_connectivity: false,
            }
        } else if server_reachable {
            RoutingDecision {
                execute_on_device: false,
                control_layer: ControlLayer::Strategic,
                reason: "On‑device model not loaded — escalating to server".into(),
                estimated_latency_ms: 400,
                requires_connectivity: true,
            }
        } else {
            RoutingDecision {
                execute_on_device: true,
                control_layer: ControlLayer::Deterministic,
                reason: "No model and no server — deterministic fallback".into(),
                estimated_latency_ms: 15,
                requires_connectivity: false,
            }
        }
    }

    /// Delegate a complex task to the server.
    pub async fn delegate_to_cloud(
        &self,
        task_description: &str,
    ) -> CloudDelegation {
        let delegation = CloudDelegation {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_description: task_description.to_string(),
            delegated_at: chrono::Utc::now(),
            status: DelegationStatus::Pending,
            reason: "Task complexity exceeded local capacity".into(),
        };
        self.cloud_delegations.write().await.push(delegation.clone());
        delegation
    }

    /// Mark the local model as loaded.
    pub async fn set_model_loaded(&self, loaded: bool) {
        *self.model_loaded.write().await = loaded;
    }

    /// Check if the on‑device model is ready.
    pub async fn is_model_loaded(&self) -> bool {
        *self.model_loaded.read().await
    }
}
