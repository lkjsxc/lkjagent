use std::fs;
use std::path::Path;

pub fn long_novel_failure(workspace: &Path) -> Result<(), String> {
    let text = read_transcript(workspace)?;
    require_all(
        &text,
        &[
            "fixture=long-novel-active-run",
            "root=stories/long-novel-with-detailed-settings",
            "profile=NarrativeManuscript",
            "doc.audit content_readiness=failed",
            "weak_paths=28",
            "child_file_tags=refused",
            "schema_fault=unsupported child tags",
            "second_same_shape=artifact.next",
            "next_decision_required=true",
            "provider_anomaly=reasoning_only_response",
            "touched_paths=artifact-ledger-derived",
            "maintenance_noop=cooldown",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "touched_paths=none",
            "repeat child_file_tags",
            "provider_anomaly=parse_fault",
            "graph.evidence artifact-readiness=accepted",
            "agent.done=accepted_before_audit",
            "maintenance_noop=endpoint_churn",
        ],
    )
}

fn read_transcript(workspace: &Path) -> Result<String, String> {
    fs::read_to_string(workspace.join("transcript.md"))
        .map_err(|error| format!("transcript.md unreadable: {error}"))
}

fn require_all(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if !text.contains(needle) {
            return Err(format!("missing {needle}"));
        }
    }
    Ok(())
}

fn forbid_any(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if text.contains(needle) {
            return Err(format!("forbidden stale shape {needle}"));
        }
    }
    Ok(())
}
