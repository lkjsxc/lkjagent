use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::prompt_rows::insert_prompt_frame;
use rusqlite::Connection;

pub fn persist_prompt_frame(
    conn: &Connection,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    now: &str,
) -> Result<(), String> {
    insert_prompt_frame(
        conn,
        &format!("prompt-{}", decision.id),
        decision,
        prompt,
        &format!("inline:{}", prompt.fingerprint),
        now,
    )
    .map_err(|error| error.to_string())
}
