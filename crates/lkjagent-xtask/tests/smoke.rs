use std::path::Path;

#[test]
fn deterministic_smoke_summary_names_required_fields() -> Result<(), String> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "missing workspace root".to_string())?;
    let summary = lkjagent_xtask::smoke::replay_summary(root)?;

    for expected in [
        "case=missing-root-loop",
        "case=generic-root",
        "case=false-close",
        "case=provider-anomaly",
        "case=manuscript-incomplete",
        "decision_ids=",
        "root=",
        "paths=",
        "word_count=",
        "completion_gate=",
        "token_aggregate=",
    ] {
        assert!(summary.contains(expected), "missing {expected}: {summary}");
    }
    Ok(())
}
