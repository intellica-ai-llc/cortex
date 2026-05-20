pub struct CreativeReasoner;
impl CreativeReasoner {
    pub fn reason(&self, question: &str) -> String {
        format!("Creative edge cases for: {}", question)
    }
}
