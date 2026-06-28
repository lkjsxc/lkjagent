use std::path::Path;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn checked_in_current_model_run_is_long_novel_repair_fixture() -> TestResult<()> {
    let current = include_str!("../../../data/logs/current-model-run.md");

    assert!(current.contains("active_node: document"));
    assert!(current.contains("active_phase: execution"));
    assert!(current.contains("Create a long novel. with structured settings."));
    assert!(current.contains("stories/long-novel-with-structured-settings"));
    assert!(current.contains("document scaffold created"));
    assert!(current.contains("profile=NarrativeManuscript"));
    assert!(current.contains("document audit failed"));
    assert!(current.contains("structure_only_content: project/premise.md"));
    assert!(current.contains("invalid parameter: too many files; max=20"));
    assert!(current.contains("provider anomaly: reasoning_only_response"));
    assert!(current.contains("| document audit | pending | graph case check |"));
    assert!(current.contains("| artifact readiness audit | pending | graph case check |"));
    Ok(())
}

#[test]
fn historical_empty_content_turn_remains_provider_fixture() -> TestResult<()> {
    let latest = repo_root().join("data/logs/model/epoch-1782344195/case-1/turn-000019");
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

const HISTORICAL_EMPTY_RESPONSE: &str =
    "{\"content\":\"\",\"finish_reason\":\"stop\",\"closure_mode\":\"Unclosed\",\"usage\":{\"completion_tokens\":485}}";

const HISTORICAL_EMPTY_PARSED_ACTION: &str =
    "{\"status\":\"fault\",\"content_bytes\":0,\"error\":\"MissingActionEnvelope\"}";

const HISTORICAL_EMPTY_EXPORT: &str =
    "{\"status\":\"succeeded\",\"files\":[\"request.json\",\"admission.json\",\"observation.txt\"]}";

fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
