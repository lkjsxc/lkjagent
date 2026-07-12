# Configuration

## Purpose

Define one flat JSON configuration with an actual consumer for every accepted key.

## File And Shape

The runtime reads `<data>/lkjagent.json`. The document is one flat object of
strings, integers, numbers, and booleans. Arrays, nested objects, nulls, unknown
keys, empty strings, and out-of-range values fail startup.

Secrets are environment values named by configuration. Secret bytes never enter
the file, SQLite, prompts, status, logs, or evidence.

## Active Registry

```json
{
  "context_max_tokens": 16384,
  "context_output_reserve_tokens": 2048,
  "endpoint_api_key_env": "LKJAGENT_API_KEY",
  "endpoint_model": "local-model",
  "endpoint_reasoning_effort": "none",
  "endpoint_timeout_seconds": 300,
  "endpoint_url": "http://127.0.0.1:8080",
  "file_read_max_bytes": 65536,
  "matter_effect_limit": 64,
  "matter_model_call_limit": 64,
  "model_max_output_tokens": 1024,
  "model_temperature": 0.2,
  "model_top_p": 0.9,
  "no_progress_limit": 3,
  "queue_wake_milliseconds": 250,
  "recovery_cost_limit": 24,
  "shell_enabled": false,
  "shell_timeout_seconds": 300,
  "timezone": "Asia/Tokyo",
  "tool_output_max_bytes": 32768,
  "tui_history_messages": 1000,
  "tui_refresh_milliseconds": 100,
  "workspace_file_max_tokens": 512,
  "workspace_root": "../workspace"
}
```

This is the target registry. `../current-state.md` records keys not yet consumed.
A repository check rejects any accepted key without a production reference.

## Precedence

Narrow environment overrides for endpoint, model, key, and deployment root win
over file values; file values win over documented defaults. A changed effective
nonsecret config fingerprint emits a durable wake event when eligibility changes.

## Endpoint Request

Temperature, top-p, output bound, timeout, and optional reasoning effort reach
the actual HTTP request. Unsupported optional fields are omitted by endpoint
policy rather than always sent.

## Compose

Compose mounts `${LKJAGENT_DATA_DIR:-./data}` at `/data` and
`${LKJAGENT_WORKSPACE_DIR:-./workspace}` at `/workspace`. The default relative
root therefore resolves consistently in direct and container execution.
