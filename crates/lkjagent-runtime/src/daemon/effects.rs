use lkjagent_store::events::{append_event, EventKind};
use lkjagent_store::state as store_state;
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::prompt::token_estimate;
use crate::step::{Effect, StepResult};
use crate::task::StopReason;

impl ResidentDaemon {
    pub(super) fn apply_step_result(
        &mut self,
        conn: &mut Connection,
        now: &str,
        result: StepResult,
        skip_owner_record: bool,
    ) -> RuntimeResult<DaemonTick> {
        let mut tick = tick_for_stop(result.stop_reason);
        self.state = result.state;
        for effect in result.effects {
            if let Some(next) = self.apply_effect(conn, now, effect, skip_owner_record)? {
                tick = next;
            }
        }
        self.write_observable(conn)?;
        Ok(tick)
    }

    pub(super) fn record_endpoint_error(
        &self,
        conn: &Connection,
        now: &str,
        message: &str,
    ) -> RuntimeResult<()> {
        append_event(
            conn,
            self.event_turn(),
            EventKind::Error,
            message,
            token_estimate(message) as i64,
            now,
        )?;
        store_state::set(conn, "daemon state", "error")?;
        store_state::set(conn, "daemon error", message)?;
        Ok(())
    }

    fn apply_effect(
        &mut self,
        conn: &mut Connection,
        now: &str,
        effect: Effect,
        skip_owner_record: bool,
    ) -> RuntimeResult<Option<DaemonTick>> {
        match effect {
            Effect::RecordEvent {
                kind: EventKind::Owner,
                ..
            } if skip_owner_record => Ok(None),
            Effect::RecordEvent {
                kind,
                content,
                tokens,
            } => {
                append_event(conn, self.event_turn(), kind, &content, tokens, now)?;
                Ok(None)
            }
            Effect::ExecuteTool { action_text } => self.execute_pending(conn, now, &action_text),
            Effect::DistillTask { summary, .. } => {
                self.save_task_summary(conn, now, &summary)?;
                Ok(None)
            }
            Effect::RecordGraphEvidence {
                case_id,
                requirement,
                kind,
                summary,
                path,
            } => {
                let evidence = lkjagent_store::graph::GraphEvidenceRow {
                    requirement,
                    kind,
                    summary,
                    path,
                };
                lkjagent_store::graph::record_evidence(conn, case_id, &evidence, now)?;
                Ok(None)
            }
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
                Ok(None)
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
                Ok(None)
            }
            Effect::RecordGraphNote {
                case_id,
                kind,
                summary,
            } => {
                lkjagent_store::graph::notes::record_note(
                    conn, case_id, &kind, &summary, "runtime", now,
                )?;
                Ok(None)
            }
            Effect::RecordGraphTransition {
                case_id,
                from_node,
                to_node,
                decision,
                reason,
            } => {
                lkjagent_store::graph::transitions::record_transition(
                    conn, case_id, &from_node, &to_node, &decision, &reason, now,
                )?;
                Ok(None)
            }
            Effect::UpdateGraphCase {
                case_id,
                phase,
                active_node,
                status,
            } => {
                lkjagent_store::graph::update_case(
                    conn,
                    case_id,
                    &phase,
                    &active_node,
                    &status,
                    now,
                )?;
                Ok(None)
            }
            Effect::Pause { reason } => {
                store_state::set(conn, "daemon error", &reason)?;
                Ok(Some(DaemonTick::Paused))
            }
            Effect::CompactionRecorded {
                before_tokens,
                after_tokens,
                memory_ids,
                policy,
            } => self.record_compaction(conn, now, before_tokens, after_tokens, memory_ids, policy),
        }
    }
}

fn tick_for_stop(stop: Option<StopReason>) -> DaemonTick {
    match stop {
        Some(StopReason::Done) => DaemonTick::Done,
        Some(StopReason::Ask) => DaemonTick::Waiting,
        Some(StopReason::EndpointError) => DaemonTick::EndpointError,
        Some(StopReason::Acted | StopReason::Compaction | StopReason::Maintenance) => {
            DaemonTick::Working
        }
        Some(StopReason::InvalidAction | StopReason::UnknownTool) => DaemonTick::Working,
        Some(StopReason::BadParams | StopReason::BudgetNotice) => DaemonTick::Working,
        Some(StopReason::ToolError | StopReason::RepeatAction) => DaemonTick::Working,
        None => DaemonTick::Working,
    }
}
