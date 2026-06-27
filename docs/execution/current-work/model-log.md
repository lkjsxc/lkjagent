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
- Derive touched paths from artifact ledgers, write observations, and workspace
  events, not only direct graph evidence.

## Inputs

- provider request and response wire structs.
- runtime authority decision ids, prompt frame ids, and fingerprints.
- artifact ledger rows, write observations, scaffold observations, and workspace
  events for touched-path synthesis.
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
- active status fields for artifact root, weak cursor, latest decision id,
  prompt frame id, provider anomaly retry state, and next executable action.
- sanitized reproduction archive.
- focused tests for empty content with usage, interrupted output, stop closure,
  admission before dispatch, export manifest integrity, and CLI inspection.

## Verification

- `cargo test -p lkjagent-store provider_exchange`
- `cargo test -p lkjagent-runtime model_log`
- `cargo test -p lkjagent-cli model_log`
- `cargo run -p lkjagent-xtask -- quiet verify`

## Status

implemented. The current Markdown handoff, provider exchange rows, request and
response files, timing and error files, parsed-action, admission, observation,
index files, prompt-frame ids, raw-case inspection, sanitized replay exports,
and missing-file manifest records are store-backed. Provider anomalies are
logged before parsing, use status `provider_anomaly`, and avoid fake parse,
admission, or observation success records. Authority files include persisted
decision id, prompt frame id, authority fingerprint, kernel mission, and
staleness fingerprint. Touched-path synthesis reads graph evidence, artifact
ledger roots, and batch cursor write outcomes. Final evidence is the quiet
verify and Docker Compose final gate.
