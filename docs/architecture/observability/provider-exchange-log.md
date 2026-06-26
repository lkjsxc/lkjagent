# Provider Exchange Log

## Purpose

Define durable raw model input and output logging under `data/logs` for every
provider call made by the daemon.

## Relationship To Model Log

[model-log.md](model-log.md) owns the single synthesized Markdown handoff. This
file owns per-call JSON and text records used for replay, debugging, and
sanitized reproduction. The two logs do not replace each other.

## Directory Layout

```text
data/logs/
  README.md
  index.ndjson
  model/YYYY-MM-DD/case-<case-id>/turn-000001/
    request.json
    response.json
    parsed-action.json
    authority.json
    admission.json
    observation.txt
    timing.json
    errors.ndjson
```

Each file is a complete record. Writers create `*.tmp` files and rename them to
the final path after the write succeeds. A leftover `*.tmp` file is an orphaned
partial record and is reported by log inspection.

## Request Record

`request.json` stores:

- provider kind and endpoint URL with secret parts redacted or hashed.
- model name, case id, turn id, prompt frame id, and authority decision id.
- messages exactly as sent to the provider.
- sampling fields, token limit, stop sequences, and tool configuration.
- created timestamp and request hash.

## Response Record

`response.json` stores:

- raw response body after redaction.
- assistant content and provider reasoning fields when present.
- tool call fields when present.
- provider anomaly class when content is missing or unusable.
- finish reason, usage, provider stats, timing, and response hash.

Reasoning fields are logged as evidence only. They never drive parser output,
admission, graph state, or dispatch.

## Runtime Records

`authority.json` stores active mode, mission, kernel mission, kernel staleness
fingerprint, tool surface, preferred next action, recovery route, completion
gate, invariants, and decision id.

`parsed-action.json` stores parse status, closure mode, envelope mode,
normalized text hash, parse faults, tool name, parameters, and byte counts.

`admission.json` stores accepted or refused status, reason, schema findings,
repeat fingerprint, staleness fingerprint, and dispatch plan when accepted.

`observation.txt` stores the tool result or runtime observation shown to the
model. Redaction markers remain visible.

`errors.ndjson` stores one fault per line with class, message, route, retry
count, and prior action hash.

## Store Record

The SQLite store has a `provider_exchange` table with identifiers, case id,
turn id, prompt frame id, authority decision id, admission decision id,
provider, model, created timestamp, redacted request and response JSON, hashes,
finish reason, usage, stats, latency, status, error class, and redaction schema
number. Indexes cover case plus turn and creation time.

## Redaction

Logs never store API keys, authorization headers, full environment dumps, host
secrets, private tokens, or unredacted remote endpoint credentials. Local loopback
ports may be kept when needed for diagnosis; non-loopback endpoint URLs are
hashed or reduced to scheme and host class.

## CLI Surface

The CLI exposes implemented inspection commands:

```sh
lkjagent model-log list
lkjagent model-log show --case <case-id> --turn <n>
```

The export surface remains open:

```sh
lkjagent model-log export --case <case-id> --out tmp/repro-<case-id>.tar.zst
lkjagent log --raw --case <case-id>
```

The export command includes sanitized requests, responses, authority decisions,
admission decisions, observations, graph snapshots, artifact ledger snapshots,
and redacted configuration.

## Export Manifest Integrity

A turn export manifest must be self-consistent. Each file named in `files`
must exist in that turn directory at export time. If a logical artifact is
expected but absent, the manifest records it under `missing_files` with a stable
reason instead of listing it as present. An export with missing logical artifacts
uses a warning or failed status; it does not report `succeeded` while naming
absent files. Raw-case replay reads only files proven present plus explicit
missing-file records.

## Acceptance Criteria

Verification proves that a request creates `request.json`, a response creates
`response.json`, empty content with usage records a provider anomaly without
dispatch, interrupted output records `InterruptedGeneration`, stop-closure
repair appears in `parsed-action.json`, admission is persisted before dispatch,
observations are persisted after dispatch, export manifests list only existing
files or explicit missing-file records, and the CLI can inspect the records.

## Status

partially implemented. The runtime writes the current Markdown model handoff,
SQLite `provider_exchange` rows, and atomic request, authority, response,
parsed-action, admission, observation, timing, and error files for daemon
provider calls that have a log root. Parsed-action records include envelope
mode and normalized text hash. The CLI lists and shows provider exchange rows.
`index.ndjson` is written for exchange discovery. Export refresh tests prove
`files` entries are limited to files present in the turn directory, and missing
previously listed files are recorded in `missing_files` with stable reasons.
