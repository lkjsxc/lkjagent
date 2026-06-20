use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::graph_state_tracks::store_track_rows;
use crate::step::Effect;

impl ResidentDaemon {
    pub(super) fn apply_graph_effect(
        &self,
        conn: &Connection,
        now: &str,
        effect: Effect,
    ) -> RuntimeResult<()> {
        match effect {
            Effect::RecordGraphEvidence {
                case_id,
                requirement,
                kind,
                summary,
                path,
            } => record_evidence(conn, now, case_id, requirement, kind, summary, path),
            Effect::RecordGraphPlan { case_id, steps } => {
                let rows = steps
                    .into_iter()
                    .enumerate()
                    .map(
                        |(index, step)| lkjagent_store::graph::plan::GraphPlanStepRow {
                            case_id,
                            step_id: step.step_id,
                            title: step.title,
                            rationale: step.rationale,
                            status: step.status,
                            node: step.node,
                            target_paths: step.target_paths,
                            checks: step.checks,
                            sort_order: index as i64,
                        },
                    )
                    .collect::<Vec<_>>();
                lkjagent_store::graph::plan::replace_plan_steps(conn, case_id, &rows, now)?;
                Ok(())
            }
            Effect::RecordGraphContext {
                case_id,
                packages,
                reason,
            } => {
                for package in packages {
                    lkjagent_store::graph::context::record_context_binding(
                        conn, case_id, &package, &reason, "selected", now,
                    )?;
                }
                Ok(())
            }
            Effect::RecordGraphNote {
                case_id,
                kind,
                summary,
            } => lkjagent_store::graph::notes::record_note(
                conn, case_id, &kind, &summary, "runtime", now,
            )
            .map_err(Into::into),
            Effect::RecordGraphTransition {
                case_id,
                from_node,
                to_node,
                decision,
                reason,
            } => lkjagent_store::graph::transitions::record_transition(
                conn, case_id, &from_node, &to_node, &decision, &reason, now,
            )
            .map_err(Into::into),
            Effect::UpdateGraphCase {
                case_id,
                phase,
                active_node,
                status,
            } => lkjagent_store::graph::update_case(
                conn,
                case_id,
                &phase,
                &active_node,
                &status,
                now,
            )
            .map_err(Into::into),
            Effect::RecordGraphFault {
                case_id,
                kind,
                action_fingerprint,
                summary,
                count,
            } => lkjagent_store::graph::faults::record_fault(
                conn,
                case_id,
                &kind,
                action_fingerprint.as_deref(),
                &summary,
                count,
                now,
            )
            .map_err(Into::into),
            Effect::UpdateGraphRecovery {
                case_id,
                ladder_position,
                strategy,
            } => lkjagent_store::graph::faults::upsert_recovery_state(
                conn,
                case_id,
                ladder_position,
                &strategy,
                now,
            )
            .map_err(Into::into),
            Effect::ReplaceGraphStateTracks { case_id, tracks } => {
                let rows = store_track_rows(tracks);
                lkjagent_store::graph::state_tracks::replace_state_tracks(conn, case_id, &rows, now)
                    .map_err(Into::into)
            }
            _ => Ok(()),
        }
    }
}

fn record_evidence(
    conn: &Connection,
    now: &str,
    case_id: i64,
    requirement: String,
    kind: String,
    summary: String,
    path: Option<String>,
) -> RuntimeResult<()> {
    let evidence = lkjagent_store::graph::GraphEvidenceRow {
        requirement,
        kind,
        summary,
        path,
    };
    lkjagent_store::graph::record_evidence(conn, case_id, &evidence, now)?;
    Ok(())
}
