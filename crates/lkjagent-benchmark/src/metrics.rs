#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OperationalMetrics {
    pub turn_count: u32,
    pub parse_errors: u32,
    pub repeat_action_notices: u32,
    pub tool_errors: u32,
    pub shell_actions: u32,
    pub file_writes_edits: u32,
    pub questions: u32,
}

pub fn metrics_from_status_and_log(status: &str, log: &str) -> OperationalMetrics {
    OperationalMetrics {
        turn_count: status_value(status, "turns").unwrap_or_else(|| count_turns(log)),
        parse_errors: count_needles(log, &["parse faults", "MissingAct", "MultipleAct"]),
        repeat_action_notices: count_needles(log, &["repeat action refused"]),
        tool_errors: count_needles(log, &["<status>error</status>"]),
        shell_actions: count_needles(log, &["<tool>shell.run</tool>"]),
        file_writes_edits: count_needles(log, &["<tool>fs.write</tool>", "<tool>fs.edit</tool>"]),
        questions: count_needles(log, &["<tool>agent.ask</tool>"]),
    }
}

fn status_value(status: &str, key: &str) -> Option<u32> {
    let prefix = format!("{key}=");
    status.lines().find_map(|line| {
        line.strip_prefix(&prefix)
            .and_then(|value| value.trim().parse::<u32>().ok())
    })
}

fn count_turns(log: &str) -> u32 {
    let mut max_turn = 0_u32;
    for line in log.lines() {
        if let Some(value) = field_value(line, "turn") {
            if let Ok(parsed) = value.parse::<u32>() {
                max_turn = max_turn.max(parsed);
            }
        }
    }
    max_turn
}

fn field_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&prefix))
}

fn count_needles(text: &str, needles: &[&str]) -> u32 {
    needles
        .iter()
        .map(|needle| text.matches(needle).count() as u32)
        .sum()
}
