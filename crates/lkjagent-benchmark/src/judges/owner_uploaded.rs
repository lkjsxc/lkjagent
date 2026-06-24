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
            "fixture=japanese-cookbook-drift",
            "owner_input=Create a very big cookbook about japanese foods.",
            "artifact_kind=Cookbook",
            "subject=Japanese food",
            "bread_profile=rejected",
            "forbidden_bread_paths=absent",
            "fixture=document-structure-graph-evidence-bypass",
            "graph.evidence document-structure=refused",
            "next_action=doc.audit",
            "fixture=batch-write-payload-schema-fault",
            "fs.batch_write json_payload=refused",
            "canonical_grammar=line-block",
            "partial_write=absent",
            "fixture=shell-parameter-missing-command",
            "shell.run missing_command=refused",
            "schema_repair=command required",
            "invalid_timeout_retry=absent",
            "fixture=queue-story-interrupt",
            "case1 objective=cookbook",
            "case2 objective=japanese story",
            "cross_case_contamination=absent",
            "fixture=context-compaction-resume",
            "durable_snapshot=created",
            "post_compaction_check=passed",
            "missing_evidence=preserved",
            "last_refused_action=preserved",
            "fixture=missing-act-block",
            "parse_fault=MissingActBlock",
            "dispatch=absent",
            "recovery_route=RenderSingleExactActionExample",
            "fixture=empty-content-interrupted",
            "fault=InterruptedGeneration",
            "resume=last_durable_observation",
            "fixture=unclosed-act-from-stop",
            "closure_mode=StopSequenceClosed",
            "parse_repair=logged",
            "fixture=contradictory-authority",
            "allowed_tools_none_with_tool_action=impossible",
            "authority_recomputed=without_model",
            "fixture=provider-exchange-logging",
            "request_json=written",
            "response_json=written",
            "parsed_action=written",
            "admission_before_dispatch=required",
            "observation=written",
            "prompt_frame_id=stored",
            "replay_export=available",
            "fixture=repeated-recovery-action",
            "repeated_action_signature=blocked",
            "next_action=different_action_class",
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
            "graph.evidence kind=document-structure",
            "completion=ready",
            "bread_profile=selected",
            "path=ciabatta.md",
            "partial_write=present",
            "invalid_timeout_retry=present",
            "active_objective=overwritten",
            "cross_case_contamination=present",
            "post_compaction_check=skipped",
            "missing_evidence=lost",
            "empty assistant content",
            "dispatched graph.state",
            "parse_repair=silent",
            "allowed_tools=none",
            "preferred_next_action=graph.state",
            "request_json=missing",
            "response_json=missing",
            "prompt_frame_id=missing",
            "replay_export=missing",
            "repeat identical graph.recover",
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
