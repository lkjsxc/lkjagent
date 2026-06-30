# Runtime Smoke

## Purpose

Define deterministic and live smoke evidence for runtime claims.

## Deterministic Replay

`cargo run -p lkjagent-xtask -- smoke replay` runs without an endpoint. It
reads checked-in log fixtures, writes a bounded summary to
`tmp/runtime-smoke-replay/summary.txt`, and prints one success line. The summary
names case id, decision ids, root, paths, word count, completion gate result,
observed fixture counts, and token aggregate fields.

The replay covers these historical failure classes:

- missing-root loops;
- generic roots;
- false closes;
- provider anomalies;
- incomplete manuscript paths.

## Live Smoke

`cargo run -p lkjagent-xtask -- smoke live` is explicit. When endpoint config is
absent it writes `tmp/runtime-smoke-live/summary.txt` with a skipped status and
exits 0. When endpoint config is present it still records that an operator must
run the live daemon smoke deliberately; quiet verification never contacts a
secret endpoint.

## Artifact Bounds

Smoke artifacts stay under `tmp/` and contain summaries, not generated novels or
large logs. Checked-in historical fixtures remain under `data/logs/` until an
owner chooses to replace them with inspected evidence.

## Status

implemented.
