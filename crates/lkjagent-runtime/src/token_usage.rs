use lkjagent_context::budget::ContextBudgetPolicy;
use lkjagent_llm::wire::CompletionUsage;
use rusqlite::Connection;

use crate::error::RuntimeResult;
use crate::task::RuntimeState;

pub(crate) fn record_completion_usage(
    conn: &Connection,
    now: &str,
    state: &RuntimeState,
    budget: ContextBudgetPolicy,
    usage: &CompletionUsage,
) -> RuntimeResult<()> {
    let event = lkjagent_store::token_usage::TokenUsageEvent {
        task_id: state.graph.as_ref().and_then(|graph| graph.case_id),
        turn: state.turn,
        input_tokens: usage.prompt_tokens,
        output_tokens: usage.completion_tokens,
        cached_input_tokens: usage.cached_prompt_tokens,
        total_tokens: usage.total_tokens,
        context_window: Some(budget.window as u64),
        context_used_estimate: Some(state.context.used_tokens() as u64),
        source: "endpoint".to_string(),
    };
    lkjagent_store::token_usage::record(conn, &event, now)?;
    Ok(())
}
