use std::path::{Path, PathBuf};

use lkjagent_core::render::Prompt;
use lkjagent_llm::client::complete;
use lkjagent_llm::message::Message;
use lkjagent_llm::wire::CallSpec;

use crate::config::load_client;
use crate::daemon::Endpoint;

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
    fn complete(&mut self, prompt: &Prompt, attempt: u32) -> Result<String, String> {
        let config = load_client(&self.data_dir)?;
        let max_tokens = u16::try_from(prompt.max_tokens).unwrap_or(u16::MAX);
        let spec = CallSpec::with_stop(max_tokens, &prompt.stop);
        let messages = vec![
            Message::system(prompt.system.clone()),
            Message::user(prompt.user.clone()),
        ];
        complete(&config, &messages, &spec, attempt)
            .map(|completion| completion.content)
            .map_err(|error| error.to_string())
    }
}
