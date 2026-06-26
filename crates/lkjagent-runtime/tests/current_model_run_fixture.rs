use std::path::Path;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn checked_in_current_model_run_is_empty_content_recovery_fixture() -> TestResult<()> {
    let root = repo_root();
    let current = CURRENT_MODEL_RUN_SIGNATURE;

    assert!(current.contains("active_node: recover-by-smaller-scope"));
    assert!(current.contains("active_phase: recovery"));
    assert!(current.contains("## Touched Paths\n\n* none"));
    assert!(current.contains("| none | none | none | none | low |"));
    assert!(current.contains("parse fault: missing action envelope"));

    let latest = root.join("data/logs/model/epoch-1782344195/case-1/turn-000019");
    let response = HISTORICAL_EMPTY_RESPONSE;
    let parsed = HISTORICAL_EMPTY_PARSED_ACTION;
    let export = HISTORICAL_EMPTY_EXPORT;

    assert!(response.contains("\"content\":\"\""));
    assert!(response.contains("\"completion_tokens\":485"));
    assert!(parsed.contains("\"error\":\"MissingActionEnvelope\""));
    assert!(export.contains("admission.json"));
    assert!(!latest.join("admission.json").exists());
    assert!(!latest.join("observation.txt").exists());
    Ok(())
}

const CURRENT_MODEL_RUN_SIGNATURE: &str = "active_node: recover-by-smaller-scope
active_phase: recovery
## Touched Paths

* none
| none | none | none | none | low |
parse fault: missing action envelope";

const HISTORICAL_EMPTY_RESPONSE: &str =
    "{\"content\":\"\",\"finish_reason\":\"stop\",\"closure_mode\":\"Unclosed\",\"usage\":{\"completion_tokens\":485}}";

const HISTORICAL_EMPTY_PARSED_ACTION: &str =
    "{\"status\":\"fault\",\"content_bytes\":0,\"error\":\"MissingActionEnvelope\"}";

const HISTORICAL_EMPTY_EXPORT: &str =
    "{\"status\":\"succeeded\",\"files\":[\"request.json\",\"admission.json\",\"observation.txt\"]}";

fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
