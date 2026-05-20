pub struct StrategicReasoner;
impl StrategicReasoner {
    pub fn reason(&self, question: &str) -> String {
        format!("Strategic analysis of: {}", question)
    }
}
