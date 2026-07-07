use lkjagent_core::render::Prompt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRecord {
    pub content: String,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub cached_tokens: Option<u32>,
    pub finish_reason: String,
    pub closure_mode: String,
    pub cache_metrics: Vec<(String, String)>,
    pub anomaly: Option<String>,
}

impl CompletionRecord {
    pub fn scripted(content: String) -> Self {
        Self {
            content,
            prompt_tokens: None,
            completion_tokens: None,
            cached_tokens: None,
            finish_reason: "scripted".to_string(),
            closure_mode: "unknown".to_string(),
            cache_metrics: Vec::new(),
            anomaly: None,
        }
    }
}

pub trait Endpoint {
    fn complete(&mut self, prompt: &Prompt, attempt: u32) -> Result<CompletionRecord, String>;

    fn timeout_seconds(&self) -> Option<u64> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ScriptedEndpoint {
    pub outputs: Vec<String>,
    pub index: usize,
}

impl Endpoint for ScriptedEndpoint {
    fn complete(&mut self, prompt: &Prompt, _attempt: u32) -> Result<CompletionRecord, String> {
        let Some(output) = self.outputs.get(self.index).cloned() else {
            return Err("scripted endpoint exhausted".to_string());
        };
        self.index = self.index.saturating_add(1);
        Ok(CompletionRecord::scripted(fill_prompt_markers(
            &output, prompt,
        )))
    }
}

fn fill_prompt_markers(output: &str, prompt: &Prompt) -> String {
    output
        .replace("__DECISION_ID__", &prompt_value(prompt, "- decision_id: "))
        .replace(
            "__CONTEXT_FRAME_FINGERPRINT__",
            &prompt_value(prompt, "- context_fingerprint: "),
        )
}

fn prompt_value(prompt: &Prompt, prefix: &str) -> String {
    prompt
        .user
        .lines()
        .chain(prompt.system.lines())
        .find_map(|line| line.trim().strip_prefix(prefix).map(str::to_string))
        .unwrap_or_default()
}
