# Data Directory

## Purpose

Runtime data lives here during local runs. SQLite databases, exchange logs,
workspace files, and flat local configuration are local runtime evidence and are
not committed by default.

## Configuration

`data/lkjagent.json` is optional flat JSON configuration. It may contain scalar
or primitive-array keys only. Nested objects and nested arrays are startup
errors. Supported keys include `endpoint_url`, `endpoint_model`,
`endpoint_api_key_env`, `endpoint_timeout_seconds`, `workspace_root`,
`prompt_max_context_tokens`, and `live_campaign_seconds`.

Secrets stay in environment variables. Do not put secret values in this file.
Model-visible prompts do not include the raw JSON config blob.

## Workspace

`data/workspace` is the default owner-visible memory tree. The owner should be
able to inspect records, transcripts, artifacts, indexes, and proof files there
without asking the agent.
