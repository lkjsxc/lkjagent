use lkjagent_benchmark::metrics::OperationalMetrics;
use lkjagent_benchmark::report::{compare_tsv, render_markdown, render_tsv, ReportEntry};

#[test]
fn report_rendering_includes_score_and_machine_fields() {
    let entries = vec![entry("a", true, "ok"), entry("b", false, "wrong")];

    let tsv = render_tsv(&entries);
    let markdown = render_markdown(&entries);

    assert!(tsv.contains("run_id\ttimestamp"));
    assert!(tsv.contains("a\tfamily"));
    assert!(markdown.contains("score: 1/2"));
    assert!(markdown.contains("| b | family | fail | 0/1 | timeout | wrong |"));
}

#[test]
fn compare_reports_names_regressions_and_improvements() {
    let old = render_tsv(&[entry("a", false, "wrong"), entry("b", true, "ok")]);
    let new = render_tsv(&[entry("a", true, "ok"), entry("b", false, "new wrong")]);

    let diff = compare_tsv(&old, &new);

    assert!(diff.contains("improvements=a"));
    assert!(diff.contains("regressions=b"));
}

fn entry(task_id: &str, passed: bool, reason: &str) -> ReportEntry {
    ReportEntry {
        run_id: "run".to_string(),
        timestamp: "1".to_string(),
        git_state: "git".to_string(),
        model_label: "model".to_string(),
        endpoint_host: "host".to_string(),
        suite: "tiny".to_string(),
        task_id: task_id.to_string(),
        family: "family".to_string(),
        difficulty: "tiny".to_string(),
        passed,
        points_earned: if passed { 1 } else { 0 },
        points_possible: 1,
        judge_reason: reason.to_string(),
        elapsed_ms: 10,
        end_state: "timeout".to_string(),
        metrics: OperationalMetrics::default(),
        workspace_path: "workspace".to_string(),
        transcript_path: "transcript".to_string(),
    }
}
