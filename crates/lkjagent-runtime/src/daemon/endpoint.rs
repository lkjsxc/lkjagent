use rusqlite::Connection;

use super::runner::{DaemonTick, ResidentDaemon};
use crate::daemon::endpoint_complete;
use crate::error::{RuntimeError, RuntimeResult};
use crate::step::{step, StepInput};

impl ResidentDaemon {
    pub(super) fn endpoint_turn(
        &mut self,
        conn: &mut Connection,
        now: &str,
    ) -> RuntimeResult<DaemonTick> {
        match endpoint_complete(
            &self.runtime.client,
            &self.state.context,
            self.endpoint_attempt,
        ) {
            Ok(completion) => self.apply_completion(conn, now, completion),
            Err(RuntimeError::CompletionOversize) => self.apply_oversize(conn, now),
            Err(error) => {
                self.endpoint_attempt = self.endpoint_attempt.saturating_add(1);
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
        let result = step(
            self.state.clone(),
            StepInput::Completion {
                content: completion.content,
                tokens: completion.usage.completion_tokens as usize,
            },
        );
        self.apply_step_result(conn, now, result, false)
    }

    fn apply_oversize(&mut self, conn: &mut Connection, now: &str) -> RuntimeResult<DaemonTick> {
        self.endpoint_attempt = 0;
        let result = step(self.state.clone(), StepInput::EndpointOversize);
        self.apply_step_result(conn, now, result, false)
    }
}
