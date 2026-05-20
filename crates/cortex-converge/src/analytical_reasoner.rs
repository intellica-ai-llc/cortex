pub struct AnalyticalReasoner;
impl AnalyticalReasoner {
    pub fn reason(&self, question: &str) -> String {
        format!("Analytical evidence on: {}", question)
    }
}
