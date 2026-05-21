pub struct EmbeddingRouter;

impl EmbeddingRouter {
    pub fn new() -> Self { Self }

    pub fn embed(&self, text: &str) -> Vec<f32> {
        let common: Vec<&str> = vec![
            "show","me","the","work","order","orders","asset","assets","employee","cost","budget","report",
            "open","closed","pending","compare","create","update","delete","alert","notify",
            "status","priority","location","vendor","schedule","maintenance","inspection","repair",
            "failure","downtime","revenue","expense","profit","loss","q1","q2","q3","q4",
            "performance","compliance","audit","safety","risk","kpi","metric","benchmark",
            "peer","industry","regulation","nerc","ferc","epa","osha","iso","hipaa","sox",
            "capital","expenditure","customer","client","patient","provider","payer",
            "claim","policy","premium","medication","diagnosis","procedure",
        ];
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut vec = vec![0.0f32; 128];
        for w in &words {
            let w = w.to_lowercase();
            if let Some(i) = common.iter().position(|&c| c == w.as_str()) {
                vec[i] += 1.0;
            } else {
                let h = w.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
                vec[(h % 128) as usize] += 0.1;
            }
        }
        let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.0 { vec.iter_mut().for_each(|v| *v /= norm); }
        vec
    }
}
