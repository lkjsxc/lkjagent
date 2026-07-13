use super::{shared, Context, Measured};

pub fn measure(ctx: &Context<'_>) -> Result<Measured, String> {
    let checks = std::fs::read_to_string(ctx.scenario.path.join("checks.tsv"))
        .map_err(|error| error.to_string())?;
    let expected = checks
        .lines()
        .skip(1)
        .find_map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            (fields.get(1) == Some(&"workspace-file-sha256"))
                .then(|| fields.get(2).copied())
                .flatten()
        })
        .ok_or("exact scenario byte check missing")?;
    let (path, sha256) = expected
        .split_once('=')
        .ok_or("exact scenario byte check malformed")?;
    let after = shared::manifest_rows(ctx.after);
    let changed = shared::changed_paths(ctx.before, ctx.after);
    let agent_messages = shared::count(
        ctx.db,
        "SELECT count(*) FROM conversation_messages WHERE role='agent'",
    )?;
    let closed_matters = shared::count(
        ctx.db,
        "SELECT count(*) FROM matters WHERE lifecycle='closed'",
    )?;
    let effect_count = shared::count(ctx.db, "SELECT count(*) FROM effect_journal")?;
    let tool_admissions = shared::count(ctx.db, "SELECT count(*) FROM tool_admissions")?;
    let expected_ok = after.get(path).is_some_and(|row| row.sha256 == sha256);
    let one_file = after.len() == 1;
    let requested_only = changed == [path.to_string()];
    let fields = vec![
        ("fact_exact_path".into(), path.into()),
        ("fact_exact_sha256".into(), sha256.into()),
        ("fact_workspace_file_count".into(), after.len().to_string()),
        ("fact_changed_path_fingerprint".into(), shared::fingerprint(&changed)),
        ("fact_changed_path_count".into(), changed.len().to_string()),
        ("fact_closed_matter_count".into(), closed_matters.to_string()),
        ("fact_agent_message_count".into(), agent_messages.to_string()),
        (
            "fact_current_passed_check_count".into(),
            ctx.common.current_passed_checks.to_string(),
        ),
        ("fact_effect_count".into(), effect_count.to_string()),
        ("fact_tool_admission_count".into(), tool_admissions.to_string()),
        ("fact_table_count".into(), ctx.common.table_count.to_string()),
        (
            "semantic_detail".into(),
            format!(
                "file_exact={expected_ok};one_file={one_file};closed={closed_matters};owner={};agent={agent_messages};passed_checks={};effects={effect_count};admissions={tool_admissions};providers={};tables={}",
                ctx.common.owner_turns,
                ctx.common.current_passed_checks,
                ctx.common.provider_exchanges,
                ctx.common.table_count,
            ),
        ),
    ];
    let passed = expected_ok
        && one_file
        && requested_only
        && closed_matters >= 3
        && ctx.common.owner_turns >= 5
        && agent_messages >= 3
        && ctx.common.current_passed_checks >= 6
        && effect_count == 1
        && tool_admissions > 0
        && ctx.common.provider_exchanges > 0
        && ctx.common.table_count == 18;
    Ok(Measured::new(passed, fields))
}
