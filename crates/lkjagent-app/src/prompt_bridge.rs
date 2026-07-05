use std::path::Path;

use lkjagent_core::render::Prompt;
use lkjagent_core::runtime_decision::RuntimeDecision;
use lkjagent_store::prompt_rows::insert_prompt_frame;
use rusqlite::Connection;

use crate::context_bridge::PromptContext;

pub fn persist_prompt_frame(
    conn: &Connection,
    logs: &Path,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    context: &PromptContext,
    now: &str,
) -> Result<(), String> {
    let body_ref = write_prompt_body(logs, decision, prompt, context)?;
    insert_prompt_frame(
        conn,
        &format!("prompt-{}", decision.id),
        decision,
        prompt,
        &body_ref,
        now,
    )
    .map_err(|error| error.to_string())
}

fn write_prompt_body(
    logs: &Path,
    decision: &RuntimeDecision,
    prompt: &Prompt,
    context: &PromptContext,
) -> Result<String, String> {
    let relative = format!(
        "logs/case-{}/decision-{}/prompt-frame.json",
        decision.case_id, decision.id
    );
    let path = logs
        .join(format!("case-{}", decision.case_id))
        .join(format!("decision-{}", decision.id));
    std::fs::create_dir_all(&path).map_err(|error| error.to_string())?;
    let body = serde_json::json!({
        "decision_id": decision.id,
        "prompt_fingerprint": prompt.fingerprint,
        "context_frame_fingerprint": decision.context_frame_fingerprint,
        "tool_view_fingerprint": decision.tool_view_fingerprint().unwrap_or_default(),
        "context_plan": context.plan,
        "system": prompt.system,
        "user": prompt.user,
        "max_tokens": prompt.max_tokens,
        "stop": prompt.stop,
    });
    std::fs::write(path.join("prompt-frame.json"), body.to_string())
        .map_err(|error| error.to_string())?;
    Ok(relative)
}
