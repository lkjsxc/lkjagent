use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_effects::workspace_edit::{DurablePhase, Revision, VerifiedOutcome};
use lkjagent_store::transactions::{
    Decision, Effect, FinalClose, Intake, NativeStore, Obligation, Settlement, Target,
};
use rusqlite::Connection;
use sha2::{Digest, Sha256};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn settled_edit(name: &str) -> TestResult<(PathBuf, OpenedWorkspace)> {
    settled_effect(name, false)
}
pub fn settled_create(name: &str) -> TestResult<(PathBuf, OpenedWorkspace)> {
    settled_effect(name, true)
}
fn settled_effect(name: &str, create: bool) -> TestResult<(PathBuf, OpenedWorkspace)> {
    let root = super::automatic_checks_root(name)?;
    let workspace_root = root.join("workspace");
    fs::create_dir_all(workspace_root.join("notes"))?;
    if !create {
        fs::write(workspace_root.join("notes/a.md"), "alpha is current\n")?;
    }
    let workspace = OpenedWorkspace::open(&workspace_root)?;
    let (revision, old, new) = if create {
        (Revision::Absent, "", "beta is current\n")
    } else {
        let observed = workspace
            .observe_edit_target("notes/a.md")
            .map_err(edit_error)?;
        let lkjagent_effects::workspace_edit::ObservedTarget::Present(value) = observed else {
            return Err("seed file missing".into());
        };
        (Revision::Sha256(value.revision), "alpha", "beta")
    };
    let edit = workspace
        .prepare_exact_edit("notes/a.md".into(), revision, old, new, 0o644)
        .map_err(edit_error)?;
    let digest = Sha256::digest(&edit.intended_bytes);
    let byte = serde_json::json!({"path":"notes/a.md","sha256":hex(&digest)}).to_string();
    let content = serde_json::json!({"path":"notes/a.md","old":old,"new":new,
        "old_count":0,"new_count":1})
    .to_string();
    let collateral = serde_json::json!({"allowed_paths":["notes/a.md"]}).to_string();
    let obligations = [
        Obligation("byte", "workspace-byte", byte.as_bytes(), true),
        Obligation("content", "workspace-content", content.as_bytes(), true),
        Obligation(
            "collateral",
            "workspace-collateral",
            collateral.as_bytes(),
            true,
        ),
    ];
    let db = root.join("native.sqlite3");
    let mut store = NativeStore::open(&db)?;
    store.owner_intake(&Intake {
        matter: "m",
        objective: b"replace alpha",
        turn: "t",
        queue_sequence: 1,
        raw_text: b"edit",
        message_fingerprint: b"owner-fp",
        event: "e1",
        event_sequence: 1,
        event_payload: b"intake",
        monotonic_ms: 1,
        wall_time: "now",
        obligations: &obligations,
        cells: &[],
    })?;
    store.select_decision(&Decision {
        id: "d",
        matter: "m",
        event: "e2",
        event_sequence: 2,
        event_payload: b"selected",
        operation: b"effect.workspace.edit",
        idempotency: b"decision-idem",
        monotonic_ms: 2,
        wall_time: "now",
        specs: [
            b"state",
            b"context",
            b"tool",
            b"grammar",
            b"budget",
            b"recovery",
            b"automatic-checks",
            b"checked-close",
        ],
    })?;
    store.attach_compilation("d", b"attached", b"frame", b"context", b"tool", &[])?;
    let target = Target {
        path: b"notes/a.md",
        prior: edit.prior_bytes.as_deref(),
        intended: Some(&edit.intended_bytes),
        operation: if create { "create" } else { "replace" },
        prior_mode: edit.expected_mode.map(i64::from),
        intended_mode: Some(i64::from(edit.intended_mode)),
        stage_identity: edit.stage_identity.as_bytes(),
    };
    store.prepare_effect(&Effect {
        admission: "a",
        journal: "j",
        decision: "d",
        action_ordinal: 0,
        action_fingerprint: b"action",
        reason: b"admitted",
        parsed_call: b"edit",
        tool_spec: b"tool",
        idempotency: b"effect-idem",
        intended_fingerprint: &digest,
        prior_fingerprint: None,
        targets: &[target],
    })?;
    phase(&mut store, &workspace, &edit)?;
    store.settle_effect(&Settlement {
        journal: "j",
        observation: "obs",
        event: "e3",
        matter: "m",
        event_sequence: 3,
        monotonic_ms: 3,
        wall_time: "now",
        event_payload: b"committed edit",
        status: "succeeded",
        outcome: b"written",
        content_ref: b"rev",
        fingerprint: &digest,
        document: "doc",
        path: b"notes/a.md",
        revision: "rev",
        parent: None,
        sha256: &digest,
        content: &edit.intended_bytes,
    })?;
    workspace
        .cleanup_exact_edit(&edit, VerifiedOutcome::Settled)
        .map_err(edit_error)?;
    Ok((root, workspace))
}

fn phase(
    store: &mut NativeStore,
    workspace: &OpenedWorkspace,
    edit: &lkjagent_effects::workspace_edit::PreparedEdit,
) -> TestResult<()> {
    store.effect_phase("j", "prepared", "staging")?;
    workspace
        .advance_exact_edit(edit, DurablePhase::Staged)
        .map_err(edit_error)?;
    store.effect_phase("j", "staging", "exchange-ready")?;
    store.effect_phase("j", "exchange-ready", "exchanging")?;
    workspace
        .advance_exact_edit(edit, DurablePhase::Exchanged)
        .map_err(edit_error)?;
    store.effect_phase("j", "exchanging", "exchanged")?;
    workspace
        .advance_exact_edit(edit, DurablePhase::Settled)
        .map_err(edit_error)?;
    store.effect_phase("j", "exchanged", "observing")?;
    Ok(())
}

pub fn close(db: &Path) -> lkjagent_store::error::StoreResult<()> {
    NativeStore::open(db)?
        .close_matter(&FinalClose {
            matter: "m",
            body: b"done",
            body_fingerprint: b"final-fp",
            event: "close",
            event_sequence: 10,
            monotonic_ms: 10,
            wall_time: "now",
            payload: b"close",
        })
        .map(|_| ())
}
pub fn scalar(connection: &Connection, sql: &str) -> TestResult<i64> {
    Ok(connection.query_row(sql, [], |row| row.get(0))?)
}
pub fn text(connection: &Connection, sql: &str) -> TestResult<String> {
    Ok(connection.query_row(sql, [], |row| row.get(0))?)
}
fn edit_error(error: impl std::fmt::Debug) -> std::io::Error {
    std::io::Error::other(format!("{error:?}"))
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
