use super::{shared, Context, Measured};

pub fn measure(ctx: &Context<'_>) -> Result<Measured, String> {
    let changed = shared::changed_paths(ctx.before, ctx.after);
    let readme = changed
        .iter()
        .find(|path| path.starts_with("artifacts/documents/") && path.ends_with("/README.md"))
        .cloned()
        .unwrap_or_default();
    let root = readme.strip_suffix("README.md").unwrap_or("");
    let children = changed
        .iter()
        .filter(|path| {
            !root.is_empty()
                && path.starts_with(root)
                && path.ends_with(".md")
                && *path != &readme
                && !path[root.len()..].contains('/')
        })
        .cloned()
        .collect::<Vec<_>>();
    let mut aggregate_words = 0;
    let mut nonplaceholder = 0;
    let mut empty_units = 0;
    let mut truncated = 0;
    for path in &children {
        let text = shared::read(ctx.capture.workspace.as_path(), path)?;
        aggregate_words += body_words(&text);
        if text.trim().is_empty() {
            empty_units += 1;
        } else if shared::placeholder_count(&text) == 0 {
            nonplaceholder += 1;
        }
        truncated += u64::from(text.contains("..."));
    }
    let readme_text = if readme.is_empty() {
        String::new()
    } else {
        shared::read(ctx.capture.workspace.as_path(), &readme)?
    };
    if !readme_text.trim().is_empty() && shared::placeholder_count(&readme_text) == 0 {
        nonplaceholder += 1;
    } else {
        empty_units += 1;
    }
    let readme_links = readme_text.matches("](").count() as u64;
    let recovery_events = shared::count(
        ctx.db,
        "SELECT count(*) FROM runtime_events WHERE kind='model-output-rejected' AND instr(lower(CAST(payload AS TEXT)),'output-limit')>0",
    )?;
    let strategy_changes = shared::count(
        ctx.db,
        "SELECT count(DISTINCT CAST(operation_key AS TEXT)) FROM runtime_decisions WHERE CAST(operation_key AS TEXT) LIKE 'recovery.%'",
    )?;
    let restart_resume = shared::count(
        ctx.db,
        "SELECT count(*) FROM runtime_events WHERE kind='owner-resume'",
    )?;
    let early_completion = shared::count(
        ctx.db,
        "SELECT count(*) FROM conversation_messages WHERE role='agent' AND receipt IS NOT NULL AND sequence < (SELECT coalesce(max(sequence),0) FROM conversation_messages WHERE role='owner')",
    )?;
    let source_lineage = shared::count(
        ctx.db,
        "SELECT count(*) FROM context_items i WHERE i.decision_id IN (SELECT e.decision_id FROM effect_journal e JOIN workspace_revisions r ON r.effect_id=e.id JOIN workspace_documents d ON d.id=r.document_id WHERE CAST(d.current_path AS TEXT) GLOB 'artifacts/documents/*.md' OR CAST(d.current_path AS TEXT) GLOB 'artifacts/documents/*/*.md')",
    )?;
    let current_revisions = if root.is_empty() {
        0
    } else {
        shared::scalar_with(
        ctx.db,
        "SELECT count(*) FROM workspace_documents WHERE status='active' AND CAST(current_path AS TEXT) LIKE ?1",
        &format!("{root}%"),
    )?
    };
    let report_checks = shared::count(
        ctx.db,
        "SELECT count(*) FROM checks WHERE current=1 AND passed=1 AND kind IN ('managed-report-map','managed-report-member','managed-report-complete')",
    )?;
    let fields = vec![
        ("fact_artifact_readme_path".into(), readme.clone()),
        (
            "fact_artifact_child_link_fingerprint".into(),
            shared::fingerprint(&children),
        ),
        (
            "fact_artifact_child_count".into(),
            children.len().to_string(),
        ),
        ("fact_readme_link_count".into(), readme_links.to_string()),
        (
            "fact_nonplaceholder_unit_count".into(),
            nonplaceholder.to_string(),
        ),
        ("fact_empty_unit_count".into(), empty_units.to_string()),
        (
            "fact_aggregate_word_count".into(),
            aggregate_words.to_string(),
        ),
        (
            "fact_source_lineage_count".into(),
            source_lineage.to_string(),
        ),
        (
            "fact_current_revision_count".into(),
            current_revisions.to_string(),
        ),
        (
            "fact_report_current_check_count".into(),
            report_checks.to_string(),
        ),
        (
            "fact_output_limit_recovery_count".into(),
            recovery_events.to_string(),
        ),
        (
            "fact_strategy_change_count".into(),
            strategy_changes.to_string(),
        ),
        (
            "fact_truncated_revision_count".into(),
            truncated.to_string(),
        ),
        (
            "fact_restart_resume_count".into(),
            restart_resume.to_string(),
        ),
        (
            "fact_early_completion_count".into(),
            early_completion.to_string(),
        ),
    ];
    let passed = !readme.is_empty()
        && children.len() >= 2
        && readme_links == children.len() as u64
        && nonplaceholder == children.len() as u64 + 1
        && empty_units == 0
        && aggregate_words >= 1_500
        && source_lineage > children.len() as u64
        && current_revisions == children.len() as u64 + 1
        && report_checks >= children.len() as u64 + 2
        && recovery_events > 0
        && strategy_changes > 1
        && truncated == 0
        && restart_resume > 0
        && early_completion == 0;
    Ok(Measured::new(passed, fields))
}

fn body_words(text: &str) -> u64 {
    let body = text.split("\n---\n").nth(1).unwrap_or(text);
    shared::word_count(body.lines().skip(2).collect::<Vec<_>>().join("\n").as_str())
}
