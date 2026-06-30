# Completion Policy

## Purpose

Define the authority-owned gate that every owner-task close path must use.

## Decision Owner

`lkjagent-runtime` owns completion policy. The graph can recommend completion,
and the model can request `agent.done`, but the reducer decides close
eligibility.

## Gate Inputs

Every close path reads the same typed `CompletionGateInput` packet:

- case id, objective, owner request, active mode, and active node;
- intent kind, artifact root, artifact kind, profile, requested scale, and
  lifecycle when artifact work exists;
- story manuscript lifecycle, target words, word floor, chapter count, exact
  requested manuscript paths, missing manuscript paths, next manuscript path,
  and manuscript words written;
- required evidence ids and their owning audit or verification surface;
- latest document audit status, artifact readiness status, weak paths,
  profile-scale facts, and manuscript progress facts;
- verification command status or explicit not-applicable reason;
- unresolved recovery faults, provider anomalies, blocked handoff status, and
  progress key exhaustion;
- current decision id, prompt frame id, and staleness fingerprint.

## Gate

```text
completion_allowed(case) iff:
  objective exists
  plan evidence exists
  observation evidence exists
  task-kind required evidence exists
  declared verification passed or is not applicable
  artifact tasks have structure and readiness evidence
  recovery faults are resolved or represented in a blocked handoff
  unsupported claims are absent
```

The same gate is used by `agent.done`, graph close, maintenance close, recovery
handoff, turn-budget checkpoint, console close, and daemon shutdown handoff.
A checkpoint that lacks evidence refuses completion and continues with an
admitted internal next action when one exists.

## Refusal Shape

```text
CompletionRefusal
- failed_gate
- missing_evidence
- blocking_faults
- admitted_tools
- next_executable_action
- partial_handoff
```

The next action must parse, dispatch, and be admitted by the current authority
decision. Artifact close refusals keep audit, read, repair, and batch tools
admitted.

## Required Facts

Close requires resolved or explicitly blocked owner objective, present required
evidence, passing document or artifact audit when relevant, no scaffold-only
or weak leaves, requested manuscript prose paths and word floor when the
manuscript lifecycle is active, no active recovery fault, observed verification
result, and no unsupported verification claim.

## Prohibited States

- `agent.done` closes while content readiness is missing.
- A graph close bypasses the central gate.
- Failed audit is treated as a warning.
- Maintenance no-op work closes an owner objective.
- Partial completion is implied instead of explicitly recorded.

## Mechanical Tests Required

- `agent.done` after artifact planning is refused.
- `agent.done` after scaffold-only output is refused.
- Structure pass plus readiness failure is refused.
- Story-bible-only output is refused for manuscript lifecycle work.
- Verification pending is refused when the task kind requires verification.
- Turn-budget exhaustion writes a blocked partial handoff.
- The refusal next action parses and dispatches.

## Fixture

`false_completion_after_scaffold` proves scaffold output cannot close.
`cookbook_weak_content_audit` proves weak content leaves keep the case open.

## Verification

Run `cargo test -p lkjagent-runtime completion` and
`cargo test -p lkjagent-benchmark corpus`.

## Implemented Slice

The pure `decide_completion` reducer returns allowed status, completion kind,
failed gates, missing evidence, existing evidence, next executable action,
valid example, blocked-handoff allowance, and status text. Runtime admission
uses this reducer for `agent.done`. Maintenance completion remains separate
from owner completion. Missing-evidence refusals keep audit, read, repair, and
batch tools available. Daemon graph dispatch and authority evidence-gap snapshots
refuse `agent.done` when artifact-readiness evidence exists but the latest
artifact ledger row is not passed or still has weak paths. This is focused
coverage, not proof that every close path is fully wired.

## Status

partially implemented. Runtime admission uses the reducer for `agent.done`, and
the dense-network implementation records typed completion input packets and
persists them as dense authority rows. The remaining work is proving every
non-daemon close path reads the same packet.
