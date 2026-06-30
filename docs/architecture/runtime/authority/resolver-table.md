# Resolver Table

## Purpose

Define the total resolver table that turns obligations and runtime facts into
one persisted decision rule.

## Contract

The resolver is a named table, not a fallback branch. Each rule has a stable
id, a predicate over mission plus facts, and one decision class:

| Rule id | Predicate | Decision class |
| --- | --- | --- |
| `runtime-effect` | hard compaction or closed idle | runtime effect |
| `close-case` | completion allowed and no missing evidence | close case |
| `semantic-write-contract` | root identity or content contract exists | `fs.batch_write` contract |
| `record-graph-plan` | planning evidence is missing | `graph.plan` |
| `inspect-artifact-plan` | artifact root is missing for artifact work | `artifact.plan` |
| `audit-doc-audit` | document structure must be audited | `doc.audit` |
| `audit-artifact-audit` | artifact readiness or verification must be audited | `artifact.audit` |
| `inspect-artifact-next` | weak paths or requested next batch exist | `artifact.next` |
| `record-fs-write` | non-artifact observation write is needed | `fs.write` |
| `record-graph-evidence` | non-audit verification evidence is needed | `graph.evidence` |
| `inspect-workspace-summary` | no artifact work exists and workspace facts are needed | `workspace.summary` |
| `record-memory-find` | idle maintenance is admitted | `memory.find` |
| `blocked-handoff` | no safe route remains | runtime blocked handoff |

The first matching rule wins after mission selection and obligation ordering.
A no-match owner case becomes `blocked-handoff` with exact missing facts; it
must not close the case as success.

## Persistence

Every decision stores:

- `resolver_plan` as `rule=<id> plan=<details>`;
- a dense `resolver_rule` row with the selected rule id;
- a dense `progress` row with the selected progress key;
- status state keys `authority resolver plan` and `authority progress key`.

`lkjagent status` exposes these as `authority.resolver_plan` and
`authority.progress_key`.

## Invariants

- Graph policy is input only; it never admits a tool after runtime refusal.
- Missing-root facts choose `semantic-write-contract`, not same-root audit.
- Repeated schema faults change rule or block with the remaining missing facts.
- Runtime effects require no model-authored action.
- Content writes require exact paths and stored limits.

## Verification

Focused resolver tests cover primary obligation rules, root identity repair,
content contracts, audits, and blocked handoff. Store tests prove resolver plan,
resolver rule id, and progress rows reopen for a decision.

## Status

implemented.
