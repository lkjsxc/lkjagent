use std::collections::BTreeMap;

use lkjagent_core::engine::Command;
use lkjagent_core::parse::Action;
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction, ToolAdmission};
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::admission_rows::insert_tool_admission;
use rusqlite::{params, Connection};

pub fn persist_tool_admissions(
    conn: &Connection,
    decision: &RuntimeDecision,
    commands: &[Command],
    now: &str,
) -> Result<(), String> {
    let mut ordinal = existing_admission_count(conn, &decision.id)?;
    for command in commands {
        let Command::RunExplore(action) = command else {
            continue;
        };
        let model_action = model_action(action);
        let mut admission = admit_action(decision, &model_action).map_err(|error| error.message)?;
        let parsed = serde_json::to_string(&model_action).map_err(|error| error.to_string())?;
        if admission.status == AdmissionStatus::Admitted
            && repeated_admitted_action(conn, &decision.id, &parsed)?
        {
            admission = repeated_admission(&admission);
        }
        ordinal += 1;
        let id = format!("{}-admission-{ordinal:04}", decision.id);
        insert_tool_admission(conn, &id, &decision.case_id, &admission, &parsed, now)
            .map_err(|error| error.to_string())?;
        if admission.status == AdmissionStatus::Rejected {
            return Err(format!("admission rejected: {}", admission.reason));
        }
    }
    Ok(())
}

fn existing_admission_count(conn: &Connection, decision_id: &str) -> Result<i64, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM tool_admissions WHERE decision_id = ?1",
        [decision_id],
        |row| row.get(0),
    )
    .map_err(|error| error.to_string())
}

fn repeated_admitted_action(
    conn: &Connection,
    decision_id: &str,
    parsed: &str,
) -> Result<bool, String> {
    let found: i64 = conn
        .query_row(
            "SELECT EXISTS(
             SELECT 1 FROM tool_admissions
             WHERE decision_id = ?1 AND status = 'Admitted' AND parsed_action_json = ?2)",
            params![decision_id, parsed],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    Ok(found != 0)
}

fn repeated_admission(admission: &ToolAdmission) -> ToolAdmission {
    let mut rejected = admission.clone();
    rejected.status = AdmissionStatus::Rejected;
    rejected.reason =
        "repeated tool call; state the next different tool call or finish".to_string();
    rejected
}

fn model_action(action: &Action) -> ModelAction {
    ModelAction {
        tool: action.tool.clone(),
        params: action
            .params
            .iter()
            .filter(|(name, _)| name != "tool_name")
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect::<BTreeMap<_, _>>(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lkjagent_core::runtime_decision::{
        OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
    };
    use lkjagent_store::plan_schema::setup;

    #[test]
    fn repeat_guard_rejects_previously_admitted_action() -> Result<(), String> {
        let conn = Connection::open_in_memory().map_err(|error| error.to_string())?;
        setup(&conn).map_err(|error| error.to_string())?;
        let decision = RuntimeDecision::new(
            "decision-1",
            "case-1",
            OperationKey("model.call/1".to_string()),
            ToolSetView::new(vec![
                ToolViewEntry::new("fs.read", "read").with_params(vec!["path"], Vec::new())
            ]),
            OutputEnvelope::Action,
        );
        let command = Command::RunExplore(action("fs.read", "path", "README.md"));

        persist_tool_admissions(&conn, &decision, std::slice::from_ref(&command), "now")?;
        let error = match persist_tool_admissions(&conn, &decision, &[command], "later") {
            Ok(()) => return Err("repeat admitted".to_string()),
            Err(error) => error,
        };

        assert!(error.contains("repeated tool call"));
        assert_eq!(admission_count(&conn, "Admitted")?, 1);
        assert_eq!(admission_count(&conn, "Rejected")?, 1);
        Ok(())
    }

    #[test]
    fn mismatch_reason_persists_for_hidden_tool() -> Result<(), String> {
        let conn = Connection::open_in_memory().map_err(|error| error.to_string())?;
        setup(&conn).map_err(|error| error.to_string())?;
        let decision = RuntimeDecision::new(
            "decision-1",
            "case-1",
            OperationKey("model.call/1".to_string()),
            ToolSetView::new(vec![
                ToolViewEntry::new("fs.read", "read").with_params(vec!["path"], Vec::new())
            ]),
            OutputEnvelope::Action,
        );
        let command = Command::RunExplore(action("shell.run", "cmd", "date"));

        let error = match persist_tool_admissions(&conn, &decision, &[command], "now") {
            Ok(()) => return Err("mismatch admitted".to_string()),
            Err(error) => error,
        };

        assert!(error.contains("tool-view mismatch"));
        assert!(result_json(&conn)?.contains("tool-view mismatch"));
        Ok(())
    }

    fn action(tool: &str, name: &str, value: &str) -> Action {
        Action {
            tool: tool.to_string(),
            params: vec![(name.to_string(), value.to_string())],
        }
    }

    fn admission_count(conn: &Connection, status: &str) -> Result<i64, String> {
        conn.query_row(
            "SELECT COUNT(*) FROM tool_admissions WHERE status = ?1",
            [status],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())
    }

    fn result_json(conn: &Connection) -> Result<String, String> {
        conn.query_row("SELECT result_json FROM tool_admissions", [], |row| {
            row.get(0)
        })
        .map_err(|error| error.to_string())
    }
}
