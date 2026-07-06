use crate::workbench_state::{UiState, WorkbenchMode};

const CAP: usize = 12_000;

pub fn render(state: &UiState) -> String {
    match state.mode {
        WorkbenchMode::Append => render_append(state),
        WorkbenchMode::Pane => render_pane(state),
    }
}

fn render_append(state: &UiState) -> String {
    bounded(&format!(
        "== workbench refresh {} mode={} ==\n{}\ninput: plain text enqueues; /mode pane switches layout; /quit exits workbench",
        state.refreshes,
        state.mode.as_str(),
        state.latest
    ))
}

fn render_pane(state: &UiState) -> String {
    let sections = split_sections(&state.latest);
    let transcript = sections
        .iter()
        .filter(|(name, _)| *name != "status")
        .map(|(name, body)| format!("[{}]\n{}", name, body.trim()))
        .collect::<Vec<_>>()
        .join("\n\n");
    let left = window(&transcript, state.scroll, 18);
    let right = sections
        .iter()
        .find(|(name, _)| *name == "status")
        .map(|(_, body)| body.trim())
        .unwrap_or("status: unavailable");
    bounded(&format!(
        "== workbench pane refresh {} scroll={} ==\n+-- transcript --+\n{}\n+-- right rail --+\n{}\n+-- input --+\nplain text enqueues | /mode append | /quit",
        state.refreshes, state.scroll, left, right
    ))
}

fn split_sections(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::<(String, String)>::new();
    for line in body.lines() {
        if let Some(name) = section_name(line) {
            sections.push((name, String::new()));
        } else if let Some((_, text)) = sections.last_mut() {
            if !text.is_empty() {
                text.push('\n');
            }
            text.push_str(line);
        }
    }
    if sections.is_empty() {
        sections.push(("body".to_string(), body.to_string()));
    }
    sections
}

fn window(text: &str, scroll: usize, height: usize) -> String {
    let lines = text.lines().skip(scroll).take(height).collect::<Vec<_>>();
    if lines.is_empty() {
        return "[end of pane]".to_string();
    }
    lines.join("\n")
}

fn section_name(line: &str) -> Option<String> {
    line.strip_prefix("== ")
        .and_then(|rest| rest.strip_suffix(" =="))
        .map(str::to_string)
}

fn bounded(text: &str) -> String {
    if text.len() <= CAP {
        return text.to_string();
    }
    let keep = CAP.saturating_sub(20);
    format!(
        "{}\n[workbench truncated]",
        text.chars().take(keep).collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_renderer_groups_status_and_transcript() {
        let mut state = UiState::new(WorkbenchMode::Pane);
        state.refreshes = 2;
        state.scroll = 1;
        state.latest = "== status ==\ndaemon: idle\n== recent events ==\none\ntwo".to_string();

        let text = render(&state);

        assert!(text.contains("== workbench pane refresh 2 scroll=1 =="));
        assert!(text.contains("+-- transcript --+"));
        assert!(text.contains("two"));
        assert!(!text.contains("[recent events]"));
        assert!(text.contains("daemon: idle"));
    }
}
