use super::{shared, Context, Measured};

pub fn measure(ctx: &Context<'_>) -> Result<Measured, String> {
    let after = shared::manifest_rows(ctx.after);
    let journal_path = after
        .keys()
        .find(|path| path.starts_with("life/journal/") && path.ends_with("/entry.md"))
        .cloned()
        .unwrap_or_default();
    let journal_text = if journal_path.is_empty() {
        String::new()
    } else {
        shared::read(ctx.capture.workspace.as_path(), &journal_path)?
    };
    let journal_revision = if journal_path.is_empty() {
        None
    } else {
        shared::revision_for_path(ctx.db, &journal_path)?
    };
    let journal_parent = match journal_revision.as_deref() {
        Some(revision) => shared::parent_revision(ctx.db, revision)?,
        None => None,
    };
    let memory_revision = shared::text(
        ctx.db,
        "SELECT CAST(current_revision_id AS TEXT) FROM workspace_documents WHERE managed=1 AND CAST(current_path AS TEXT) LIKE 'knowledge/notes/%' ORDER BY CAST(current_path AS TEXT) LIMIT 1",
    )?
    .unwrap_or_default();
    let relevant_recall = shared::count(
        ctx.db,
        "SELECT count(*) FROM context_items WHERE instr(CAST(source_id AS TEXT),'older-fact.md')>0",
    )?;
    let noise_recall = shared::count(
        ctx.db,
        "SELECT count(*) FROM context_items WHERE instr(CAST(source_id AS TEXT),'recent-noise.md')>0",
    )?;
    let stale_memory = if memory_revision.is_empty() {
        0
    } else {
        shared::scalar_with(
            ctx.db,
            "SELECT count(*) FROM context_items WHERE source_kind='memory' AND CAST(source_revision AS TEXT)<>?1",
            &memory_revision,
        )?
    };
    let rogue_memory = shared::count(
        ctx.db,
        "SELECT count(*) FROM context_items WHERE source_kind='memory' AND CAST(source_revision AS TEXT) NOT IN (SELECT CAST(current_revision_id AS TEXT) FROM workspace_documents WHERE managed=1 AND status='active')",
    )?;
    let token_units = shared::token_units(&journal_text);
    let placeholder_count = shared::placeholder_count(&journal_text);
    let journal_lineage = if journal_path.is_empty() {
        0
    } else {
        shared::scalar_with(
        ctx.db,
        "SELECT count(*) FROM context_items WHERE decision_id=(SELECT e.decision_id FROM workspace_documents d JOIN workspace_revisions r ON r.id=d.current_revision_id JOIN effect_journal e ON e.id=r.effect_id WHERE CAST(d.current_path AS TEXT)=?1)",
        &journal_path,
    )?
    };
    let journal_sha = after
        .get(&journal_path)
        .map(|row| row.sha256.clone())
        .unwrap_or_default();
    let fields = vec![
        ("fact_journal_path".into(), journal_path.clone()),
        (
            "fact_journal_revision_id".into(),
            journal_revision.unwrap_or_default(),
        ),
        (
            "fact_journal_parent_revision_id".into(),
            journal_parent.unwrap_or_default(),
        ),
        ("fact_journal_sha256".into(), journal_sha),
        ("fact_journal_token_units".into(), token_units.to_string()),
        (
            "fact_journal_placeholder_count".into(),
            placeholder_count.to_string(),
        ),
        (
            "fact_journal_lineage_count".into(),
            journal_lineage.to_string(),
        ),
        (
            "fact_memory_current_revision_id".into(),
            memory_revision.clone(),
        ),
        (
            "fact_relevant_recall_context_count".into(),
            relevant_recall.to_string(),
        ),
        (
            "fact_noise_recall_context_count".into(),
            noise_recall.to_string(),
        ),
        (
            "fact_stale_memory_context_count".into(),
            stale_memory.to_string(),
        ),
        (
            "fact_rogue_memory_context_count".into(),
            rogue_memory.to_string(),
        ),
    ];
    let passed = !journal_path.is_empty()
        && !memory_revision.is_empty()
        && token_units > 0
        && token_units <= 512
        && placeholder_count == 0
        && journal_lineage > 0
        && relevant_recall == 1
        && noise_recall == 0
        && stale_memory == 0
        && rogue_memory == 0;
    Ok(Measured::new(passed, fields))
}
