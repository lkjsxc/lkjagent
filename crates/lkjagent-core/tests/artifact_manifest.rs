use lkjagent_core::artifact_manifest::{
    nested_unit_paths, validate_manifest, ArtifactManifest, ArtifactManifestUnit,
};
use lkjagent_core::runtime_artifact::DEFAULT_UNIT_TARGET_TOKENS;

#[test]
fn artifact_manifest_supports_nested_units_and_generic_checks() -> Result<(), String> {
    let mut manifest = ArtifactManifest::new("artifact-1", "report", "Quarter", "reports/q1");
    manifest.source_records.push("record:rec_1".to_string());
    let mut unit = ArtifactManifestUnit::new("unit-1", "reports/q1/sections/intro.md");
    unit.required_source_refs.push("record:rec_1".to_string());
    unit.target_words = Some(350);
    manifest.units.push(unit);

    assert_eq!(manifest.units[0].target_tokens, DEFAULT_UNIT_TARGET_TOKENS);
    assert_eq!(
        nested_unit_paths(&manifest),
        vec!["reports/q1/sections/intro.md"]
    );
    assert!(validate_manifest(&manifest).is_empty());
    assert!(manifest
        .fingerprint()
        .map_err(|error| error.message)?
        .starts_with("fnv1a64:"));
    Ok(())
}

#[test]
fn artifact_manifest_rejects_placeholders_missing_refs_and_bad_paths() {
    let mut manifest = ArtifactManifest::new("artifact-1", "report", "Quarter", "reports/q1");
    let mut unit = ArtifactManifestUnit::new("unit-1", "../TODO.md");
    unit.checks.clear();
    manifest.units.push(unit);

    let messages = validate_manifest(&manifest)
        .into_iter()
        .map(|issue| issue.message)
        .collect::<Vec<_>>();
    assert!(messages
        .iter()
        .any(|message| message.contains("workspace path escapes")));
    assert!(messages
        .iter()
        .any(|message| message == "source refs missing"));
    assert!(messages.iter().any(|message| message == "checks missing"));
    assert!(messages.iter().any(|message| message == "placeholder path"));
}
