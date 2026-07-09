# Configuration Registry

## Purpose

Define the only accepted keys, types, bounds, reload policy, and consumers for
`data/lkjagent.json`.

## Shape And Precedence

The root is one flat JSON object. Every value is a string, integer, or boolean.
Nested objects, arrays, unknown keys, empty strings for required names, wrong
types, and out-of-range integers fail before the daemon starts. File values
override defaults; documented environment overrides take precedence over file
values. Secrets remain in named environment variables.

Type is `string`, `integer`, or `boolean`. Bounds are inclusive. Reload is
`hot` or `restart`.

## Endpoint And Runtime

| Key | Type | Default or bound | Reload | Consumer |
| --- | --- | --- | --- | --- |
| endpoint_url | string | http://127.0.0.1:8080 | restart | provider |
| endpoint_model | string | local-model | restart | provider |
| endpoint_api_key_env | string | LKJAGENT_API_KEY | restart | provider |
| endpoint_timeout_seconds | integer | 1..1800; 300 | hot | provider |
| endpoint_retry_limit | integer | 0..8; 3 | hot | provider |
| endpoint_backoff_milliseconds | integer | 50..60000; 500 | hot | provider |
| queue_wake_milliseconds | integer | 50..60000; 500 | hot | daemon |
| no_progress_window | integer | 1..10; 3 | hot | selector |
| live_campaign_seconds | integer | 840..7200; 900 | restart | evaluation |

## Prompt And Context

| Key | Type | Default or bound | Reload | Consumer |
| --- | --- | --- | --- | --- |
| prompt_context_tokens | integer | 2048..262144; 16384 | restart | compiler |
| prompt_output_reserve_tokens | integer | 256..32768; 2048 | restart | compiler |
| context_objective_tokens | integer | 64..4096; 512 | hot | selector |
| context_evidence_tokens | integer | 128..32768; 8192 | hot | selector |
| context_history_tokens | integer | 64..16384; 2048 | hot | selector |
| context_recovery_tokens | integer | 64..8192; 1024 | hot | selector |
| context_retrieval_limit | integer | 1..50; 12 | hot | retrieval |
| tool_view_max_items | integer | 1..4; 4 | restart | compiler |

## Workspace And Maintenance

| Key | Type | Default or bound | Reload | Consumer |
| --- | --- | --- | --- | --- |
| workspace_root | string | ../workspace | restart | workspace |
| timezone | string | Asia/Tokyo | restart | clock |
| workspace_file_max_tokens | integer | 64..512; 512 | restart | workspace |
| workspace_scan_debounce_milliseconds | integer | 50..10000; 500 | hot | scanner |
| workspace_reconcile_seconds | integer | 30..86400; 900 | hot | scanner |
| navigation_page_max_items | integer | 10..200; 80 | hot | indexer |
| archive_after_days | integer | 30..36500; 3650 | hot | maintenance |
| activity_retention_days | integer | 30..36500; 3650 | hot | maintenance |
| maintenance_interval_seconds | integer | 60..86400; 900 | hot | maintenance |
| effect_output_max_bytes | integer | 1024..1048576; 131072 | restart | effects |

## Recovery, Shell, And TUI

| Key | Type | Default or bound | Reload | Consumer |
| --- | --- | --- | --- | --- |
| recovery_parse_attempts | integer | 0..3; 1 | hot | recovery |
| recovery_output_limit_attempts | integer | 0..8; 4 | hot | recovery |
| recovery_effect_attempts | integer | 0..3; 2 | hot | recovery |
| shell_enabled | boolean | true | restart | tool view |
| shell_timeout_seconds | integer | 1..1800; 300 | hot | shell |
| tui_refresh_milliseconds | integer | 16..2000; 100 | hot | TUI |
| tui_history_messages | integer | 50..10000; 1000 | hot | TUI |

## Cross-Key Guards

`prompt_output_reserve_tokens` is smaller than `prompt_context_tokens`. Context
lane caps plus the stable-prefix maximum fit the remaining prompt context.
`workspace_file_max_tokens` never exceeds 512. Endpoint URL, model,
environment-variable name, workspace root, and timezone are nonempty. The
timezone resolves through the IANA database. Relative workspace paths resolve
from the configuration file directory, never the process working directory.
