# Uploaded Run Fixtures

## Purpose

Catalog owner-uploaded run-log failures as mechanical benchmark fixtures.

## Contract

Uploaded run fixtures model observable harness behavior, not one-off content
quality. Each fixture has a known-bad workspace or transcript pattern and a
known-good expectation that proves the runtime refuses false completion and
selects a productive next action.

## Fixture Shape

```text
UploadedRunFixture
- fixture_name
- transcript_slice
- initial_runtime_snapshot
- event
- expected_authority_decision
- expected_admitted_tools
- expected_blocked_tools
- expected_exact_next_action
- expected_evidence_delta
- expected_persistence_delta
```

Judges are deterministic. Control-plane fixtures do not score model prose
quality.

## Invariants

- Fixtures must detect the failure class, not the exact prose of one run.
- Known-good cases must prove repair, refusal, or completion evidence.
- Known-bad cases must fail when scaffolds, plans, or weak content are treated
  as completion.
- Fixture names must be stable and descriptive.

## Required Uploaded Cookbook Cases

- `uploaded-cookbook-parse-recovery`
- `uploaded-cookbook-maintenance-preemption`
- `uploaded-cookbook-large-write-overflow`
- `uploaded-cookbook-batch-write-schema-fault`
- `uploaded-cookbook-scaffold-only-readiness`
- `uploaded-cookbook-semantic-mismatch-bread`
- `uploaded-cookbook-recovery-tool-block`
- `uploaded-cookbook-turn-budget-handoff`
- `uploaded-cookbook-completion-refusal`
- `uploaded-cookbook-compaction-resume`

## Existing Failure Classes

- `recover-repeat-parameter-fault`: repeated `fs.list`, `fs.stat`, or
  `fs.read` parameter refusals must produce one canonical schema repair, no
  repeated `graph.state`, and no completion.
- `bread-dictionary-shallow-content`: a detailed dictionary request with a
  shallow bread terms file must fail content readiness and admit repair.
- `large-write-payload-risk`: raw large `fs.write` attempts must route to
  artifact batch planning.
- `completion-with-blocked-mutation`: graph completion with missing content and
  blocked mutation must refuse close and return to repair.
- `maintenance-during-owner-work`: maintenance must yield while owner work is
  active and must not create a memory loop.
- `cookbook-scaffold-false-ready`: scaffold-only cookbook output may pass
  structure but must fail content readiness and refuse `agent.done`.
- `cookbook-placeholder-batch`: `artifact.next` write contracts and direct
  write tools must reject scaffold phrases before overwriting richer content.

## Agent-Facing Report Matrix

- `japanese-cookbook-drift`: a Japanese-food cookbook must record a cookbook
  contract with Japanese food as subject, reject a bread profile, and keep
  ciabatta or focaccia paths absent unless explicitly requested.
- `document-structure-graph-evidence-bypass`: direct `graph.evidence` for
  `document-structure` is refused and the next action is `doc.audit`.
- `batch-write-payload-schema-fault`: JSON-like `fs.batch_write` payloads and
  child `<file>` blocks are refused before mutation; partial writes are absent.
- `shell-parameter-missing-command`: `shell.run` without `command` is refused
  with a canonical command-required repair and no timeout retry loop.
- `queue-story-interrupt`: cookbook and Japanese story owner tasks remain
  separate cases with no cross-case artifact contamination.
- `context-compaction-resume`: compaction creates a durable snapshot, passes a
  post-compaction check, and preserves missing evidence plus refused actions.
- `repeated-recovery-action`: a refused recovery action signature is blocked
  and the next action uses a different action class.
- `sf-novel-active-run`: the active novel run uses owner text
  `Create a SF novel. with detailed structured settings.` and root
  `stories/novel`. It keeps weak-content repair, batch-limit refusal,
  provider anomaly retry, audit-owned evidence refusal, and final verification
  pending under one authority decision stream. The fixture checks behavior and
  fails stale long-novel wording or sentence-like roots.

## Verification

`benchmark check-corpus` must materialize known-good and known-bad fixtures for
each uploaded failure pattern. Runtime tests may cover the same patterns at a
smaller unit boundary, but the benchmark corpus remains the durable owner-log
evidence.

## Related Files

- [mechanical-benchmarks.md](mechanical-benchmarks.md)
- [task-contract.md](task-contract.md)
- [../architecture/runtime/authority/README.md](../architecture/runtime/authority/README.md)
- [../architecture/artifacts/content-readiness.md](../architecture/artifacts/content-readiness.md)
