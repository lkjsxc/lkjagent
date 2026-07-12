# Configuration

## Purpose

Define one flat JSON configuration with an actual consumer for every accepted key.

## File And Shape

The runtime reads `<data>/lkjagent.json`. The document is one flat object of
strings and integers. Arrays, nested objects, nulls, booleans, unknown
keys, empty strings, and out-of-range values fail startup.

Secrets are environment values named by configuration. Secret bytes never enter
the file, SQLite, prompts, status, logs, or evidence.

## Active Registry

```json
{
  "endpoint_api_key_env": "LKJAGENT_API_KEY",
  "endpoint_model": "local-model",
  "endpoint_timeout_seconds": 300,
  "endpoint_url": "http://127.0.0.1:8080",
  "live_campaign_seconds": 900,
  "prompt_context_tokens": 16384,
  "workspace_root": "../workspace"
}
```

This is the current registry. Endpoint transport consumes the endpoint keys,
doctor and the evaluation boundary consume the prompt/campaign bounds, and
workspace operations consume the root. Unknown future keys fail closed.

## Precedence

Narrow environment overrides for endpoint, model, timeout, key, prompt bound,
campaign duration, and deployment root win over file values; file values win
over documented defaults.

## Endpoint Request

The configured timeout reaches the actual HTTP request. Temperature, top-p, and
output bounds are direct-runtime constants selected with each decision.

## Compose

Compose mounts `${LKJAGENT_DATA_DIR:-./data}` at `/data` and
`${LKJAGENT_WORKSPACE_DIR:-./workspace}` at `/workspace`. The default relative
root therefore resolves consistently in direct and container execution.
