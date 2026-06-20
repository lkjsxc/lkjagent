use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::daemon::endpoint_complete;
use crate::error::{RuntimeError, RuntimeResult};
use crate::prompt::token_estimate;
use crate::step::{step, StepInput};

impl ResidentDaemon {
    pub(super) fn endpoint_turn(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<DaemonTick> {
        if self.endpoint_retry_pending(now) {
            return Ok(DaemonTick::EndpointError);
        }
        if let Some(tick) = self.compact_before_endpoint(conn, now)? {
            return Ok(tick);
        }
        match endpoint_complete(
            &self.runtime.client,
            &self.state.context,
            self.endpoint_attempt,
        ) {
            Ok(completion) => self.apply_completion(conn, now, completion),
            Err(RuntimeError::CompletionOversize { preview }) => {
                self.apply_oversize(conn, now, preview)
            }
            Err(error) => {
                self.endpoint_attempt = self.endpoint_attempt.saturating_add(1);
                self.endpoint_retry_at = retry_deadline(now, error.retry_after_secs());
                self.record_endpoint_error(conn, now, &error.to_string())?;
                Ok(DaemonTick::EndpointError)
            }
        }
    }

    fn apply_completion(
        &mut self,
        conn: &mut Connection,
        now: &str,
        completion: lkjagent_llm::wire::Completion,
    ) -> RuntimeResult<DaemonTick> {
        self.endpoint_attempt = 0;
        self.endpoint_retry_at = None;
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
        let result = step(
            self.state.clone(),
            StepInput::Completion {
                content: completion.content,
                tokens: tokens as usize,
            },
        );
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
