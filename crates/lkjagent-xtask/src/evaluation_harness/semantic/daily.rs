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
    let memory_revision = shared::text(
        ctx.db,
        "SELECT CAST(current_revision_id AS TEXT) FROM workspace_documents WHERE managed=1 AND CAST(current_path AS TEXT)='knowledge/notes/transit-card-location.md' AND status='active'",
    )?.unwrap_or_default();
    let memory_text = shared::text(
        ctx.db,
        "SELECT CAST(r.content AS TEXT) FROM workspace_documents d JOIN workspace_revisions r ON r.id=d.current_revision_id WHERE CAST(d.current_path AS TEXT)='knowledge/notes/transit-card-location.md' AND d.status='active'",
    )?.unwrap_or_default();
    let initial_recall = shared::count(
        ctx.db,
        &memory_context("Use saved transit-card-location%", None),
    )?;
    let relevant_recall = shared::count(
        ctx.db,
        &recall_context("Use corrected transit-card-location%", &memory_revision),
    )?;
    let correction_memory = shared::count(
        ctx.db,
        &memory_context("correct transit-card-location:%", None),
    )?;
    let stale_memory = shared::count(
        ctx.db,
        &format!(
            "{} AND CAST(i.source_revision AS TEXT)<>'{}'",
            recall_context("Use corrected transit-card-location%", ""),
            sql(&memory_revision)
        ),
    )?;
    let noise_recall = shared::count(
        ctx.db,
        "SELECT count(*) FROM context_items WHERE instr(CAST(source_id AS TEXT),'recent-noise.md')>0",
    )?;
    let rogue_memory = shared::count(
        ctx.db,
        "SELECT count(*) FROM context_items WHERE source_kind='memory' AND CAST(source_revision AS TEXT) NOT IN (SELECT CAST(r.id AS TEXT) FROM workspace_revisions r JOIN workspace_documents d ON d.id=r.document_id WHERE d.managed=1)",
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
    let journal_lower = journal_text.to_ascii_lowercase();
    let journal_grounded = u64::from(
        journal_lower.contains("card")
            && journal_lower.contains("drawer")
            && ["second", "2nd", "number two", "drawer two"]
                .iter()
                .any(|term| journal_lower.contains(term)),
    );
    let memory_lower = memory_text.to_ascii_lowercase();
    let memory_grounded = u64::from(
        memory_lower.contains("transit card")
            && memory_lower.contains("top")
            && memory_lower.contains("drawer"),
    );
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
            "fact_journal_grounded_owner_fact_count".into(),
            journal_grounded.to_string(),
        ),
        (
            "fact_memory_current_revision_id".into(),
            memory_revision.clone(),
        ),
        (
            "fact_memory_grounded_correction_count".into(),
            memory_grounded.to_string(),
        ),
        (
            "fact_initial_recall_context_count".into(),
            initial_recall.to_string(),
        ),
        (
            "fact_relevant_recall_context_count".into(),
            relevant_recall.to_string(),
        ),
        (
            "fact_correction_memory_context_count".into(),
            correction_memory.to_string(),
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
        && (1..=512).contains(&token_units)
        && placeholder_count == 0
        && journal_lineage > 0
        && journal_grounded == 1
        && memory_grounded == 1
        && initial_recall == 1
        && relevant_recall == 1
        && correction_memory == 0
        && noise_recall == 0
        && stale_memory == 0
        && rogue_memory == 0;
    Ok(Measured::new(passed, fields))
}

fn recall_context(objective: &str, revision: &str) -> String {
    let revision = if revision.is_empty() {
        String::new()
    } else {
        format!(" AND CAST(i.source_revision AS TEXT)='{}'", sql(revision))
    };
    format!(
        "SELECT count(DISTINCT m.id) FROM context_items i JOIN runtime_decisions d ON d.id=i.decision_id JOIN matters m ON m.id=d.matter_id WHERE (i.source_kind='memory' OR (i.source_kind='source' AND CAST(i.source_id AS TEXT)='knowledge/notes/transit-card-location.md')) AND CAST(m.objective AS TEXT) LIKE '{}'{}",
        sql(objective), revision
    )
}
fn memory_context(objective: &str, revision: Option<&str>) -> String {
    let revision = revision.map_or(String::new(), |value| {
        format!(" AND CAST(i.source_revision AS TEXT)='{}'", sql(value))
    });
    format!(
        "SELECT count(DISTINCT m.id) FROM context_items i JOIN runtime_decisions d ON d.id=i.decision_id JOIN matters m ON m.id=d.matter_id WHERE i.source_kind='memory' AND CAST(m.objective AS TEXT) LIKE '{}'{}",
        sql(objective), revision
    )
}
fn sql(value: &str) -> String {
    value.replace('\'', "''")
}
