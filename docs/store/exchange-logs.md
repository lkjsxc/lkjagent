# Exchange Logs

## Purpose

Define file evidence for large provider exchange bodies referenced by SQLite
rows.

## Directory Shape

Each provider call writes files under this directory shape:

```text
data/logs/matter-<matter-id>/decision-<decision-id>/exchange-<exchange-id>/
  request.json
  response.json
  outcome.json
  timing.json
```

The path is stored in `provider_exchanges.exchange_ref`. Proof summaries index
metadata and paths without copying model bodies by default.

## File Meanings

- `request.json`: endpoint request body, decision id, prompt-frame id, prompt
  fingerprint, context-frame fingerprint, tool-view fingerprint, active timeout,
  max tokens, and stop sequence.
- `response.json`: endpoint response or provider anomaly body.
- `outcome.json`: parsed envelope or fault, diagnosis, closure mode, admission
  refs, usage refs, cache metrics, and check result refs.
- `timing.json`: start time, end time, duration, active timeout, and retry
  timing.

## Durable Owners

SQLite rows own resumable facts. Exchange files own large request and response
bodies. The decision row owns turn authority; the provider-exchange row owns the
exchange path; token usage rows own nullable usage numbers; events and context
items own bounded owner-visible summaries.

## Integrity

An exchange directory without a committed provider-exchange row is an orphan from
a crash mid-call and is ignored by resume. Proof bundles warn about orphans.
