# Cells

## Purpose

Define the durable state cell shape and unknown-key behavior.

## State Key

A state key is data, not a closed enum:

```text
StateKey = namespace + ":" + name
```

Suggested namespaces include `case`, `plan`, `tool`, `context`, `artifact`,
`recovery`, and `completion`. Names may contain scoped suffixes such as
`context:conflict/target-root` when the semantic key matters.

## Cell Fields

Each cell stores:

- case id;
- key namespace and name;
- status: active, inactive, suppressed, resolved, or blocked;
- priority and confidence;
- payload schema name;
- payload JSON;
- evidence refs;
- source event id;
- created and updated times;
- optional expiry or cooldown;
- optional conflict group; and
- optional parent or lineage key.

## Unknown Keys

Storage, hydration, diagnostics, and reducer plumbing must preserve unknown
state keys. Known helpers may decode common payload schemas, but a new state
cell must not require editing a central enum merely to survive a round trip.

## Evidence Rule

Every active cell records why it exists. Evidence may be an owner message, a
check result, an artifact fingerprint, a context item, a tool observation, or a
recovery event. Cells without evidence are invalid for completion decisions.

## Failure This Prevents

A future capability can add `tool:fs-read-needed` or `context:conflict-open`
without replacing the runtime authority model.
