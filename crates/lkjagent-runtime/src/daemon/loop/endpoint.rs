use std::time::Instant;

use lkjagent_context::assemble::assemble_messages;
use lkjagent_llm::client::{complete, request_json};
use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::error::{RuntimeError, RuntimeResult};
use crate::mode::EndpointDecision;
use crate::prompt::token_estimate;
use crate::step::{step, Effect, StepInput};
use crate::task::{PendingActionAuthority, TaskState};

const PROVIDER_ANOMALY_RETRY_LIMIT: u32 = 3;

impl ResidentDaemon {
    pub(super) fn endpoint_turn(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<DaemonTick> {
        let retry_pending = self.endpoint_retry_pending(now);
        let authority = self.decide_authority(conn, now, retry_pending)?;
        self.turn_authority = Some(authority.clone());
        match authority.endpoint_decision {
            EndpointDecision::WaitForRetry => return Ok(DaemonTick::EndpointError),
            EndpointDecision::RuntimeCompact => {
                if let Some(tick) = self.compact_before_endpoint(conn, now)? {
                    return Ok(tick);
                }
                self.write_observable(conn)?;
                return Ok(DaemonTick::Working);
            }
            EndpointDecision::DeliverOwner => {
                self.deliver_owner(conn, now)?;
                return Ok(DaemonTick::Working);
            }
            EndpointDecision::DeferMaintenance => {
                self.state.maintenance = None;
                return Ok(DaemonTick::Working);
            }
            EndpointDecision::ClosedIdle => {
                self.write_observable(conn)?;
                return Ok(DaemonTick::Idle);
            }
            EndpointDecision::CallModel => {}
        }
        if self.task_checkpoint_due() {
            self.state.continuation_epoch.checkpoint_turns = self.runtime.task_turn_budget.max(1);
            let result = step(self.state.clone(), StepInput::TurnBudgetCheckpoint);
            return self.apply_step_result(conn, now, result, false);
        }
        self.refresh_authority_card(conn, &authority)?;
        let messages = assemble_messages(&self.state.context);
        let request = request_json(&self.runtime.client, &messages)?;
        let provider_log = self.record_model_request(conn, now, &request)?;
        let started = Instant::now();
        match complete(&self.runtime.client, &messages, self.endpoint_attempt) {
            Ok(completion) => {
                self.record_model_response(conn, provider_log.as_ref(), &completion, started)?;
                if completion.provider_anomaly.is_none() {
                    self.record_model_parse(provider_log.as_ref(), &completion)?;
                }
                self.apply_completion(conn, now, completion)
            }
            Err(error) => {
                self.record_model_error(conn, provider_log.as_ref(), &error, started)?;
                match RuntimeError::from(error) {
                    RuntimeError::CompletionOversize { preview } => {
                        self.apply_oversize(conn, now, preview)
                    }
                    error => {
                        self.endpoint_attempt = self.endpoint_attempt.saturating_add(1);
                        self.endpoint_retry_at = retry_deadline(now, error.retry_after_secs());
                        self.record_endpoint_error(conn, now, &error.to_string())?;
                        Ok(DaemonTick::EndpointError)
                    }
                }
            }
        }
    }

    fn apply_completion(
        &mut self,
        conn: &mut Connection,
        now: &str,
        completion: lkjagent_llm::wire::Completion,
    ) -> RuntimeResult<DaemonTick> {
        let provider_anomaly = completion.provider_anomaly.clone();
        if provider_anomaly.is_some() {
            self.endpoint_attempt = self.endpoint_attempt.saturating_add(1);
            self.endpoint_retry_at = retry_deadline(
                now,
                Some(lkjagent_llm::backoff::delay_for_attempt(self.endpoint_attempt).as_secs()),
            );
        } else {
            self.endpoint_attempt = 0;
            self.endpoint_retry_at = None;
        }
        crate::token_usage::record_completion_usage(
            conn,
            now,
            &self.state,
            self.runtime.budget,
            &completion.usage,
        )?;
        let tokens = completion
            .usage
            .completion_tokens
            .unwrap_or_else(|| token_estimate(&completion.content) as u64);
        let result = if let Some(anomaly) = provider_anomaly {
            let mut result = step(
                self.state.clone(),
                StepInput::ProviderAnomaly(anomaly.kind.as_str().to_string(), anomaly.detail),
            );
            if self.endpoint_attempt >= PROVIDER_ANOMALY_RETRY_LIMIT {
                let reason = format!(
                    "provider anomaly retry budget exhausted; class={}",
                    anomaly.kind.as_str()
                );
                result.state.task = TaskState::Paused {
                    reason: reason.clone(),
                };
                result.effects.push(Effect::Pause { reason });
                self.endpoint_retry_at = None;
            }
            result
        } else {
            step(
                self.state.clone(),
                StepInput::AuthorizedCompletion(
                    completion.content,
                    tokens as usize,
                    pending_authority(conn)?,
                ),
            )
        };
        self.apply_step_result(conn, now, result, false)
    }

    fn apply_oversize(
        &mut self,
        conn: &mut Connection,
        now: &str,
        preview: String,
    ) -> RuntimeResult<DaemonTick> {
        self.endpoint_attempt = 0;
        self.endpoint_retry_at = None;
        let result = step(self.state.clone(), StepInput::EndpointOversize { preview });
        self.apply_step_result(conn, now, result, false)
    }

    fn task_checkpoint_due(&self) -> bool {
        matches!(self.state.task, TaskState::Open { turns_remaining: 0 })
            && self.state.pending_action.is_none()
    }

    fn endpoint_retry_pending(&mut self, now: &str) -> bool {
        let Some(deadline) = &self.endpoint_retry_at else {
            return false;
        };
        if seconds_before(now, deadline) {
            return true;
        }
        self.endpoint_retry_at = None;
        false
    }
}

fn pending_authority(conn: &Connection) -> RuntimeResult<PendingActionAuthority> {
    Ok(PendingActionAuthority {
        authority_decision_id: lkjagent_store::state::get(conn, "authority decision id")?,
        prompt_frame_id: lkjagent_store::state::get(conn, "authority prompt frame id")?,
        staleness_fingerprint: lkjagent_store::state::get(conn, "kernel staleness fingerprint")?,
    })
}

fn retry_deadline(now: &str, retry_after_secs: Option<u64>) -> Option<String> {
    let delay = retry_after_secs?;
    now.parse::<u64>()
        .ok()
        .map(|stamp| stamp.saturating_add(delay).to_string())
}

fn seconds_before(now: &str, deadline: &str) -> bool {
    match (now.parse::<u64>(), deadline.parse::<u64>()) {
        (Ok(now), Ok(deadline)) => now < deadline,
        _ => false,
    }
}
