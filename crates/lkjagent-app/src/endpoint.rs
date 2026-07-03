use std::path::{Path, PathBuf};
use std::time::Duration;

use lkjagent_core::render::Prompt;
use lkjagent_llm::client::complete;
use lkjagent_llm::message::Message;
use lkjagent_llm::wire::{CallSpec, Completion};

use crate::config::load_client;
use crate::model_io::{CompletionRecord, Endpoint};

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
        let spec = CallSpec::with_stop(max_tokens, &prompt.stop);
        let messages = vec![
            Message::system(prompt.system.clone()),
            Message::user(prompt.user.clone()),
        ];
        let mut last_error = String::new();
        for offset in 0..5 {
            match complete(&config, &messages, &spec, attempt.saturating_add(offset)) {
                Ok(completion) => return Ok(record(completion)),
                Err(error) => {
                    last_error = error.to_string();
                    std::thread::sleep(Duration::from_secs(3));
                }
            }
        }
        Err(last_error)
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
