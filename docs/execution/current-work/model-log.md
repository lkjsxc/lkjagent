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
- Keep parsed-action, admission, observation, and index files for every
  model-authored tool turn.
- Classify provider anomalies before parse records claim ordinary parser faults.
- Add export files for every turn.
- Export manifests list only files present on disk or explicit missing-file
  records with reasons.
- Keep CLI list and show inspection commands.
- Add export and raw-case inspection commands.

## Inputs

- provider request and response wire structs.
- runtime authority decision ids and prompt frame fingerprints.
- parser result or parse fault.
- admission result.
- tool observation or runtime error observation.
- redaction rules.

## Outputs

- atomic request, response, authority, timing, and error files.
- CLI list and show inspection output.
- atomic parsed-action, admission, observation, and index files.
- atomic export files.
- `provider_exchange` store rows with hashes and status.
- sanitized reproduction archive.
- focused tests for empty content with usage, interrupted output, stop closure,
  admission before dispatch, export manifest integrity, and CLI inspection.

## Verification

- `cargo test -p lkjagent-store provider_exchange`
- `cargo test -p lkjagent-runtime model_log`
- `cargo test -p lkjagent-cli model_log`
- `cargo run -p lkjagent-xtask -- quiet verify`

## Status

partially implemented. The current Markdown handoff, provider exchange store
rows, request files, response files, timing files, error files, per-turn export
manifests, parsed-action, admission, observation, index files, prompt-frame ids
on exchange rows, CLI list and show, raw-case inspection, and sanitized replay
export commands with raw turn-file copying exist for daemon provider calls with
a log root. Provider anomalies are logged before parsing for new responses, and
export manifests list only files present on disk. When a previously listed file
is absent during manifest refresh, the export records it under `missing_files`
with reason `listed_file_absent`. Provider anomaly exports use status
`provider_anomaly` instead of `succeeded`. New authority files include persisted
decision id, prompt frame id, authority fingerprint, kernel mission, and
staleness fingerprint. Live replay proof is tracked by the verification plan.
