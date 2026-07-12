pub mod checks;
pub mod error;
pub mod exchange;
pub mod shell;
pub mod workspace;
mod workspace_capability;
mod workspace_scan;

pub mod observation {
    const OBSERVATION_CHARS: usize = 6_000;

    pub fn observation(status: &str, content: &str) -> String {
        format!(
            "<observation>\n<status>{status}</status>\n<content>\n{}\n</content>\n</observation>",
            bound(content, OBSERVATION_CHARS)
        )
    }

    pub fn bound(text: &str, cap: usize) -> String {
        if text.len() <= cap {
            return text.to_string();
        }
        let keep = cap.saturating_sub(5) / 2;
        let head = text.chars().take(keep).collect::<String>();
        let tail = text
            .chars()
            .rev()
            .take(keep)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        format!("{head}[...]{tail}")
    }
}
