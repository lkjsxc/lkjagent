use lkjagent_core::prompt::Prompt;
use lkjagent_llm::client::complete;
use lkjagent_llm::message::Message;
use lkjagent_llm::wire::{CallSpec, Completion};
use std::path::{Path, PathBuf};

use crate::config::load_client;

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

#[derive(Debug, Clone)]
pub struct LlmEndpoint {
    data_dir: PathBuf,
}

impl LlmEndpoint {
    pub fn new(data_dir: &Path) -> Self {
        Self {
            data_dir: data_dir.to_path_buf(),
        }
    }
}

impl Endpoint for LlmEndpoint {
    fn complete(&mut self, prompt: &Prompt, attempt: u32) -> Result<CompletionRecord, String> {
        let config = load_client(&self.data_dir)?;
        let max_tokens = u16::try_from(prompt.max_tokens).unwrap_or(u16::MAX);
        let spec = if prompt.stop.is_empty() {
            CallSpec::action(max_tokens)
        } else {
            CallSpec::with_stop(max_tokens, &prompt.stop)
        }
        .with_reasoning_effort("none");
        let messages = vec![
            Message::system(prompt.system.clone()),
            Message::user(prompt.user.clone()),
        ];
        complete(&config, &messages, &spec, attempt)
            .map(record)
            .map_err(|error| error.to_string())
    }

    fn timeout_seconds(&self) -> Option<u64> {
        load_client(&self.data_dir)
            .ok()
            .map(|config| config.timeout.as_secs())
    }
}

fn record(completion: Completion) -> CompletionRecord {
    CompletionRecord {
        content: completion.content,
        prompt_tokens: u32_opt(completion.usage.prompt_tokens),
        completion_tokens: u32_opt(completion.usage.completion_tokens),
        cached_tokens: u32_opt(completion.usage.cached_prompt_tokens),
        finish_reason: format!("{:?}", completion.finish_reason),
        closure_mode: completion.closure_mode.as_str().to_string(),
        cache_metrics: completion
            .cache_metrics
            .into_iter()
            .map(|metric| (metric.name, metric.value))
            .collect(),
        anomaly: completion
            .provider_anomaly
            .map(|anomaly| format!("{}:{}", anomaly.kind.as_str(), anomaly.detail)),
    }
}

fn u32_opt(value: Option<u64>) -> Option<u32> {
    value.map(|value| value.min(u32::MAX as u64) as u32)
}
