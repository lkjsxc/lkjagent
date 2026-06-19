use crate::metrics::OperationalMetrics;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportEntry {
    pub run_id: String,
    pub timestamp: String,
    pub git_state: String,
    pub model_label: String,
    pub endpoint_host: String,
    pub suite: String,
    pub task_id: String,
    pub family: String,
    pub difficulty: String,
    pub passed: bool,
    pub points_earned: u16,
    pub points_possible: u16,
    pub judge_reason: String,
    pub elapsed_ms: u128,
    pub end_state: String,
    pub metrics: OperationalMetrics,
    pub workspace_path: String,
    pub transcript_path: String,
}

pub fn render_tsv(entries: &[ReportEntry]) -> String {
    let mut output = String::from(header());
    for entry in entries {
        output.push('\n');
        output.push_str(&row(entry));
    }
    output.push('\n');
    output
}

pub fn render_markdown(entries: &[ReportEntry]) -> String {
    let (earned, possible) = totals(entries);
    let passed = entries.iter().filter(|entry| entry.passed).count();
    let mut output = format!(
        "# Benchmark Run {}\n\n## Purpose\n\nScore summary for suite {}.\n\n",
        entries
            .first()
            .map_or("unknown", |entry| entry.run_id.as_str()),
        entries
            .first()
            .map_or("unknown", |entry| entry.suite.as_str())
    );
    output.push_str(&format!(
        "score: {earned}/{possible}\npassed: {passed}/{}\n\n",
        entries.len()
    ));
    output.push_str("| task | family | result | points | end | reason |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for entry in entries {
        output.push_str(&format!(
            "| {} | {} | {} | {}/{} | {} | {} |\n",
            entry.task_id,
            entry.family,
            if entry.passed { "pass" } else { "fail" },
            entry.points_earned,
            entry.points_possible,
            entry.end_state,
            table_text(&entry.judge_reason)
        ));
    }
    output
}

pub fn compare_tsv(old: &str, new: &str) -> String {
    let old_rows = parse_rows(old);
    let new_rows = parse_rows(new);
    let mut improvements = Vec::new();
    let mut regressions = Vec::new();
    let mut changed = Vec::new();
    for new_row in &new_rows {
        if let Some(old_row) = old_rows.iter().find(|row| row.task_id == new_row.task_id) {
            match (old_row.passed, new_row.passed) {
                (false, true) => improvements.push(new_row.task_id.clone()),
                (true, false) => regressions.push(new_row.task_id.clone()),
                _ if old_row.reason != new_row.reason => changed.push(new_row.task_id.clone()),
                _ => {}
            }
        }
    }
    format!(
        "improvements={}\nregressions={}\nchanged_failures={}\n",
        list(&improvements),
        list(&regressions),
        list(&changed)
    )
}

fn header() -> &'static str {
    "run_id\ttimestamp\tgit_state\tmodel_label\tendpoint_host\tsuite\ttask_id\tfamily\tdifficulty\tpassed\tpoints_earned\tpoints_possible\tjudge_reason\tturn_count\telapsed_ms\tend_state\tparse_errors\trepeat_action_notices\ttool_errors\tshell_actions\tfile_writes_edits\tquestions\tworkspace_path\ttranscript_path"
}

fn row(entry: &ReportEntry) -> String {
    [
        esc(&entry.run_id),
        esc(&entry.timestamp),
        esc(&entry.git_state),
        esc(&entry.model_label),
        esc(&entry.endpoint_host),
        esc(&entry.suite),
        esc(&entry.task_id),
        esc(&entry.family),
        esc(&entry.difficulty),
        entry.passed.to_string(),
        entry.points_earned.to_string(),
        entry.points_possible.to_string(),
        esc(&bounded(&entry.judge_reason)),
        entry.metrics.turn_count.to_string(),
        entry.elapsed_ms.to_string(),
        esc(&entry.end_state),
        entry.metrics.parse_errors.to_string(),
        entry.metrics.repeat_action_notices.to_string(),
        entry.metrics.tool_errors.to_string(),
        entry.metrics.shell_actions.to_string(),
        entry.metrics.file_writes_edits.to_string(),
        entry.metrics.questions.to_string(),
        esc(&entry.workspace_path),
        esc(&entry.transcript_path),
    ]
    .join("\t")
}

fn totals(entries: &[ReportEntry]) -> (u32, u32) {
    entries.iter().fold((0, 0), |(earned, possible), entry| {
        (
            earned + u32::from(entry.points_earned),
            possible + u32::from(entry.points_possible),
        )
    })
}

fn esc(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}

fn bounded(value: &str) -> String {
    value.chars().take(240).collect()
}

fn table_text(value: &str) -> String {
    bounded(value).replace('|', "/")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    task_id: String,
    passed: bool,
    reason: String,
}

fn parse_rows(text: &str) -> Vec<Row> {
    text.lines().skip(1).filter_map(parse_row).collect()
}

fn parse_row(line: &str) -> Option<Row> {
    let fields: Vec<&str> = line.split('\t').collect();
    Some(Row {
        task_id: fields.get(6)?.to_string(),
        passed: fields.get(9)? == &"true",
        reason: fields.get(12)?.to_string(),
    })
}

fn list(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}
