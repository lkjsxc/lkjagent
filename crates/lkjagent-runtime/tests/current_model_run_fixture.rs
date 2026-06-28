use std::path::{Path, PathBuf};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn checked_in_current_model_run_is_long_novel_failure_fixture() -> TestResult<()> {
    let current = include_str!("../../../data/logs/current-model-run.md");

    assert!(current.contains("active_node: document"));
    assert!(current.contains("active_phase: execution"));
    assert!(current.contains("Create a long novel. with detailed structured settings."));
    assert!(current.contains("stories/novel"));
    assert!(current.contains("no active graph case"));
    assert!(current.contains("authority refused fs.mkdir"));
    assert!(current.contains("fs.batch_write"));
    assert!(current.contains("document audit failed"));
    assert!(current.contains("document audit passed"));
    assert!(current.contains("invalid parameter: audit-owned graph evidence requirement"));
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

#[test]
fn checked_in_model_log_index_points_to_present_turn_dirs() -> TestResult<()> {
    let root = repo_root();
    let index = std::fs::read_to_string(root.join("data/logs/index.ndjson"))?;
    let mut checked = 0;
    for line in index.lines().filter(|line| !line.trim().is_empty()) {
        let path = index_path(line).ok_or("missing index path")?;
        let full = root.join(path);
        assert!(full.is_dir(), "missing turn dir: {}", full.display());
        checked += 1;
    }
    assert!(checked > 0);
    Ok(())
}

fn index_path(line: &str) -> Option<PathBuf> {
    let marker = "\"path\":\"";
    let (_, rest) = line.split_once(marker)?;
    let (value, _) = rest.split_once('"')?;
    if let Some(relative) = value.strip_prefix("/") {
        return Some(PathBuf::from(relative));
    }
    if value.starts_with("data/logs/") {
        return Some(PathBuf::from(value));
    }
    Some(Path::new("data/logs").join(value))
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
