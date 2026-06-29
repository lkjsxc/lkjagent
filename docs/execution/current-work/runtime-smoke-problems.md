# Runtime Smoke Problems

## Purpose

Record the current problems found by live Docker smoke runs after the obligation
network root-repair implementation. This page is written for handoff to another
reasoning system and for future implementation work.

## Evidence Sources

These transcripts are local smoke evidence and are not tracked fixtures:

- `tmp/user-story-smoke-data-fix/logs/current-model-run.md`: neutral long-novel
  title smoke that eventually reached `open_task=none`.
- `tmp/problem-probe-compact-data/logs/current-model-run.md`: long-novel title
  containing `Compact` that closed incorrectly.
- `tmp/user-smoke-data/logs/current-model-run.md`: earlier `Compact Smoke Novel`
  run with the same classification failure pattern.

The checked-in `data/logs/current-model-run.md` remains historical failure
evidence for the missing-root loop.

## Summary

Root-missing and structure-failed audit facts can now escape to contracted
`fs.batch_write`, and the neutral long-novel smoke completed. The runtime still
has serious quality gaps: classification can choose the wrong family, root names
can lose the owner title, early actions can drift before the contract route, and
completion can close a long-novel task with a small story-bible seed rather than
a real long-form deliverable.

## Problem 1: Compact Keyword Misclassifies Novel Work

`crates/lkjagent-graph/src/classify.rs` checks `lower.contains("compact")`
before long-content detection. A novel title containing `Compact` can be routed
as compaction-related work instead of a content artifact.

Observed in `tmp/problem-probe-compact-data/logs/current-model-run.md`:

```text
request = Create a long novel named "Compact Compass" ...
final_open_task=none
missing_root=0
fs.batch_write=0
doc.audit=0
artifact.next=0
artifact.audit=0
graph.evidence=1
agent.done=1
```

The action sequence ended as:

```text
memory.find -> graph.plan -> fs.mkdir -> workspace.summary -> graph.evidence
-> agent.done
```

Impact: the runtime can falsely close a long-novel request without artifact
identity, document audit, artifact audit, or contracted content writes.

## Problem 2: Root Identity Loses The Owner Title

The successful neutral smoke used the owner title `Harbor Lantern`, but the
artifact root became `stories/novel-named`. That root is generic and does not
preserve the owner's semantic identity.

Observed in `tmp/user-story-smoke-data-fix/logs/current-model-run.md`:

```text
doc.audit(stories/novel-named)
artifact.next(stories/novel-named)
artifact.audit(stories/novel-named)
```

Impact: artifacts are harder to find, audit, resume, and compare against the
owner objective. Future writes may also collide across unrelated novel tasks.

## Problem 3: Root Repair Still Has Noisy Pre-Repair Turns

The neutral smoke completed, but it still showed repeated audit and refused or
unhelpful actions before the final repair route.

Observed counts from the successful smoke:

```text
missing_root=4
fs.mkdir=2
fs.write=5
doc.audit=9
artifact.next=10
repeat action refused=1
authority refused=1
```

The desired route is a fast conversion from missing-root facts to a root identity
contract. The current route can still spend turns on same-root `doc.audit`, a
refused `fs.mkdir`, and unrelated small writes before `artifact.next` creates
the useful root identity contract.

Impact: the loop is not fully eliminated; it is now escapable, but costly and
model-dependent.

## Problem 4: Recovery Example Can Use The Wrong Root

The successful smoke included `artifact.next(stories/example-story)` before the
correct `artifact.next(stories/novel-named)` action. That means at least one
recovery or prompt example still leaks a placeholder root.

Impact: the model can waste turns or create contracts for the wrong root. This
also weakens the claim that the persisted decision is the only next-action
surface.

## Problem 5: Completion Is Too Weak For Long Novel Requests

The successful smoke reached `agent.done`, and audit gates passed, but this does
not prove that a long novel or complete detailed setting bible exists. The root
identity and repair contracts require small story signals, not scale-appropriate
coverage for a long-form novel.

Impact: the runtime can satisfy the current gates with a compact story-bible
seed. The owner asked for detailed structured settings, locations, continuity
notes, and a cast bible; the completion gate should require profile-scale
coverage and stronger artifact readiness evidence.

## Problem 6: Success Evidence Is Not A Durable Regression Fixture

The best success transcript lives in `tmp/user-story-smoke-data-fix/`. It is
useful local evidence but not a tracked benchmark or replay fixture.

Impact: future changes can regress the same behavior without a deterministic
corpus failure. The compact-title false close and the noisy root repair should
both become focused tests or replay fixtures.

## Recommended Next Work

1. Fix classification so artifact and long-content signals outrank incidental
   title words such as `compact`.
2. Derive artifact roots from owner title identity and verify root-objective
   alignment.
3. Convert missing-root facts to root identity contracts immediately, without
   same-root audit repeats or placeholder-root recovery examples.
4. Expand structure-failure contract coverage beyond `h1_count` examples.
5. Strengthen story readiness and completion gates for long-novel scale.
6. Add replay or benchmark fixtures for:
   - compact-title false close;
   - generic `stories/novel-named` root drift;
   - placeholder `stories/example-story` recovery root;
   - successful root repair with no repeated same-root audit.

## Related Task

The executable task is
[../tasks/runtime-smoke-problem-sweep.md](../tasks/runtime-smoke-problem-sweep.md).
