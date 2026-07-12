use super::{TEMPERATURE, TOP_P};

#[derive(Debug, Clone, PartialEq)]
pub struct CallSpec {
    pub max_tokens: u16,
    pub stop: Vec<String>,
    pub temperature: f32,
    pub top_p: f32,
    pub reasoning_effort: Option<String>,
}

impl CallSpec {
    pub fn action(max_tokens: u16) -> Self {
        Self {
            max_tokens,
            stop: Vec::new(),
            temperature: TEMPERATURE,
            top_p: TOP_P,
            reasoning_effort: None,
        }
    }

    pub fn with_stop(max_tokens: u16, closing_tag: &str) -> Self {
        Self {
            max_tokens,
            stop: vec![closing_tag.to_string()],
            temperature: TEMPERATURE,
            top_p: TOP_P,
            reasoning_effort: None,
        }
    }

    pub fn with_sampling(mut self, temperature: f32, top_p: f32) -> Self {
        self.temperature = temperature;
        self.top_p = top_p;
        self
    }

    pub fn with_reasoning_effort(mut self, effort: impl Into<String>) -> Self {
        let effort = effort.into();
        self.reasoning_effort = (!effort.is_empty() && effort != "none").then_some(effort);
        self
    }
}
