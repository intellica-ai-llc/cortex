use crate::{strategic_reasoner, analytical_reasoner, creative_reasoner, synthesiser};

pub struct ConvergeController {
    pub strategic: strategic_reasoner::StrategicReasoner,
    pub analytical: analytical_reasoner::AnalyticalReasoner,
    pub creative: creative_reasoner::CreativeReasoner,
    pub synthesiser: synthesiser::Synthesiser,
}

impl ConvergeController {
    pub fn new() -> Self {
        Self {
            strategic: strategic_reasoner::StrategicReasoner,
            analytical: analytical_reasoner::AnalyticalReasoner,
            creative: creative_reasoner::CreativeReasoner,
            synthesiser: synthesiser::Synthesiser,
        }
    }

    pub async fn converge(&self, question: &str) -> synthesiser::ConvergentResult {
        let s = self.strategic.reason(question);
        let a = self.analytical.reason(question);
        let c = self.creative.reason(question);
        self.synthesiser.synthesise(&s, &a, &c)
    }
}
