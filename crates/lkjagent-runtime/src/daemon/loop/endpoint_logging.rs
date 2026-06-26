use std::path::PathBuf;
use std::time::Instant;

use lkjagent_llm::error::ClientError;
use lkjagent_llm::wire::{Completion, CompletionUsage, FinishReason};
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::model_log::{
    json_escape, record_parsed_action, record_provider_error, record_provider_index,
    record_provider_request, record_provider_response, ProviderLogContext, ProviderLogHandle,
};

impl ResidentDaemon {
    pub(super) fn record_model_request(
        &self,
        conn: &Connection,
        now: &str,
        request_json: &str,
    ) -> RuntimeResult<Option<ProviderLogHandle>> {
        let Some(root) = self.provider_log_root() else {
            return Ok(None);
        };
        let context = self.provider_context(conn, now)?;
        let handle = record_provider_request(conn, &root, &context, request_json)?;
        record_provider_index(&root, &handle, &context)?;
        lkjagent_store::state::set(conn, "provider exchange id", &handle.id)?;
        lkjagent_store::state::set(conn, "provider exchange dir", &handle.dir.to_string_lossy())?;
        Ok(Some(handle))
    }

    pub(super) fn record_model_response(
        &self,
        conn: &Connection,
        handle: Option<&ProviderLogHandle>,
        completion: &Completion,
        started: Instant,
    ) -> RuntimeResult<()> {
        let Some(handle) = handle else {
            return Ok(());
        };
        let response_json = completion_response_json(completion);
        let usage = usage_json(&completion.usage);
        record_provider_response(
            conn,
            handle,
            &response_json,
            finish_reason_name(&completion.finish_reason),
            Some(&usage),
            latency_ms(started),
        )
    }

    pub(super) fn record_model_parse(
        &self,
        handle: Option<&ProviderLogHandle>,
        completion: &Completion,
    ) -> RuntimeResult<()> {
        let Some(handle) = handle else {
            return Ok(());
        };
        record_parsed_action(
            handle,
            &completion.content,
            completion.closure_mode.as_str(),
        )
    }

    pub(super) fn record_model_error(
        &self,
        conn: &Connection,
        handle: Option<&ProviderLogHandle>,
        error: &ClientError,
        started: Instant,
    ) -> RuntimeResult<()> {
        let Some(handle) = handle else {
            return Ok(());
        };
        record_provider_error(
            conn,
            handle,
            error_class(error),
            &error.to_string(),
            latency_ms(started),
        )
    }

    fn provider_log_root(&self) -> Option<PathBuf> {
        self.runtime
            .model_log_path
            .as_ref()
            .and_then(|path| path.parent())
            .map(PathBuf::from)
    }

    fn provider_context(&self, conn: &Connection, now: &str) -> RuntimeResult<ProviderLogContext> {
        Ok(ProviderLogContext {
            case_id: self.case_id_string(),
            turn_id: self.state.turn,
            prompt_frame_id: lkjagent_store::state::get(conn, "authority prompt frame id")?,
            authority_decision_id: lkjagent_store::state::get(conn, "authority decision id")?,
            provider: "openai-compatible".to_string(),
            model: self.runtime.client.model.clone(),
            created_at: now.to_string(),
            authority_json: self.authority_json(conn)?,
        })
    }

    fn case_id_string(&self) -> String {
        self.state
            .graph
            .as_ref()
            .and_then(|graph| graph.case_id)
            .map_or_else(|| "none".to_string(), |id| id.to_string())
    }

    fn authority_json(&self, conn: &Connection) -> RuntimeResult<String> {
        let Some(authority) = &self.turn_authority else {
            return Ok("{}\n".to_string());
        };
        let kernel_mission =
            lkjagent_store::state::get(conn, "kernel mission")?.unwrap_or_default();
        let kernel_stale =
            lkjagent_store::state::get(conn, "kernel staleness fingerprint")?.unwrap_or_default();
        Ok(format!(
            "{{\"active_mode\":\"{:?}\",\"mission\":\"{}\",\"kernel_mission\":\"{}\",\"kernel_staleness_fingerprint\":\"{}\",\"admitted_tools\":\"{}\",\"blocked_tools\":\"{}\"}}\n",
            authority.mode,
            json_escape(authority.mission.as_str()),
            json_escape(&kernel_mission),
            json_escape(&kernel_stale),
            json_escape(&authority.effective_policy.allowed_tools.join(",")),
            json_escape(&authority.effective_policy.blocked_tools.join(",")),
        ))
    }
}

fn completion_response_json(completion: &Completion) -> String {
    format!(
        "{{\"content\":\"{}\",\"provider_anomaly\":{},\"finish_reason\":\"{}\",\"closure_mode\":\"{}\",\"usage\":{}}}\n",
        json_escape(&completion.content),
        provider_anomaly_json(completion),
        finish_reason_name(&completion.finish_reason),
        completion.closure_mode.as_str(),
        usage_json(&completion.usage)
    )
}

fn provider_anomaly_json(completion: &Completion) -> String {
    completion.provider_anomaly.as_ref().map_or_else(
        || "null".to_string(),
        |anomaly| {
            format!(
                "{{\"kind\":\"{}\",\"detail\":\"{}\"}}",
                anomaly.kind.as_str(),
                json_escape(&anomaly.detail)
            )
        },
    )
}

fn usage_json(usage: &CompletionUsage) -> String {
    format!(
        "{{\"prompt_tokens\":{},\"completion_tokens\":{},\"cached_prompt_tokens\":{},\"total_tokens\":{}}}",
        opt_u64(usage.prompt_tokens),
        opt_u64(usage.completion_tokens),
        opt_u64(usage.cached_prompt_tokens),
        opt_u64(usage.total_tokens)
    )
}

fn opt_u64(value: Option<u64>) -> String {
    value.map_or_else(|| "null".to_string(), |value| value.to_string())
}

fn finish_reason_name(reason: &FinishReason) -> &str {
    match reason {
        FinishReason::Stop => "stop",
        FinishReason::Length => "length",
        FinishReason::Other(_) => "other",
        FinishReason::Missing => "missing",
    }
}

fn error_class(error: &ClientError) -> &str {
    match error {
        ClientError::Endpoint { .. } => "EndpointError",
        ClientError::EndpointOverflow { .. } => "EndpointOverflow",
        ClientError::Oversize { .. } => "CompletionOversize",
    }
}

fn latency_ms(started: Instant) -> i64 {
    i64::try_from(started.elapsed().as_millis()).unwrap_or(i64::MAX)
}
