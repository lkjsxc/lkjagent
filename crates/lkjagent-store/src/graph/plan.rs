use rusqlite::{params, Connection};

use crate::error::StoreResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphPlanStepRow {
    pub case_id: i64,
    pub step_id: String,
    pub title: String,
    pub rationale: String,
    pub status: String,
    pub node: String,
    pub target_paths: Vec<String>,
    pub checks: Vec<String>,
    pub sort_order: i64,
}

pub fn replace_plan_steps(
    conn: &Connection,
    case_id: i64,
    steps: &[GraphPlanStepRow],
    now: &str,
) -> StoreResult<()> {
    conn.execute(
        "DELETE FROM graph_plan_steps WHERE case_id = ?1",
        params![case_id],
    )?;
    for step in steps {
        conn.execute(
            "INSERT INTO graph_plan_steps
             (case_id, step_id, title, rationale, status, node, target_paths,
              checks, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)",
            params![
                case_id,
                step.step_id,
                step.title,
                step.rationale,
                step.status,
                step.node,
                step.target_paths.join("\n"),
                step.checks.join("\n"),
                step.sort_order,
                now
            ],
        )?;
    }
    Ok(())
}
