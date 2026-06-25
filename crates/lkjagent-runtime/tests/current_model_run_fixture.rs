use std::fs;
use std::path::{Path, PathBuf};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn checked_in_current_model_run_is_empty_content_recovery_fixture() -> TestResult<()> {
    let root = repo_root();
    let current = fs::read_to_string(root.join("data/logs/current-model-run.md"))?;

    assert!(current.contains("active_node: recover-by-smaller-scope"));
    assert!(current.contains("active_phase: recovery"));
    assert!(current.contains("## Touched Paths\n\n* none"));
    assert!(current.contains("| none | none | none | none | low |"));
    assert!(current.contains("parse fault: missing action envelope"));

    let latest = root.join("data/logs/model/epoch-1782344195/case-1/turn-000019");
    let response = fs::read_to_string(latest.join("response.json"))?;
    let parsed = fs::read_to_string(latest.join("parsed-action.json"))?;
    let export = fs::read_to_string(latest.join("export.json"))?;

    assert!(response.contains("\"content\":\"\""));
    assert!(response.contains("\"completion_tokens\":485"));
    assert!(parsed.contains("\"error\":\"MissingActionEnvelope\""));
    assert!(export.contains("admission.json"));
    assert!(!latest.join("admission.json").exists());
    assert!(!latest.join("observation.txt").exists());
    Ok(())
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
