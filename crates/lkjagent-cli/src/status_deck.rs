#[path = "status_artifact.rs"]
mod status_artifact;
#[path = "status_context.rs"]
mod status_context;
#[path = "status_facts.rs"]
mod status_facts;

use std::path::Path;

use lkjagent_context::budget::ContextBudgetPolicy;
use rusqlite::Connection;

use crate::accounting::{self, AccountingDeck};
use crate::config::load_context_policy_for_status;
use crate::error::CliError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusFact {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusDeck {
    pub facts: Vec<StatusFact>,
    pub daemon_state: String,
    pub state_label: String,
    pub pending: usize,
    pub open_task: String,
    pub turns: String,
    pub active_states: String,
    pub accounting: AccountingDeck,
    pub model_log: String,
    pub question: String,
    pub error: String,
    pub next_action: String,
}

pub fn load(data_dir: &Path, conn: &Connection) -> Result<StatusDeck, CliError> {
    let pending = pending_count(conn)?;
    let active = lkjagent_store::graph::active_case(conn)?;
    let policy = load_context_policy_for_status(data_dir)?;
    let accounting = accounting::deck(conn, policy)?;
    let values = CoreValues::read(data_dir, conn, active.as_ref().map(|row| row.id))?;
    let mut facts = Vec::new();
    status_facts::push_all(
        conn,
        &mut facts,
        active.as_ref(),
        &values,
        &accounting,
        policy,
    )?;
    Ok(StatusDeck {
        facts,
        daemon_state: values.daemon_state.clone(),
        state_label: state_label(&values.daemon_state).to_string(),
        pending,
        open_task: values.open_task,
        turns: values.turns,
        active_states: values.active_states,
        accounting,
        model_log: values.model_log,
        question: values.question,
        error: values.error,
        next_action: values.next_action,
    })
}

pub fn render_status(deck: &StatusDeck) -> String {
    deck.facts
        .iter()
        .map(|fact| format!("{}={}", fact.key, fact.value))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) struct CoreValues {
    pub daemon_state: String,
    pub pending: usize,
    pub open_task: String,
    pub turns: String,
    pub active_states: String,
    pub model_log: String,
    pub question: String,
    pub error: String,
    pub next_action: String,
}

impl CoreValues {
    fn read(data_dir: &Path, conn: &Connection, case_id: Option<i64>) -> Result<Self, CliError> {
        Ok(Self {
            daemon_state: status_facts::state_value(conn, "daemon state", "stopped")?,
            pending: pending_count(conn)?,
            open_task: status_facts::state_value(conn, "open task", "none")?,
            turns: status_facts::state_value(conn, "turn", "0")?,
            active_states: status_facts::active_states(conn, case_id)?,
            model_log: lkjagent_runtime::model_log::current_log_path(data_dir)
                .to_string_lossy()
                .to_string(),
            question: status_facts::state_value(conn, "daemon question", "none")?,
            error: status_facts::state_value(conn, "daemon error", "none")?,
            next_action: status_facts::state_value(conn, "authority next action", "none")?,
        })
    }
}

pub(super) fn fact(key: impl Into<String>, value: impl Into<String>) -> StatusFact {
    StatusFact {
        key: key.into(),
        value: value.into(),
    }
}

fn pending_count(conn: &Connection) -> Result<usize, CliError> {
    Ok(lkjagent_store::queue::list(conn)?
        .iter()
        .filter(|row| row.status == "pending")
        .count())
}

fn state_label(state: &str) -> &'static str {
    match state {
        "idle" => "IDLE",
        "working" => "WORKING",
        "waiting" => "WAITING",
        "error" => "ERROR",
        _ => "STOPPED",
    }
}

pub(super) type Policy = ContextBudgetPolicy;
