# Status

## Purpose

Define stable `lkjagent status` output for operators and tests.

## Shape

`lkjagent status` is a snapshot command. It prints plain text, one fact per
line, with stable keys. The target contract groups keys by prefix rather than
human-only headings so scripts and tests can compare exact output.

## Required Sections

| Prefix | Required facts |
| --- | --- |
| `runtime.` | daemon state, turn count, continuation epoch, checkpoint reason |
| `queue.` | pending count, blocked count, newest queue id |
| `task.` | active case id, objective preview, phase, node, owner question |
| `authority.` | decision id, prompt frame id, mission, admitted tools, blocked tools |
| `artifact.` | root, kind, profile, plan status, atom counts, next path, active contract, measured total, readiness, blockers |
| `context.` | used, window, percent, pressure, prefix, log, reserve, headroom |
| `tokens.` | fields from [token-output.md](token-output.md) |
| `model.` | current handoff path and latest provider anomaly |
| `next.` | next executable action and missing evidence |

A missing optional fact renders as `unknown` when the fact exists but the value
was not reported, and as `none` when no row exists. Token counts use `unknown`
for omitted provider fields.

## Example

```text
runtime.daemon_state=working
runtime.turns=12
queue.pending=1
task.active_case=7
task.phase=execution
authority.decision_id=42
authority.mission=owner_execution
artifact.root=stories/second-period-first-love
artifact.profile=manuscript
artifact.atom_total=30
artifact.atom_ready=12
artifact.next_path=stories/second-period-first-love/manuscript/scenes/chapter-07/scene-01.md
artifact.active_contract=contract-7
context.used=12.34K
context.window=24.58K
context.pressure=green
tokens.latest.input=1.24K
tokens.latest.output=512
tokens.task.total=8.19K
tokens.session.cache_ratio=0.42
model.current_log=data/logs/current-model-run.md
next.action=fs.batch_write
next.missing=manuscript-word-count
```

## Artifact Progress

The artifact progress lines are store projections, not parsed transcript text.
When no active artifact exists, each optional artifact fact renders `none`. When
a plan exists, status and watch show atom totals, ready atoms, missing atoms,
next atom, exact next path, active contract id, measured total, accepted floor,
assembly pending state, readiness, and completion blockers.

## Tests

CLI tests must assert exact output for stopped, waiting, working, error, missing
usage, and active-artifact stores.
