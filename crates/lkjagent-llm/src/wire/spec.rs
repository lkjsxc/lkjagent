use super::{TEMPERATURE, TOP_P};

const ACTION_CLOSE: &str = "</tool_call>";

#[derive(Debug, Clone, PartialEq)]
pub struct CallSpec {
    pub max_tokens: u16,
    pub stop: Vec<String>,
    pub temperature: f32,
    pub top_p: f32,
}

impl CallSpec {
    pub fn action(max_tokens: u16) -> Self {
        Self::with_stop(max_tokens, ACTION_CLOSE)
    }

    pub fn with_stop(max_tokens: u16, closing_tag: &str) -> Self {
        Self {
            max_tokens,
            stop: vec![closing_tag.to_string()],
            temperature: TEMPERATURE,
            top_p: TOP_P,
        }
    }

    pub fn primary_stop(&self) -> &str {
        self.stop
            .first()
            .map(String::as_str)
            .unwrap_or(ACTION_CLOSE)
    }
}
