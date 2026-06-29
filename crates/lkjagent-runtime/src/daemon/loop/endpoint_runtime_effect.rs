use rusqlite::Connection;

use super::super::runner::{DaemonTick, ResidentDaemon};
use crate::error::RuntimeResult;
use crate::mode::TurnAuthority;
use crate::step::{step, StepInput};
use crate::task::PendingActionAuthority;

impl ResidentDaemon {
    pub(super) fn runtime_effect_turn(
        &mut self,
        conn: &mut Connection,
        now: &str,
        authority: &TurnAuthority,
    ) -> RuntimeResult<DaemonTick> {
        if authority.valid_example == "none" {
            self.write_observable(conn)?;
            return Ok(DaemonTick::Working);
        }
        let result = step(
            self.state.clone(),
            StepInput::AuthorizedCompletion(
                authority.valid_example.clone(),
                0,
                PendingActionAuthority {
                    authority_decision_id: authority.input.latest_decision_id.clone(),
                    prompt_frame_id: authority.input.prompt_frame_id.clone(),
                    staleness_fingerprint: authority.input.staleness_fingerprint.clone(),
                },
            ),
        );
        self.apply_step_result(conn, now, result, false)
    }
}
