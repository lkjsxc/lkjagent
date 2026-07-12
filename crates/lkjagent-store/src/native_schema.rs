use std::path::Path;

use rusqlite::Connection;

use crate::error::{StoreError, StoreResult};

pub const NATIVE_SCHEMA_VERSION: i64 = 1;
pub const NATIVE_TABLES: &[&str] = &[
    "checks",
    "config_fingerprints",
    "context_items",
    "conversation_messages",
    "daemon_leases",
    "effect_journal",
    "effect_targets",
    "matters",
    "obligations",
    "observations",
    "owner_turns",
    "provider_exchanges",
    "runtime_decisions",
    "runtime_events",
    "state_cells",
    "tool_admissions",
    "workspace_documents",
    "workspace_revisions",
];

pub fn open(path: impl AsRef<Path>) -> StoreResult<Connection> {
    let connection = Connection::open(path)?;
    setup(&connection)?;
    Ok(connection)
}

pub fn setup(connection: &Connection) -> StoreResult<()> {
    reject_nonempty(connection)?;
    connection.execute_batch(
        "PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL;
         PRAGMA synchronous=FULL; PRAGMA busy_timeout=5000; BEGIN EXCLUSIVE;",
    )?;
    let result = reject_nonempty(connection).and_then(|()| create_schema(connection));
    match result {
        Ok(()) => connection.execute_batch("COMMIT;").map_err(Into::into),
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK;");
            Err(error)
        }
    }
}

fn reject_nonempty(connection: &Connection) -> StoreResult<()> {
    let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let mut statement = connection
        .prepare("SELECT name FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%' ORDER BY name")?;
    let names = statement
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    if version != 0 || !names.is_empty() {
        return Err(StoreError::IncompatibleSchema {
            version,
            objects: names,
        });
    }
    Ok(())
}

fn create_schema(connection: &Connection) -> StoreResult<()> {
    connection.execute_batch(SCHEMA)?;
    connection.pragma_update(None, "user_version", NATIVE_SCHEMA_VERSION)?;
    Ok(())
}

const SCHEMA: &str = r#"
CREATE TABLE matters (
 id TEXT PRIMARY KEY, objective BLOB NOT NULL, lifecycle TEXT NOT NULL CHECK(lifecycle IN ('open','blocked','closed')),
 priority INTEGER NOT NULL, created_sequence INTEGER NOT NULL, updated_sequence INTEGER NOT NULL,
 closure_event_id TEXT, closure_checks_passed INTEGER NOT NULL DEFAULT 0 CHECK(closure_checks_passed IN (0,1)),
 unsettled_effects INTEGER NOT NULL DEFAULT 0 CHECK(unsettled_effects>=0),
 CHECK(lifecycle!='closed' OR (closure_event_id IS NOT NULL AND closure_checks_passed=1 AND unsettled_effects=0)),
 UNIQUE(id,lifecycle), UNIQUE(closure_event_id,id),
 FOREIGN KEY(closure_event_id,id) REFERENCES runtime_events(id,matter_id) DEFERRABLE INITIALLY DEFERRED);
CREATE TABLE owner_turns (
 id TEXT PRIMARY KEY, queue_sequence INTEGER NOT NULL UNIQUE, raw_text BLOB NOT NULL,
 delivery TEXT NOT NULL CHECK(delivery IN ('queued','delivered','failed')), matter_id TEXT REFERENCES matters(id), created_at TEXT NOT NULL);
CREATE TABLE conversation_messages (
 id TEXT PRIMARY KEY, sequence INTEGER NOT NULL UNIQUE, role TEXT NOT NULL CHECK(role IN ('owner','agent')),
 body BLOB NOT NULL, body_fingerprint BLOB NOT NULL, lifecycle TEXT NOT NULL CHECK(lifecycle IN ('active','replaced','withdrawn')),
 cause_event_id TEXT, replacement_id TEXT UNIQUE REFERENCES conversation_messages(id));
CREATE TABLE obligations (
 id TEXT PRIMARY KEY, matter_id TEXT NOT NULL REFERENCES matters(id), predicate_kind TEXT NOT NULL, predicate_payload BLOB NOT NULL,
 required INTEGER NOT NULL CHECK(required IN (0,1)), status TEXT NOT NULL CHECK(status IN ('open','passed','invalidated')),
 current_check_id TEXT, invalidating_event_id TEXT, UNIQUE(id,matter_id), UNIQUE(current_check_id,id),
 CHECK(status!='passed' OR current_check_id IS NOT NULL),
 FOREIGN KEY(current_check_id,id) REFERENCES checks(id,obligation_id) DEFERRABLE INITIALLY DEFERRED);
CREATE TABLE runtime_events (
 id TEXT PRIMARY KEY, matter_id TEXT NOT NULL REFERENCES matters(id), causal_sequence INTEGER NOT NULL,
 kind TEXT NOT NULL, monotonic_ms INTEGER NOT NULL CHECK(monotonic_ms>=0), wall_time TEXT NOT NULL, payload BLOB NOT NULL,
 source_kind TEXT NOT NULL, source_id TEXT NOT NULL, UNIQUE(matter_id,causal_sequence), UNIQUE(id,matter_id));
CREATE TABLE state_cells (
 matter_id TEXT NOT NULL REFERENCES matters(id), namespace BLOB NOT NULL, cell_key BLOB NOT NULL, payload BLOB NOT NULL,
 status TEXT NOT NULL CHECK(status IN ('active','suppressed','expired')), source_event_id TEXT NOT NULL REFERENCES runtime_events(id),
 fingerprint BLOB NOT NULL, PRIMARY KEY(matter_id,namespace,cell_key));
CREATE TABLE runtime_decisions (
 id TEXT PRIMARY KEY, matter_id TEXT NOT NULL REFERENCES matters(id), event_id TEXT NOT NULL UNIQUE REFERENCES runtime_events(id),
 operation_key BLOB NOT NULL, idempotency_key BLOB NOT NULL UNIQUE, selected_monotonic_ms INTEGER NOT NULL CHECK(selected_monotonic_ms>=0),
 selected_state BLOB NOT NULL, context_spec BLOB NOT NULL, tool_spec BLOB NOT NULL, grammar_spec BLOB NOT NULL,
 budget_spec BLOB NOT NULL, recovery_spec BLOB NOT NULL, exit_spec BLOB NOT NULL, compiler_status TEXT NOT NULL CHECK(compiler_status IN ('compiled','rejected')),
 compiler_attachments BLOB NOT NULL, status TEXT NOT NULL CHECK(status IN ('selected','admitted','running','settled','failed')),
 settlement_event_id TEXT UNIQUE REFERENCES runtime_events(id), UNIQUE(matter_id,operation_key));
CREATE TRIGGER immutable_decision_spec BEFORE UPDATE OF matter_id,event_id,operation_key,idempotency_key,selected_monotonic_ms,selected_state,context_spec,tool_spec,grammar_spec,budget_spec,recovery_spec,exit_spec,compiler_status,compiler_attachments ON runtime_decisions BEGIN SELECT RAISE(ABORT,'immutable decision specification'); END;
CREATE TABLE provider_exchanges (
 id TEXT PRIMARY KEY, decision_id TEXT NOT NULL UNIQUE REFERENCES runtime_decisions(id), request_ref BLOB NOT NULL,
 response_ref BLOB, input_tokens INTEGER CHECK(input_tokens>=0), output_tokens INTEGER CHECK(output_tokens>=0),
 started_monotonic_ms INTEGER NOT NULL CHECK(started_monotonic_ms>=0), finished_monotonic_ms INTEGER,
 status TEXT NOT NULL CHECK(status IN ('started','succeeded','failed')), CHECK(finished_monotonic_ms IS NULL OR finished_monotonic_ms>=started_monotonic_ms));
CREATE TABLE context_items (
 id TEXT PRIMARY KEY, decision_id TEXT NOT NULL REFERENCES runtime_decisions(id), source_kind TEXT NOT NULL, source_id BLOB NOT NULL,
 source_revision BLOB NOT NULL, semantic_key BLOB NOT NULL, trust TEXT NOT NULL CHECK(trust IN ('owner','workspace','tool','provider')),
 body_ref BLOB NOT NULL, UNIQUE(decision_id,source_kind,source_id,source_revision,semantic_key));
CREATE TABLE daemon_leases (lease_name TEXT PRIMARY KEY, owner_id TEXT NOT NULL, acquired_at TEXT NOT NULL, heartbeat_at TEXT NOT NULL, expires_at TEXT NOT NULL);
CREATE TABLE config_fingerprints (id TEXT PRIMARY KEY, effective_fingerprint BLOB NOT NULL UNIQUE, recorded_event_id TEXT NOT NULL REFERENCES runtime_events(id), created_at TEXT NOT NULL);
CREATE TABLE tool_admissions (
 id TEXT PRIMARY KEY, decision_id TEXT NOT NULL REFERENCES runtime_decisions(id), action_ordinal INTEGER NOT NULL CHECK(action_ordinal>=0),
 action_fingerprint BLOB NOT NULL, origin TEXT NOT NULL CHECK(origin IN ('model','harness')), effectful INTEGER NOT NULL CHECK(effectful IN (0,1)),
 status TEXT NOT NULL CHECK(status IN ('accepted','rejected')), reason BLOB NOT NULL, parsed_call BLOB NOT NULL, tool_spec BLOB NOT NULL,
 journal_id TEXT UNIQUE, UNIQUE(decision_id,action_ordinal), UNIQUE(decision_id,action_fingerprint), UNIQUE(id,journal_id),
 CHECK((status='accepted' AND effectful=1 AND journal_id IS NOT NULL) OR (journal_id IS NULL AND (status='rejected' OR effectful=0))));
CREATE TABLE effect_journal (
 id TEXT PRIMARY KEY, admission_id TEXT NOT NULL UNIQUE, decision_id TEXT NOT NULL REFERENCES runtime_decisions(id),
 command_ordinal INTEGER NOT NULL CHECK(command_ordinal>=0), idempotency_key BLOB NOT NULL UNIQUE,
 status TEXT NOT NULL CHECK(status IN ('admitted','staged','attempted','settled','failed')),
 intended_fingerprint BLOB NOT NULL, prior_fingerprint BLOB, outcome_fingerprint BLOB, observation_id TEXT UNIQUE,
 CHECK((status IN ('attempted','settled','failed') AND observation_id IS NOT NULL) OR (status IN ('admitted','staged') AND observation_id IS NULL)),
 FOREIGN KEY(admission_id,id) REFERENCES tool_admissions(id,journal_id) DEFERRABLE INITIALLY DEFERRED,
 UNIQUE(admission_id,id), UNIQUE(id,observation_id));
CREATE TABLE effect_targets (
 journal_id TEXT NOT NULL REFERENCES effect_journal(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), normalized_path BLOB NOT NULL,
 prior_bytes BLOB, intended_bytes BLOB, mode TEXT NOT NULL CHECK(mode IN ('create','replace','delete')),
 stage_identity BLOB NOT NULL, PRIMARY KEY(journal_id,ordinal), UNIQUE(journal_id,normalized_path));
CREATE TABLE observations (
 id TEXT PRIMARY KEY, journal_id TEXT NOT NULL UNIQUE, status TEXT NOT NULL CHECK(status IN ('succeeded','failed','unknown')),
 attempt_outcome BLOB NOT NULL, content_ref BLOB NOT NULL, fingerprint BLOB NOT NULL, contamination TEXT NOT NULL CHECK(contamination IN ('clean','untrusted')),
 FOREIGN KEY(journal_id,id) REFERENCES effect_journal(id,observation_id) DEFERRABLE INITIALLY DEFERRED, UNIQUE(journal_id,id));
CREATE TABLE checks (
 id TEXT PRIMARY KEY, matter_id TEXT NOT NULL, obligation_id TEXT NOT NULL, decision_id TEXT NOT NULL REFERENCES runtime_decisions(id),
 kind TEXT NOT NULL, parameters BLOB NOT NULL, current INTEGER NOT NULL CHECK(current IN (0,1)), passed INTEGER NOT NULL CHECK(passed IN (0,1)),
 measured BLOB NOT NULL, evidence_fingerprint BLOB NOT NULL, source_revision BLOB NOT NULL, checked_event_id TEXT NOT NULL REFERENCES runtime_events(id),
 FOREIGN KEY(obligation_id,matter_id) REFERENCES obligations(id,matter_id), UNIQUE(obligation_id,source_revision,kind), UNIQUE(id,obligation_id));
CREATE UNIQUE INDEX one_current_check_per_obligation ON checks(obligation_id) WHERE current=1;
CREATE TABLE workspace_documents (
 id TEXT PRIMARY KEY, current_path BLOB NOT NULL UNIQUE, status TEXT NOT NULL CHECK(status IN ('active','closed','deleted')),
 managed INTEGER NOT NULL CHECK(managed IN (0,1)), current_revision_id TEXT, UNIQUE(current_revision_id,id),
 FOREIGN KEY(current_revision_id,id) REFERENCES workspace_revisions(id,document_id) DEFERRABLE INITIALLY DEFERRED);
CREATE TABLE workspace_revisions (
 id TEXT PRIMARY KEY, document_id TEXT NOT NULL REFERENCES workspace_documents(id), parent_id TEXT REFERENCES workspace_revisions(id),
 sha256 BLOB NOT NULL CHECK(length(sha256)=32), content BLOB NOT NULL, effect_id TEXT NOT NULL REFERENCES effect_journal(id),
 created_event_id TEXT NOT NULL REFERENCES runtime_events(id), UNIQUE(document_id,sha256), UNIQUE(id,document_id),
 FOREIGN KEY(parent_id,document_id) REFERENCES workspace_revisions(id,document_id));
"#;
