# Profile Adoption Decision

## Purpose

Record the deterministic profile matrix run for the prompt-kernel and tool-card
slice.

## Profiles

- `baseline`: current XML-like tool-call grammar with strict parser/admission.
- `strict-tool-card`: placeholder-rejecting rendered tool-call card.
- `field-spec-kernel`: strict card plus `ToolFieldSpec` value classes and
  prompt-kernel card-plan persistence.

## Evidence

- `baseline.md`: all protocol cases passed.
- `strict-tool-card.md`: all protocol cases passed.
- `field-spec-kernel.md`: all protocol cases passed.
- `tmp/live-runs/20260706T084837Z-live-smoke/summary.md`: endpoint task closed.
- `tmp/live-runs/20260706T085235Z-profile-smokes/strict-tool-card/summary.md`:
  endpoint task closed.
- `tmp/live-runs/20260706T085235Z-profile-smokes/field-spec-kernel/summary.md`:
  endpoint task closed.

## Decision

Adopt `field-spec-kernel` as the current implemented profile. It has deterministic
protocol evidence and bounded endpoint smoke evidence. Defer larger context
profile tuning until prompt cards are stored as queryable rows, not only body-ref
JSON.

## Rejected Or Deferred

- Old `<action>` envelopes remain rejected.
- Placeholder values remain parseable but admission-rejected before effects.
- Full-screen pane TUI is deferred; normal-screen workbench remains current.
