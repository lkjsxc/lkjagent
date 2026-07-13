use super::{shared, Context, Measured};

pub fn measure(ctx: &Context<'_>) -> Result<Measured, String> {
    let orbit = "project-orbit/src/lib.rs";
    let orbital = "project-orbital/src/lib.rs";
    let orbit_revision = shared::revision_for_path(ctx.db, orbit)?.unwrap_or_default();
    let orbital_revision = shared::revision_for_path(ctx.db, orbital)?.unwrap_or_default();
    let orbit_in_orbital = contamination(ctx, &orbit_revision, "project-orbital")?;
    let orbital_in_orbit = contamination(ctx, &orbital_revision, "project-orbit")?;
    let changed = shared::changed_paths(ctx.before, ctx.after);
    let restart_resume = shared::restart_survivors(&ctx.capture.raw, ctx.db, "revision")?;
    let duplicate_effects = shared::count(ctx.db, "SELECT count(*) FROM effect_journal")?
        .saturating_sub(shared::count(
            ctx.db,
            "SELECT count(DISTINCT decision_id) FROM effect_journal",
        )?);
    let fields = vec![
        ("fact_orbit_revision_id".into(), orbit_revision.clone()),
        ("fact_orbital_revision_id".into(), orbital_revision.clone()),
        (
            "fact_orbit_in_orbital_context_count".into(),
            orbit_in_orbital.to_string(),
        ),
        (
            "fact_orbital_in_orbit_context_count".into(),
            orbital_in_orbit.to_string(),
        ),
        (
            "fact_changed_path_fingerprint".into(),
            shared::fingerprint(&changed),
        ),
        ("fact_changed_path_count".into(), changed.len().to_string()),
        (
            "fact_current_passed_check_count".into(),
            ctx.common.current_passed_checks.to_string(),
        ),
        (
            "fact_restart_resume_count".into(),
            restart_resume.to_string(),
        ),
        (
            "fact_duplicate_effect_count".into(),
            duplicate_effects.to_string(),
        ),
    ];
    let passed = !orbit_revision.is_empty()
        && !orbital_revision.is_empty()
        && orbit_in_orbital == 0
        && orbital_in_orbit == 0
        && !changed.is_empty()
        && restart_resume > 0
        && duplicate_effects == 0;
    Ok(Measured::new(passed, fields))
}

fn contamination(ctx: &Context<'_>, revision: &str, project: &str) -> Result<u64, String> {
    if revision.is_empty() {
        return Ok(0);
    }
    let mut query = ctx.db.prepare(
        "SELECT m.objective FROM context_items i JOIN runtime_decisions d ON d.id=i.decision_id JOIN matters m ON m.id=d.matter_id WHERE CAST(i.source_revision AS TEXT)=?1",
    ).map_err(|error| error.to_string())?;
    let rows = query
        .query_map([revision], |row| row.get::<_, Vec<u8>>(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(rows
        .into_iter()
        .filter(|body| project_ids(&String::from_utf8_lossy(body)).contains(project))
        .count() as u64)
}

fn project_ids(text: &str) -> std::collections::BTreeSet<String> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .filter(|word| word.starts_with("project-") && word.len() <= 128)
        .map(str::to_ascii_lowercase)
        .collect()
}
