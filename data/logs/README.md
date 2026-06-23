# Runtime Logs

## Purpose

Describe generated model and runtime logs under `data/logs`.

## Files

- `current-model-run.md`: bounded Markdown handoff for an owner-selected
  external model.
- `model/`: per-provider exchange records written by the daemon when model
  calls run through a configured log root.
- `index.ndjson`: optional exchange index when the runtime writer emits it.

## Provider Exchange Layout

Provider exchange directories use this shape:

```text
model/epoch-<created-at>/case-<case-id>/turn-000001/
  request.json
  authority.json
  response.json
  parsed-action.json
  admission.json
  observation.txt
  timing.json
  errors.ndjson
```

## Redaction

Request and response files are written after redaction. Logs do not store API
keys, authorization headers, full environment dumps, host secrets, private
tokens, or unredacted remote endpoint credentials.

## Inspection

Use `lkjagent model-log --print` for the current Markdown handoff. Use focused
filesystem inspection or the model-log CLI inspection commands once implemented
for per-provider exchange records.

## Git Policy

Generated large logs are diagnostic evidence. Commit only small intentional
fixtures or README files. Runtime databases and sidecar files remain ignored.
