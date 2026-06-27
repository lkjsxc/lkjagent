mod support;

use std::path::Path;

use lkjagent_graph::case_evidence::EvidenceRequirementState;
use lkjagent_graph::{
    completion::refresh_completion_state, initial_state, EvidenceKind, EvidenceRecord,
};
use lkjagent_protocol::render_action;
use lkjagent_runtime::daemon::{
    client_config, take_daemon_lock, DaemonTick, ResidentDaemon, ResidentRuntime,
};
use lkjagent_runtime::task::{PendingAction, TaskState};
use lkjagent_store::artifact_ledger::{upsert_artifact, ArtifactLedgerInput};
use lkjagent_store::graph::{open_case, OpenCase};
use lkjagent_store::state;
use support::http::serve_responses;
use support::{action, runtime_state, store, temp_workspace, TestResult};

#[test]
fn complete_graph_authority_closes_through_kernel_completion_event() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    open_case(&conn, stored_case(), "2026-01-01T00:00:00Z")?;
    upsert_artifact(&conn, &passed_artifact(), "2026-01-01T00:00:00Z")?;
    let workspace = temp_workspace("kernel-driver-completion")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 4 };
    daemon.state.graph = Some(complete_graph());
    let done = action("agent.done", &[("summary", "artifact complete")]);
    daemon.state.pending_action = Some(PendingAction {
        action: done.clone(),
        action_text: render_action(&done),
        authority_decision_id: None,
        prompt_frame_id: None,
        staleness_fingerprint: None,
    });

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Done);
    server.join()?;

    assert_eq!(
        state::get(&conn, "kernel mission")?,
        Some("owner_completion".to_string())
    );
    Ok(())
}

fn complete_graph() -> lkjagent_graph::TaskGraphState {
    let mut graph = initial_state("Finish the story artifact.", Some(1));
    if !graph.evidence.knows_requirement("artifact-readiness") {
        graph.evidence.requirements.push(EvidenceRequirementState {
            id: "artifact-readiness".to_string(),
            description: "artifact readiness evidence".to_string(),
            required_for_completion: true,
        });
    }
    for id in graph.evidence.requirement_ids() {
        graph.evidence.records.push(EvidenceRecord {
            requirement: id.clone(),
            kind: evidence_kind(&id),
            summary: format!("{id} satisfied"),
            path: Some("stories/chronos-fracture".to_string()),
            frame_ref: None,
            event_ref: None,
            confidence: 100,
            satisfies_completion: true,
        });
    }
    graph.evidence.pending_checks.clear();
    refresh_completion_state(&mut graph);
    graph
}

fn stored_case() -> OpenCase {
    OpenCase {
        objective: "Finish the story artifact.".to_string(),
        raw_owner_text: "Finish the story artifact.".to_string(),
        objective_version: 1,
        family: "documentation".to_string(),
        subroute: "story".to_string(),
        route_reason: "test".to_string(),
        phase: "execution".to_string(),
        active_node: "document".to_string(),
        plan: "test plan".to_string(),
        evidence_requirements: vec!["artifact-readiness".to_string()],
        selected_packages: Vec::new(),
        pending_checks: Vec::new(),
        next_action_class: "agent.done".to_string(),
        context_pressure: "green".to_string(),
    }
}

fn passed_artifact() -> ArtifactLedgerInput<'static> {
    ArtifactLedgerInput {
        case_id: 1,
        artifact_id: "1:story:chronos-fracture:unspecified",
        root: "stories/chronos-fracture",
        kind: "story",
        normalized_topic: "chronos-fracture",
        requested_scale: "unspecified",
        profile: "story",
        lifecycle_state: "content-ready",
        topology_status: "passed",
        readiness_status: "passed",
        objective_match_status: "passed",
        latest_audit_id: None,
        weak_path_count: 0,
    }
}

fn evidence_kind(requirement: &str) -> EvidenceKind {
    match requirement {
        "verification" => EvidenceKind::Verification,
        "artifact-readiness" | "document-structure" => EvidenceKind::File,
        "plan" => EvidenceKind::Plan,
        _ => EvidenceKind::Observation,
    }
}

fn daemon(base_url: &str, workspace: &Path) -> TestResult<ResidentDaemon> {
    let runtime = ResidentRuntime::new(
        "test".to_string(),
        client_config(base_url, "local-model", None, 180, 2_048),
        workspace.to_path_buf(),
        "100",
    );
    Ok(ResidentDaemon::new(runtime_state()?, runtime))
}
