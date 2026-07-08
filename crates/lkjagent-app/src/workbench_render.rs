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
        "== workbench refresh {} mode={} follow={} search={} ==\n{}\ninput: plain text enqueues; /mode pane switches layout; /search TEXT filters; /quit exits workbench",
        state.refreshes,
        state.mode.as_str(),
        state.follow,
        search_label(state),
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
    let transcript = filter_search(&transcript, &state.search);
    let left = window(&transcript, state.scroll, state.follow, 18);
    let right = sections
        .iter()
        .find(|(name, _)| *name == "status")
        .map(|(_, body)| body.trim())
        .unwrap_or("status: unavailable");
    bounded(&format!(
        "== workbench pane refresh {} scroll={} follow={} search={} ==\n+-- transcript --+\n{}\n+-- right rail --+\n{}\n+-- input --+\nplain text enqueues | /follow on|off | /search TEXT | /mode append | /quit",
        state.refreshes,
        state.scroll,
        state.follow,
        search_label(state),
        left,
        rail_summary(right)
    ))
}

fn search_label(state: &UiState) -> &str {
    if state.search.is_empty() {
        "none"
    } else {
        state.search.as_str()
    }
}

fn filter_search(text: &str, query: &str) -> String {
    if query.is_empty() {
        return text.to_string();
    }
    let needle = query.to_ascii_lowercase();
    text.lines()
        .filter(|line| line.to_ascii_lowercase().contains(&needle))
        .collect::<Vec<_>>()
        .join("\n")
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

fn window(text: &str, scroll: usize, follow: bool, height: usize) -> String {
    let all = text.lines().collect::<Vec<_>>();
    let start = if follow {
        all.len().saturating_sub(height)
    } else {
        scroll
    };
    let lines = all.into_iter().skip(start).take(height).collect::<Vec<_>>();
    if lines.is_empty() {
        return "[end of pane]".to_string();
    }
    lines.join("\n")
}

fn rail_summary(status: &str) -> String {
    let mut lines = status
        .lines()
        .take(12)
        .map(str::to_string)
        .collect::<Vec<_>>();
    for label in ["model", "faults", "workspace", "repo"] {
        if !status.contains(label) {
            lines.push(format!("{label}: see rows"));
        }
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
        state.follow = false;
        state.latest = "== status ==\ndaemon: idle\n== recent events ==\none\ntwo".to_string();

        let text = render(&state);

        assert!(text.contains("== workbench pane refresh 2 scroll=1 follow=false search=none =="));
        assert!(text.contains("+-- transcript --+"));
        assert!(text.contains("two"));
        assert!(!text.contains("[recent events]"));
        assert!(text.contains("daemon: idle"));
        assert!(text.contains("model: see rows"));
    }

    #[test]
    fn pane_follow_stays_bottom_anchored_after_growth() {
        let mut state = UiState::new(WorkbenchMode::Pane);
        state.follow = true;
        state.latest = transcript_body(25);
        let before = render(&state);
        state.latest = transcript_body(26);
        let after = render(&state);

        assert!(before.contains("line-25"));
        assert!(!before.contains("line-01"));
        assert!(after.contains("line-26"));
        assert!(!after.contains("line-01"));
    }

    #[test]
    fn pane_manual_scroll_stays_manual_after_growth() {
        let mut state = UiState::new(WorkbenchMode::Pane);
        state.follow = false;
        state.scroll = 1;
        state.latest = transcript_body(25);
        let before = render(&state);
        state.latest = transcript_body(26);
        let after = render(&state);

        assert!(before.contains("line-02"));
        assert!(after.contains("line-02"));
        assert!(!after.contains("line-26"));
    }

    fn transcript_body(count: usize) -> String {
        let mut lines = vec!["== status ==".to_string(), "daemon: idle".to_string()];
        lines.push("== transcript ==".to_string());
        for index in 1..=count {
            lines.push(format!("line-{index:02}"));
        }
        lines.join("\n")
    }
}
