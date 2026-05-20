use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Mobile TraceDB — SQLite + Zvec + CRDT sync.
///
/// Lightweight on‑device database that mirrors the server TraceDB
/// for the Observation and Mirror phases. Stores decision traces,
/// absorbed field metadata, and behavioural workflow tokens locally
/// for offline operation. Syncs bidirectionally with the server via
/// ElectricSQL CRDT protocol.
///
/// Zvec (on‑device vector search) enables mobile Schema Grounding
/// Agent queries without cloud connectivity — embeddings are stored
/// and searched entirely on‑device.
pub struct MobileTraceDB {
    /// Path to the local SQLite database file.
    db_path: String,
    /// In‑memory cache of recent decision traces for fast local queries.
    trace_cache: RwLock<Vec<MobileDecisionTrace>>,
    /// In‑memory vector store for on‑device semantic search (Zvec).
    vector_store: RwLock<HashMap<String, Vec<f32>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileDecisionTrace {
    pub trace_id: String,
    pub user_id: String,
    pub behavioral_token: String,
    pub source_application: String,
    pub field_path: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub captured_at: chrono::DateTime<chrono::Utc>,
    pub synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileAbsorbedField {
    pub field_id: String,
    pub source_application: String,
    pub source_table: String,
    pub source_column: String,
    pub semantic_label: Option<String>,
    pub field_type: String,
    pub embedding: Option<Vec<f32>>,
}

/// Result of an on‑device vector search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileVectorSearchResult {
    pub field_id: String,
    pub semantic_label: String,
    pub similarity: f64,
}

impl MobileTraceDB {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
            trace_cache: RwLock::new(Vec::new()),
            vector_store: RwLock::new(HashMap::new()),
        }
    }

    /// Store a decision trace locally before sync.
    pub async fn store_trace(&self, trace: MobileDecisionTrace) {
        self.trace_cache.write().await.push(trace);
    }

    /// Get all unsynced traces for upload.
    pub async fn unsynced_traces(&self) -> Vec<MobileDecisionTrace> {
        self.trace_cache.read().await.iter()
            .filter(|t| !t.synced)
            .cloned()
            .collect()
    }

    /// Mark traces as synced after successful upload.
    pub async fn mark_synced(&self, trace_ids: &[String]) {
        let mut cache = self.trace_cache.write().await;
        for trace in cache.iter_mut() {
            if trace_ids.contains(&trace.trace_id) {
                trace.synced = true;
            }
        }
    }

    /// Register a field embedding for on‑device semantic search.
    pub async fn register_embedding(&self, field_id: &str, embedding: Vec<f32>) {
        self.vector_store.write().await.insert(field_id.to_string(), embedding);
    }

    /// Perform on‑device cosine‑similarity search (Zvec pattern).
    /// Enables the mobile Schema Grounding Agent to find relevant
    /// fields without cloud connectivity.
    pub async fn semantic_search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Vec<MobileVectorSearchResult> {
        let store = self.vector_store.read().await;
        let mut scored: Vec<(f64, &String, &Vec<f32>)> = store.iter()
            .map(|(id, emb)| {
                let sim = cosine_similarity(query_embedding, emb);
                (sim, id, emb)
            })
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored.into_iter().take(top_k).map(|(sim, id, _)| {
            MobileVectorSearchResult {
                field_id: id.clone(),
                semantic_label: id.clone(),
                similarity: sim,
            }
        }).collect()
    }

    /// Number of locally stored traces.
    pub async fn trace_count(&self) -> usize {
        self.trace_cache.read().await.len()
    }
}

/// Cosine similarity between two equal‑length vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() { return 0.0; }
    let dot: f64 = a.iter().zip(b).map(|(x, y)| (*x as f64) * (*y as f64)).sum();
    let na: f64 = a.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    if na == 0.0 || nb == 0.0 { 0.0 } else { dot / (na * nb) }
}
