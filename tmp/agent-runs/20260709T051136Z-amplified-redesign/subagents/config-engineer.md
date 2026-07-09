# Config Engineer Report

## Scope

- Lane file read: `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/config-engineer.md`.
- Required context read: `docs/current-state.md`,
  `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/README.md`, and the
  candidate config docs/source files.
- Report only. No product docs or source were edited.

## Current Facts

- The lane mission says to add flat `data/lkjagent.json` loading and docs
  without nested config.
- Current code already reads `data_dir/lkjagent.json` through
  `load_flat_config()` in `crates/lkjagent-app/src/config.rs:47`.
- `load_client()` reads flat `endpoint_url`, `endpoint_model`,
  `endpoint_timeout_seconds`, and `endpoint_api_key_env`, with environment
  overrides for URL, model, timeout, and API key.
- `workspace_root()` reads flat `workspace_root`, defaults to `workspace`, and
  resolves relative paths under the data directory.
- `prompt_max_context_tokens()` and `live_campaign_seconds()` read flat config
  keys with environment overrides.
- `load_flat_config()` also rewrites an older nested `endpoint` object into
  flat keys, then writes the flattened JSON back to the same file.
- The nested rewrite preserves existing flat keys when both old and new keys
  exist.
- `docs/product/daemon.md:9` says daemon startup loads flat configuration.
- `docs/operations/running.md:9` says configuration lives in flat
  `data/lkjagent.json` plus environment overrides.
- `docs/operations/running.md:13` lists the flat keys:
  `endpoint_url`, `endpoint_model`, `endpoint_api_key_env`,
  `endpoint_timeout_seconds`, `workspace_root`,
  `prompt_max_context_tokens`, and `live_campaign_seconds`.
- `docs/operations/running.md:23` says older nested config keys may be read only
  to rewrite them into the flat file.
- `docs/llm/endpoint.md` says endpoint URL, model, API key env name, timeout,
  and context length come from `data/lkjagent.json` plus environment overrides.
- `crates/lkjagent-app/tests/endpoint.rs` has focused coverage for nested
  endpoint migration, configured endpoint use, flat workspace root, prompt cap,
  and live campaign seconds.
- `diagnostics_support.rs` reports endpoint source as env, config, or absent
  without exposing secret values.

## Contradictions

- The mission reads as unimplemented work, but the current checkout already has
  flat config loading, flat docs, and focused tests.
- "Without nested config" conflicts with the current compatibility shim that
  reads and rewrites nested `endpoint` config. The docs explicitly allow that
  old nested keys may be read only for rewrite.
- `docs/context/budgets.md` names budget constants such as
  `context.request.hard-cap-tokens=8000`, but current config source exposes
  only `prompt_max_context_tokens`, not a general config mapping for those
  dotted budget names.
- `docs/tools/guards.md` says runtime decisions carry tool, token, and retry
  budgets, but current config source does not load guard budgets from
  `lkjagent.json`; it only loads endpoint, workspace, prompt cap, and live
  campaign seconds.
- `load_flat_config()` silently accepts non-object JSON by returning it. That is
  harmless for missing flat keys, but it does not enforce "flat object" shape.
- `env_number()` silently ignores invalid numeric environment values by falling
  back to config/defaults. If invalid env should be a configuration error, source
  and docs currently disagree by omission.

## Exact Docs Edits

- No product docs edited by this lane.
- If retaining compatibility migration, no required docs edit: current
  `docs/operations/running.md` already documents flat config and nested-key
  rewrite.
- If strict "without nested config" means no nested input may be accepted, edit
  `docs/operations/running.md` to remove the nested rewrite allowance and state
  that nested objects are rejected rather than migrated.
- Clarify `docs/context/budgets.md` if `prompt_max_context_tokens` is the only
  owner-configurable context budget. Otherwise add the exact flat keys for each
  configurable budget and update source/tests accordingly.
- Clarify `docs/tools/guards.md` if guard budgets are persisted in
  `RuntimeDecision` only and are not loaded from `data/lkjagent.json`.
- `docs/current-state.md` needs no config update unless strict nested rejection
  or new budget/guard keys are implemented.

## Exact Source Edits

- No source edited by this lane.
- If strict flat-only behavior is required, change
  `crates/lkjagent-app/src/config.rs` to reject or ignore the nested `endpoint`
  object instead of rewriting it, then update the endpoint migration test.
- If current compatibility behavior is desired, no source edit is required for
  the lane mission.
- Consider making `load_flat_config()` reject non-object JSON and nested objects
  other than the explicitly documented migration shape if stricter validation is
  part of the acceptance contract.
- Consider returning an error for invalid numeric env vars if operator mistakes
  should not be silently ignored.

## Tests To Add Or Update

- If strict flat-only is required, update
  `llm_endpoint_uses_configured_chat_endpoint` to use flat JSON and add a
  rejection test for nested `endpoint`.
- If compatibility migration remains accepted, keep the existing migration test
  and add an assertion that flat keys win when both nested and flat values exist.
- Add a test for invalid JSON shape, for example array/string config, once the
  desired behavior is chosen.
- Add a test for invalid numeric env overrides if the code is changed to error.
- Add docs/source parity tests for any newly documented budget or guard config
  keys.

## Commands Run

- `cargo test -p lkjagent-app --test endpoint` -> passed, 2 tests.
- `cargo run -p lkjagent-xtask -- check-docs` -> passed, `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines` -> passed, `ok check-lines`.

## Commands To Run For Full Gate

- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify`
- If config behavior changes, rerun `cargo test -p lkjagent-app --test endpoint`
  and any added config tests before the full gates.

## Risks

- Removing nested migration could break existing local `data/lkjagent.json`
  files even though current docs permit one-time rewrite.
- Leaving silent invalid numeric env fallback may hide operator configuration
  mistakes during live endpoint runs.
- Expanding config to cover all budget/guard docs could create a second policy
  plane unless those values are still persisted through `RuntimeDecision` rows.
- The config loader rewrites `lkjagent.json` during reads; that is convenient for
  migration but may surprise operators who expect read-only diagnostics.

## Acceptance Items Affected

- Flat owner configuration for endpoint, workspace, prompt cap, and live
  campaign duration is currently implemented and tested.
- Nested config removal is not accepted as strict behavior because current docs
  and tests preserve migration.
- Budget and guard configuration acceptance remains limited to the keys actually
  loaded by `config.rs`; broader budget/guard docs are not implemented config
  keys.
- Final acceptance should require either an explicit decision to keep migration
  compatibility or a strict flat-only code/docs/test change.
