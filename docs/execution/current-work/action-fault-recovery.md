# Action Fault Recovery

## Purpose

This file owns the work to make malformed but recoverable action parameters
produce deterministic normalization or actionable refusal.

## Contract

- Normalize safe aliases before hard refusal.
- Never invent required semantic content such as summaries, questions, or file
  bodies.
- Report exact expected parameter names and one valid action example.
- Record normalization as an observable recovery event.
- Route repeated parser-level and parameter-level faults through a schema-repair
  mission, not generic parse recovery.
- Keep recovery examples concrete and path-scoped when the current artifact
  root or weak path is known.
- Classify attribute-like tags such as `<path=stories/chronos-fracture</path>`
  before registry validation and change the recovery route on repeated faults.

## Batch-Write Recovery Contract

`fs.batch_write` has a special live failure class: missing `files` plus an
unknown parameter whose name is a relative path. The recovery controller must
classify these parameter shapes:

- missing `files`;
- path-shaped unknown parameters;
- duplicate paths;
- missing `content:`;
- oversized file;
- oversized batch;
- top-level JSON action rejection;
- JSON-in-files recovery;
- unsupported child tags.

Safe normalization may convert path-shaped unknown parameters into the
canonical `files` payload only when every unknown parameter name is a relative
path, each value is content, total limits pass, no duplicate path exists, and no
semantic content is invented. Absolute paths, parent traversal, duplicate
paths, missing content, nested actions, and oversized payloads refuse before
mutation.

If normalization is unsafe, the refusal must render the canonical line protocol
with the actual current path or an artifact weak path when available:

```text
<action>
<tool>fs.batch_write</tool>
<files>
path: stories/chronos-fracture/catalog.toml
content:
[artifact]
root = "stories/chronos-fracture"
kind = "story"
</files>
</action>
```

A refusal that uses a generic placeholder instead of the current artifact path
is not acceptable for a live artifact recovery route.

## Repeated Fault Routing

The same schema fault must not keep producing the same single failing action
class. Route selection is:

1. first same schema fault: render the exact valid action form;
2. second same schema fault: switch to `artifact.next` or `graph.state` when the
   snapshot says inspection can pick a safe weak path;
3. third same schema fault: run a deterministic inspection effect or record a
   blocked handoff when no internal route remains.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Source: `crates/lkjagent-runtime/src/recovery.rs`
- Source: `crates/lkjagent-runtime/src/step/fault_wait.rs`
- Source: `crates/lkjagent-graph/src/source_recovery.rs`
- Tests: `crates/lkjagent-tools/tests/batch_write_formats.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Tests: `crates/lkjagent-runtime/tests/fault_wait.rs`
- Tests: `crates/lkjagent-runtime/tests/authority_recovery_plan.rs`
- Tests: `crates/lkjagent-graph/tests/graph.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- `graph.state` with a harmless location parameter loops on a parse notice.
- `doc.scaffold` receives `path` and fails instead of using `root`.
- `fs.batch_write` receives `stories/chronos-fracture/catalog.toml` as an
  unknown parameter while `files` is missing.
- The same invalid action is retried without a new recovery strategy.
- A refusal example parses but dispatch later rejects it.

## Status

partially implemented. Safe alias normalization and canonical examples exist
for covered cases. Dispatchable registry examples now parse, validate, and
reach tool routes in focused tests. Recovery-plan examples parse, validate, and
are admitted by recovery policy when model-authored. Safe path-shaped
`fs.batch_write` parameters now normalize into `files`; absolute, duplicate,
and empty-content path parameters refuse before mutation; the Chronos catalog
path shape has focused tool coverage; `graph.plan` conditional `checks|paths`
refuses before dispatch; and attribute-like tag output now gets a dedicated
parse fault plus concrete `<paths>` graph-plan repair before registry validation.
Repeated attribute-like parser faults now switch to `graph.state` inspection
before a third-fault blocked-handoff notice. Route-level proof across every
policy path remains open.
