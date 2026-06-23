# Model Log

## Purpose

Track the work needed to make model-run evidence inspectable as both a bounded
handoff snapshot and replayable provider exchange records.

## Contract

The current Markdown handoff remains available through `lkjagent model-log`.
Raw provider request and response evidence is recorded under the contract in
[../../architecture/observability/provider-exchange-log.md](../../architecture/observability/provider-exchange-log.md).

## Implementation Task

- Keep `data/logs/current-model-run.md` as the synthesized owner handoff.
- Keep per-call provider exchange records under `data/logs/model/`.
- Persist provider exchange rows in SQLite.
- Redact secrets before writing files or store rows.
- Record parsed-action, admission, observation, index, and export files for
  every turn.
- Add CLI list, show, export, and raw-case inspection commands.

## Inputs

- provider request and response wire structs.
- runtime authority decision ids and prompt frame fingerprints.
- parser result or parse fault.
- admission result.
- tool observation or runtime error observation.
- redaction rules.

## Outputs

- atomic request, response, authority, timing, and error files.
- atomic parser, admission, observation, index, and export files.
- `provider_exchange` store rows with hashes and status.
- sanitized reproduction archive.
- focused tests for empty content, interrupted output, stop closure, admission
  before dispatch, and CLI inspection.

## Verification

- `cargo test -p lkjagent-store provider_exchange`
- `cargo test -p lkjagent-runtime model_log`
- `cargo test -p lkjagent-cli model_log`
- `cargo run -p lkjagent-xtask -- quiet verify`

## Status

partially implemented. The current Markdown handoff, provider exchange store
rows, request files, authority files, response files, timing files, and error
files exist for daemon provider calls with a log root. Parser files, admission
files, observation files, index records, exports, and CLI inspection remain open.
