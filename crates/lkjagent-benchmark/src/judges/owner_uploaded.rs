use std::fs;
use std::path::Path;

pub fn uploaded_run_fixtures(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "fixture=recover-repeat-parameter-fault",
            "schema_repair=one canonical example",
            "next_action=graph.recover",
            "fixture=bread-dictionary-shallow-content",
            "artifact_kind=Dictionary",
            "content_readiness=failed",
            "repair_admitted=artifact.next,fs.batch_write",
            "fixture=large-write-payload-risk",
            "payload_too_large=blocked raw fs.write",
            "next_action=fs.batch_write",
            "fixture=completion-with-blocked-mutation",
            "mission=Repair",
            "mutation_tools=admitted",
            "fixture=maintenance-during-owner-work",
            "maintenance=yielded",
            "memory_loop=absent",
            "fixture=cookbook-scaffold-false-ready",
            "structure_audit=passed",
            "agent.done=refused",
            "fixture=artifact-readiness-graph-evidence-bypass",
            "graph.evidence artifact-readiness=refused",
            "next_action=artifact.audit",
        ],
    )?;
    if text.matches("completion=refused").count() < 2 {
        return Err("missing repeated completion refusal evidence".to_string());
    }
    if text.matches("content_readiness=failed").count() < 2 {
        return Err("missing content readiness failures".to_string());
    }
    forbid_any(
        &text,
        &[
            "agent.done complete",
            "retry raw fs.write",
            "mutation_tools=blocked",
            "empty maintenance cycle",
            "content_readiness=passed",
            "graph.evidence kind=artifact-readiness",
            "completion=ready",
        ],
    )
}

fn read_any(workspace: &Path) -> Result<String, String> {
    for path in ["transcript.md", "run.log"] {
        let candidate = workspace.join(path);
        if candidate.is_file() {
            return fs::read_to_string(&candidate)
                .map_err(|error| format!("{} unreadable: {error}", candidate.display()));
        }
    }
    Err("none of transcript.md or run.log exists".to_string())
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
