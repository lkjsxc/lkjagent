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
use support::http::serve_responses;
use support::{action, runtime_state, store, temp_workspace, TestResult};

#[test]
fn agent_done_refuses_when_artifact_ledger_has_weak_paths() -> TestResult<()> {
    let mut conn = store()?;
    take_daemon_lock(&conn, "test", "100", "0")?;
    let graph =
        lkjagent_runtime::graph_state::open_owner_case(&conn, "Finish cookbook artifact.", "100")?;
    let case_id = graph.case_id.ok_or("missing case id")?;
    let ids = artifact_evidence_ids(&graph.evidence.requirement_ids());
    record_all_evidence(&conn, case_id, &ids)?;
    conn.execute(
        "UPDATE graph_cases SET evidence_requirements = ?2, pending_checks = '' WHERE id = ?1",
        (case_id, ids.join("\n")),
    )?;
    upsert_artifact(&conn, &weak_artifact(), "2026-01-01T00:00:00Z")?;
    let workspace = temp_workspace("artifact-ledger-completion")?;
    let server = serve_responses(Vec::new())?;
    let mut daemon = daemon(&server.base_url, &workspace)?;
    daemon.state.task = TaskState::Open { turns_remaining: 4 };
    daemon.state.graph = Some(complete_graph_with_artifact_evidence());
    let done = action("agent.done", &[("summary", "artifact complete")]);
    daemon.state.pending_action = Some(PendingAction {
        action: done.clone(),
        action_text: render_action(&done),
        authority_decision_id: None,
        prompt_frame_id: None,
        staleness_fingerprint: None,
    });

    assert_eq!(daemon.poll_once(&mut conn, "101")?, DaemonTick::Working);
    server.join()?;

    assert!(daemon.state.pending_action.is_none());
    assert!(matches!(daemon.state.task, TaskState::Open { .. }));
    let log = daemon
        .state
        .context
        .log
        .iter()
        .map(|frame| frame.content.as_str())
        .collect::<Vec<_>>()
        .join("\n-- frame --\n");
    assert!(
        log.contains("authority refused agent.done") && log.contains("artifact-readiness"),
        "log was:\n{log}"
    );
    Ok(())
}

fn artifact_evidence_ids(ids: &[String]) -> Vec<String> {
    let mut values = ids.to_vec();
    if !values.iter().any(|id| id == "artifact-readiness") {
        values.push("artifact-readiness".to_string());
    }
    values
}

fn record_all_evidence(
    conn: &rusqlite::Connection,
    case_id: i64,
    ids: &[String],
) -> TestResult<()> {
    for id in ids {
        lkjagent_store::graph::record_evidence(
            conn,
            case_id,
            &lkjagent_store::graph::GraphEvidenceRow {
                requirement: id.clone(),
                kind: evidence_kind(id).as_str().to_string(),
                summary: format!("{id} satisfied"),
                path: Some("cookbooks/bread".to_string()),
            },
            "100",
        )?;
    }
    Ok(())
}

fn complete_graph_with_artifact_evidence() -> lkjagent_graph::TaskGraphState {
    let mut graph = initial_state("Finish cookbook artifact.", Some(1));
    if !graph.evidence.knows_requirement("artifact-readiness") {
        graph.evidence.requirements.push(EvidenceRequirementState {
            id: "artifact-readiness".to_string(),
            description: "artifact readiness evidence".to_string(),
            required_for_completion: true,
        });
    }
    let ids = graph.evidence.requirement_ids();
    for id in ids {
        graph.evidence.records.push(EvidenceRecord {
            requirement: id.clone(),
            kind: evidence_kind(&id),
            summary: format!("{id} satisfied"),
            path: Some("cookbooks/bread".to_string()),
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

fn weak_artifact() -> ArtifactLedgerInput<'static> {
    ArtifactLedgerInput {
        case_id: 1,
        artifact_id: "1:cookbook:bread:unspecified",
        root: "cookbooks/bread",
        kind: "cookbook",
        normalized_topic: "bread",
        requested_scale: "unspecified",
        profile: "cookbook",
        lifecycle_state: "content-partial",
        topology_status: "unknown",
        readiness_status: "failed",
        objective_match_status: "unknown",
        latest_audit_id: None,
        weak_path_count: 1,
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
