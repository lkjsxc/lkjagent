use lkjagent_core::engine::{Command, Work};
use lkjagent_core::model::{Attempt, AttemptOutcome, Event, EventKind, StepState, TaskSnapshot};
use lkjagent_store::plan_commit::commit_turn;
use rusqlite::Connection;

pub fn settle(
    conn: &mut Connection,
    snapshot: &TaskSnapshot,
    work: &Work,
    error: String,
) -> Result<TaskSnapshot, String> {
    let mut failed = snapshot.clone();
    let mut commands = Vec::new();
    if let Work::CallModel { step_id, prompt } = work {
        if let Some(step) = failed.steps.iter_mut().find(|step| step.id == *step_id) {
            let ordinal = step.actions_used + step.attempts_used + 1;
            step.state = StepState::Active;
            step.attempts_used = step.attempts_used.saturating_add(1);
            failed.task.budget_used = failed.task.budget_used.saturating_add(1);
            let attempt = Attempt {
                step_id: *step_id,
                ordinal,
                prompt_fingerprint: prompt.fingerprint.clone(),
                outcome: AttemptOutcome::EffectError,
                diagnosis: error.clone(),
                tokens_in: 0,
                tokens_out: 0,
                cached_tokens: 0,
            };
            failed.attempts.push(attempt.clone());
            commands.push(Command::RecordAttempt(attempt));
        }
    }
    let event = Event {
        kind: EventKind::Notice,
        content: format!("effect_error: {error}"),
    };
    failed.events.push(event.clone());
    commands.push(Command::RecordEvent(event));
    commit_turn(conn, &failed, &commands, "now").map_err(|error| error.to_string())?;
    Ok(failed)
}
