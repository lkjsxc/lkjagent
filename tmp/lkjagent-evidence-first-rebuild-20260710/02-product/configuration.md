# Configuration

## File

Read data/lkjagent.json as one flat JSON object. Reject nested objects, nested
arrays, arrays of any kind, null, unknown keys, wrong primitive types, invalid
ranges, and empty required strings. Only string, integer, and boolean values are
accepted. The complete registry is configuration-registry.md; implementations
may not invent undocumented keys.

## Example

    {
      "endpoint_url": "http://127.0.0.1:8080",
      "endpoint_model": "local-model",
      "endpoint_api_key_env": "LKJAGENT_API_KEY",
      "endpoint_timeout_seconds": 300,
      "workspace_root": "../workspace",
      "timezone": "Asia/Tokyo",
      "prompt_context_tokens": 16384,
      "prompt_output_reserve_tokens": 2048,
      "workspace_file_max_tokens": 512,
      "tool_view_max_items": 4,
      "no_progress_window": 3,
      "maintenance_interval_seconds": 900,
      "tui_refresh_milliseconds": 100,
      "shell_enabled": true,
      "shell_timeout_seconds": 300,
      "live_campaign_seconds": 900
    }

## Registry

One typed Rust registry owns key name, type, default, range, secrecy, reload
policy, documentation, and consuming component. Generate the example and key
reference from the registry, then verify every declared key is read by runtime.

## Precedence

Command-line data and workspace paths override the file only for that process.
Named environment variables override endpoint secrets and deployment-specific
connection values. The effective configuration fingerprint is durable, but raw
secrets and the raw JSON text never enter model context.

## Reload

Hot-reload only keys marked safe, such as refresh and maintenance intervals.
Endpoint identity, workspace root, timezone, and safety bounds require a clean
daemon restart and are validated before claiming the lease.
