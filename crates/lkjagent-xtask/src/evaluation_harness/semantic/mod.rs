mod artifact;
mod daily;
mod exact;
mod projects;
pub mod shared;
mod terminal;

use rusqlite::Connection;
use std::path::Path;

use super::{scenario::Scenario, snapshot::Capture};
use shared::{cast_path, count, manifest_rows};

pub struct Metrics {
    pub passed: bool,
    pub fields: Vec<(String, String)>,
}

pub struct CommonCounts {
    pub owner_turns: u64,
    pub runtime_decisions: u64,
    pub progress_decisions: u64,
    pub useful_decisions: u64,
    pub provider_exchanges: u64,
    pub activity_rows: u64,
    pub current_passed_checks: u64,
    pub table_count: u64,
}

pub struct Context<'a> {
    pub scenario: &'a Scenario,
    pub capture: &'a Capture,
    pub db: &'a Connection,
    pub before: &'a str,
    pub after: &'a str,
    pub common: &'a CommonCounts,
    pub cast: &'a Path,
}

pub fn measure(
    scenario: &Scenario,
    capture: &Capture,
    before: &str,
    after: &str,
    table_facts: &str,
) -> Result<Metrics, String> {
    let db =
        Connection::open(capture.raw.join("state.sqlite3")).map_err(|error| error.to_string())?;
    let common = common_counts(&db, table_facts)?;
    let mut fields = vec![
        (
            "measured_owner_turn_count".into(),
            common.owner_turns.to_string(),
        ),
        (
            "measured_runtime_decision_count".into(),
            common.runtime_decisions.to_string(),
        ),
        (
            "measured_progress_decision_count".into(),
            common.progress_decisions.to_string(),
        ),
        (
            "measured_useful_decision_count".into(),
            common.useful_decisions.to_string(),
        ),
        (
            "measured_provider_exchange_count".into(),
            common.provider_exchanges.to_string(),
        ),
        (
            "measured_activity_count".into(),
            common.activity_rows.to_string(),
        ),
    ];
    let ctx = Context {
        scenario,
        capture,
        db: &db,
        before,
        after,
        common: &common,
        cast: &cast_path(&capture.raw),
    };
    let measured = match scenario.id.as_str() {
        "daily-life-recall" => daily::measure(&ctx)?,
        "exact-file-edit" => exact::measure(&ctx)?,
        "long-artifact-recovery" => artifact::measure(&ctx)?,
        "multi-project-development" => projects::measure(&ctx)?,
        "slow-japanese-pty" => terminal::measure(&ctx)?,
        other => return Err(format!("unsupported scenario evaluator: {other}")),
    };
    fields.extend(measured.fields);
    Ok(Metrics {
        passed: measured.passed,
        fields,
    })
}

fn common_counts(connection: &Connection, table_facts: &str) -> Result<CommonCounts, String> {
    let runtime_decisions = count(connection, "SELECT count(*) FROM runtime_decisions")?;
    let provider_exchanges = count(connection, "SELECT count(*) FROM provider_exchanges")?;
    Ok(CommonCounts {
        owner_turns: count(connection, "SELECT count(*) FROM conversation_messages WHERE role='owner'")?,
        runtime_decisions,
        progress_decisions: count(connection, "SELECT count(DISTINCT decision_id) FROM tool_admissions WHERE effectful=1")?,
        useful_decisions: count(connection, "SELECT count(*) FROM runtime_decisions WHERE CAST(operation_key AS TEXT) NOT IN ('wait','idle')")?,
        provider_exchanges,
        activity_rows: runtime_decisions
            + count(connection, "SELECT count(*) FROM effect_journal")?
            + count(connection, "SELECT count(*) FROM checks")?,
        current_passed_checks: count(connection, "SELECT count(*) FROM checks WHERE current=1 AND passed=1")?,
        table_count: manifest_rows(table_facts).len() as u64,
    })
}

struct Measured {
    passed: bool,
    fields: Vec<(String, String)>,
}

impl Measured {
    fn new(passed: bool, fields: Vec<(String, String)>) -> Self {
        Self { passed, fields }
    }
}
