# Exchange Logs

## Purpose

Define the per-attempt file evidence written beside SQLite rows.

## Directory Shape

Each attempt writes files under this directory shape:

```text
data/logs/task-<task-id>/step-<ordinal>/attempt-<attempt-ordinal>/
  request.json
  response.json
  outcome.json
  timing.json
```

The path is stored in `attempts.exchange_ref`. The log body cap for proof
summaries is `proof.exchange.preview-bytes=0`, meaning proof bundles index
metadata and paths without copying model bodies.

## File Meanings

- `request.json`: endpoint request body, prompt fingerprint, max tokens, and
  stop sequence.
- `response.json`: endpoint response or provider anomaly body.
- `outcome.json`: parsed envelope or fault, diagnosis, closure mode, usage
  refs, cache metrics, and check result refs.
- `timing.json`: start time, end time, duration, and retry timing.

## Durable Owners

SQLite rows own resumable facts. Exchange files own large request and response
bodies. The attempt row owns the exchange path; token usage rows own usage
numbers; event and diagnosis rows own bounded owner-visible summaries.

## Integrity

An exchange directory without a committed attempt row is an orphan from a crash
mid-call and is ignored by resume. Proof bundles warn about orphans.
