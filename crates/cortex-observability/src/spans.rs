pub struct SpanEmitter;

impl SpanEmitter {
    pub fn new() -> Self { Self }
    pub fn start_inference_span(&self, agent_id: &str) {
        tracing::info_span!("inference", agent = agent_id);
    }
    pub fn start_tool_call_span(&self, tool: &str) {
        tracing::info_span!("tool_call", tool = tool);
    }
}
