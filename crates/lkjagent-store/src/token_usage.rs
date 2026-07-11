use rusqlite::{params, Connection};

use crate::error::StoreResult;

pub fn insert_usage_tx(
    tx: &Connection,
    matter_id: i64,
    attempt_id: i64,
    attempt: &lkjagent_core::model::Attempt,
    now: &str,
) -> StoreResult<()> {
    tx.execute(
        "INSERT INTO token_usage (task_id, attempt_id, prompt_tokens, completion_tokens,
         cached_tokens, input_total_tokens, input_cached_tokens, input_uncached_tokens,
         output_tokens, cache_status, raw_usage_json, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            matter_id,
            attempt_id,
            token(attempt.tokens_in),
            token(attempt.tokens_out),
            token(attempt.cached_tokens),
            token(attempt.tokens_in),
            cached_token(attempt),
            uncached_token(attempt),
            token(attempt.tokens_out),
            attempt.cache_status,
            raw_usage(attempt),
            now
        ],
    )?;
    Ok(())
}

fn raw_usage(attempt: &lkjagent_core::model::Attempt) -> String {
    serde_json::json!({
        "prompt_tokens": token(attempt.tokens_in),
        "completion_tokens": token(attempt.tokens_out),
        "cached_tokens": cached_token(attempt),
        "cache_status": attempt.cache_status,
    })
    .to_string()
}

fn token(value: u32) -> Option<i64> {
    (value > 0).then_some(value as i64)
}

fn cached_token(attempt: &lkjagent_core::model::Attempt) -> Option<i64> {
    (attempt.cache_status == "known").then_some(attempt.cached_tokens as i64)
}

fn uncached_token(attempt: &lkjagent_core::model::Attempt) -> Option<i64> {
    if attempt.cache_status != "known" || attempt.tokens_in == 0 {
        return None;
    }
    Some(attempt.tokens_in.saturating_sub(attempt.cached_tokens) as i64)
}
