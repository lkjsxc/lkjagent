use crate::workbench_state::{visible_height, UiState, WorkbenchMode};

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
        .find(|(name, _)| *name == "transcript")
        .map_or_else(
            || section_group(&sections, |name| name != "status"),
            |(_, body)| body.trim().to_string(),
        );
    let transcript = filter_search(&transcript, &state.search);
    let left = window(
        &transcript,
        state.scroll,
        state.follow,
        visible_height(state),
    );
    let right = section_group(&sections, |name| name != "transcript");
    bounded(&format!(
        "== workbench pane refresh {} scroll={} follow={} search={} ==\n+-- transcript --+\n{}\n+-- right rail --+\n{}\n+-- input --+\nplain text enqueues | /follow on|off | /search TEXT | /mode append | /quit",
        state.refreshes,
        state.scroll,
        state.follow,
        search_label(state),
        left,
        rail_summary(&right)
    ))
}

fn section_group(sections: &[(String, String)], keep: impl Fn(&str) -> bool) -> String {
    let text = sections
        .iter()
        .filter(|(name, _)| keep(name))
        .map(|(name, body)| format!("[{}]\n{}", name, body.trim()))
        .collect::<Vec<_>>()
        .join("\n\n");
    if text.trim().is_empty() {
        "status: unavailable".to_string()
    } else {
        text
    }
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
    fn pane_uses_transcript_section_as_left_pane() -> Result<(), String> {
        let mut state = UiState::new(WorkbenchMode::Pane);
        state.latest = "== status ==\ndaemon: idle\n== transcript ==\nowner: hello\nagent: hello\n== recent events ==\nstepdone hello".to_string();

        let text = render(&state);
        let left = between(&text, "+-- transcript --+", "+-- right rail --+")?;

        assert!(left.contains("owner: hello"));
        assert!(left.contains("agent: hello"));
        assert!(!left.contains("stepdone hello"));
        assert!(text.contains("[status]"));
        Ok(())
    }

    #[test]
    fn pane_follow_stays_bottom_anchored_after_growth() {
        let mut state = UiState::new(WorkbenchMode::Pane);
        state.latest = transcript_body(25);
        let before = render(&state);
        state.latest = transcript_body(26);
        let after = render(&state);

        assert!(before.contains("line-25"));
        assert!(!before.contains("line-01"));
        assert!(after.contains("line-26"));
        assert!(!after.contains("line-01"));
    }

    fn transcript_body(count: usize) -> String {
        let mut text = "== status ==\ndaemon: idle\n== transcript ==".to_string();
        for index in 1..=count {
            text.push_str(&format!("\nline-{index:02}"));
        }
        text
    }

    fn between<'a>(text: &'a str, start: &str, end: &str) -> Result<&'a str, String> {
        let start_at = text.find(start).ok_or_else(|| format!("missing {start}"))? + start.len();
        let rest = &text[start_at..];
        let end_at = rest.find(end).ok_or_else(|| format!("missing {end}"))?;
        Ok(&rest[..end_at])
    }
}
