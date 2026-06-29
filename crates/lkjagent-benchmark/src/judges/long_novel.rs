use std::fs;
use std::path::Path;

pub fn long_novel_failure(workspace: &Path) -> Result<(), String> {
    let text = read_transcript(workspace)?;
    require_all(
        &text,
        &[
            "fixture=long-novel-active-run",
            "root_alias=short-semantic",
            "root_segment_max=24",
            "stale_objective_root=absent",
            "profile=NarrativeManuscript",
            "doc.audit content_readiness=failed",
            "weak_paths=28",
            "batch_limit=refused",
            "schema_fault=too many files",
            "second_same_shape=artifact.next",
            "next_decision_required=true",
            "provider_anomaly=reasoning_only_response",
            "touched_paths=artifact-ledger-derived",
            "compact_title_route=content-artifact",
            "owner_title_root=stories/compact-compass",
            "iwanna_root=stories/iwanna",
            "generic_root=absent",
            "candidate_action_artifact_audit=respected",
            "story_scale_missing=profile-scale-content-groups",
            "maintenance_noop=cooldown",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "touched_paths=none",
            "repeat oversized_batch",
            "provider_anomaly=parse_fault",
            "graph.evidence artifact-readiness=accepted",
            "agent.done=accepted_before_audit",
            "maintenance_noop=endpoint_churn",
            "compact_title_route=compaction",
            "owner_title_root=stories/novel-named",
            "iwanna_root=stories/novel-named",
            "generic_root=stories/example-story",
            "candidate_action_artifact_audit=converted-to-batch",
            "root=stories/long-novel-with-",
            "stale_objective_root=present",
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
