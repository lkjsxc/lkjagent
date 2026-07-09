use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextPipelineStage {
    pub name: String,
    pub status: String,
    pub evidence: String,
}

impl ContextPipelineStage {
    fn new(name: &str, evidence: &str) -> Self {
        Self {
            name: name.to_string(),
            status: "applied".to_string(),
            evidence: evidence.to_string(),
        }
    }
}

pub fn default_context_pipeline() -> Vec<ContextPipelineStage> {
    vec![
        ContextPipelineStage::new("source-discovery", "durable context items"),
        ContextPipelineStage::new("scoring", "trust freshness cleanliness rank"),
        ContextPipelineStage::new("deduplication", "semantic body source fingerprint key"),
        ContextPipelineStage::new("contradiction-filtering", "unresolved conflict keys"),
        ContextPipelineStage::new("compression", "lane summaries and source refs"),
        ContextPipelineStage::new("prompt-assembly", "ordered prompt cards"),
        ContextPipelineStage::new("validation", "context frame fingerprint"),
    ]
}
