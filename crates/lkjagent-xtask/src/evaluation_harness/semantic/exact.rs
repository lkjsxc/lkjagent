use super::{shared, Context, Measured};
use crate::evaluation_harness::sha256;
use rusqlite::OptionalExtension;
use std::os::unix::fs::PermissionsExt;

pub fn measure(ctx: &Context<'_>) -> Result<Measured, String> {
    let expected = expected_files(ctx)?;
    let after = shared::manifest_rows(ctx.after);
    let changed = shared::changed_paths(ctx.before, ctx.after);
    let exact = expected
        .iter()
        .all(|(path, hash)| after.get(path).is_some_and(|row| &row.sha256 == hash));
    let requested = expected.keys().cloned().collect::<Vec<_>>();
    let requested_only = changed == requested;
    let edit = target(ctx, "notes/exact-base.txt")?;
    let create_prior_absent =
        !shared::manifest_rows(ctx.before).contains_key("notes/created-proof.txt");
    let first = shared::text(ctx.db, "SELECT CAST(t.normalized_path AS TEXT) FROM effect_targets t JOIN effect_journal j ON j.id=t.journal_id JOIN runtime_decisions d ON d.id=j.decision_id WHERE t.operation IN ('create','replace') ORDER BY d.selected_monotonic_ms,d.id LIMIT 1")?.unwrap_or_default();
    let current_mode = std::fs::metadata(ctx.capture.workspace.join("notes/exact-base.txt"))
        .map_err(|error| error.to_string())?
        .permissions()
        .mode()
        & 0o777;
    let closed = shared::count(
        ctx.db,
        "SELECT count(*) FROM matters WHERE lifecycle='closed'",
    )?;
    let agents = shared::count(
        ctx.db,
        "SELECT count(*) FROM conversation_messages WHERE role='agent'",
    )?;
    let effects = shared::count(ctx.db, "SELECT count(*) FROM effect_journal")?;
    let create_effects = effects.saturating_sub(u64::from(edit.operation == "replace"));
    let admissions = shared::count(ctx.db, "SELECT count(*) FROM tool_admissions")?;
    let receipts = shared::count(
        ctx.db,
        "SELECT count(*) FROM conversation_messages WHERE role='agent' AND receipt IS NOT NULL",
    )?;
    let fields = vec![
        ("fact_exact_path".into(), "notes/exact-base.txt".into()),
        (
            "fact_exact_sha256".into(),
            expected["notes/exact-base.txt"].clone(),
        ),
        ("fact_created_path".into(), "notes/created-proof.txt".into()),
        (
            "fact_created_sha256".into(),
            expected["notes/created-proof.txt"].clone(),
        ),
        ("fact_workspace_file_count".into(), after.len().to_string()),
        (
            "fact_changed_path_fingerprint".into(),
            shared::fingerprint(&changed),
        ),
        ("fact_changed_path_count".into(), changed.len().to_string()),
        ("fact_first_effect_path".into(), first.clone()),
        ("fact_edit_prior_sha256".into(), edit.prior_hash),
        ("fact_edit_intended_sha256".into(), edit.intended_hash),
        ("fact_edit_prior_mode".into(), edit.prior_mode.to_string()),
        (
            "fact_edit_intended_mode".into(),
            edit.intended_mode.to_string(),
        ),
        ("fact_edit_current_mode".into(), current_mode.to_string()),
        (
            "fact_create_prior_absent_count".into(),
            u64::from(create_prior_absent).to_string(),
        ),
        (
            "fact_create_effect_count".into(),
            create_effects.to_string(),
        ),
        (
            "fact_edit_effect_count".into(),
            u64::from(edit.operation == "replace").to_string(),
        ),
        ("fact_closed_matter_count".into(), closed.to_string()),
        ("fact_agent_message_count".into(), agents.to_string()),
        ("fact_receipt_count".into(), receipts.to_string()),
        (
            "fact_current_passed_check_count".into(),
            ctx.common.current_passed_checks.to_string(),
        ),
        ("fact_effect_count".into(), effects.to_string()),
        ("fact_tool_admission_count".into(), admissions.to_string()),
        (
            "fact_table_count".into(),
            ctx.common.table_count.to_string(),
        ),
    ];
    let passed = exact
        && after.len() == 2
        && requested_only
        && first == "notes/exact-base.txt"
        && edit.hashes_present
        && edit.prior_mode == edit.intended_mode
        && edit.intended_mode == current_mode
        && create_prior_absent
        && create_effects == 1
        && ctx.common.owner_turns >= 6
        && ctx.common.current_passed_checks >= 6
        && effects == 2
        && admissions > 0
        && ctx.common.provider_exchanges > 0
        && ctx.common.table_count == 18;
    Ok(Measured::new(passed, fields))
}

#[derive(Default)]
struct Target {
    prior_hash: String,
    intended_hash: String,
    prior_mode: u32,
    intended_mode: u32,
    operation: String,
    hashes_present: bool,
}
fn target(ctx: &Context<'_>, path: &str) -> Result<Target, String> {
    ctx.db.query_row("SELECT prior_bytes,intended_bytes,coalesce(prior_mode,-1),coalesce(intended_mode,-1),operation FROM effect_targets WHERE CAST(normalized_path AS TEXT)=?1", [path], |row| {
        let prior: Option<Vec<u8>> = row.get(0)?; let intended: Option<Vec<u8>> = row.get(1)?;
        Ok(Target { prior_hash: prior.as_deref().map(sha256).unwrap_or_default(), intended_hash: intended.as_deref().map(sha256).unwrap_or_default(), prior_mode: row.get::<_, i64>(2)?.max(0) as u32, intended_mode: row.get::<_, i64>(3)?.max(0) as u32, operation: row.get(4)?, hashes_present: intended.is_some() && prior.is_some() })
    }).optional().map(|value| value.unwrap_or_default()).map_err(|error| error.to_string())
}
fn expected_files(ctx: &Context<'_>) -> Result<std::collections::BTreeMap<String, String>, String> {
    let text = std::fs::read_to_string(ctx.scenario.path.join("checks.tsv"))
        .map_err(|error| error.to_string())?;
    Ok(text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            (fields.get(1) == Some(&"workspace-file-sha256"))
                .then(|| {
                    fields
                        .get(2)?
                        .split_once('=')
                        .map(|(path, hash)| (path.into(), hash.into()))
                })
                .flatten()
        })
        .collect())
}
