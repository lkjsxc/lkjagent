# Edges

## Purpose

Define generic state-edge evidence for dense relations without adding a second
graph authority.

## Edge Shape

A state edge is durable evidence with stable refs:

```text
StateEdge {
  id,
  scope,
  from_ref,
  to_ref,
  relation,
  reason,
  evidence_refs,
  created_at,
  source_event_id,
  status
}
```

Refs may target state keys, context items, artifacts, workspace records, checks,
provider exchanges, owner messages, proof rows, or unknown future types.

## Relations

Initial relation labels are `depends-on`, `blocks`, `blocked-by`,
`derived-from`, `supersedes`, `conflicts-with`, `resolves`, `verifies`,
`schedules`, `owns`, `references`, `tags`, and `repeats`. Unknown labels are
preserved for round trip and diagnostics.

## Authority Rule

Edges inform selector candidates, context admission, staleness, proof, and
status. They do not execute effects, grant tools, close cases, or replace the
persisted `RuntimeDecision` selected for a turn.

## Reducer Rule

Events may add edges or suppress edges. Suppression keeps the row and changes its
status with a reason so proof can explain why a relation no longer participates
in selection.
