# Prompt Kernel

## Purpose

Define the small ordered card set that every model request receives from a
persisted runtime decision.

## Card Order

Prompt assembly renders cards in this order:

1. kernel: product invariant, case id, decision id, and profile names;
2. objective: owner request, active operation, and success checks;
3. state: active cells, blockers, deadlines, and stale evidence needs;
4. facts: clean context items with ids, sources, and inclusion reasons;
5. conflicts: unresolved contradictions and resolution options;
6. recovery: bounded repair diagnosis when retrying or recovering;
7. tools: exact decision-visible tool cards and fingerprint;
8. output: the required envelope and copyable shape.

## Fingerprints

Each card has a deterministic fingerprint over canonical data. The prompt frame
stores card ids, section fingerprints, included context ids, excluded context ids
with reasons, and the final context-frame fingerprint. `prompt_cards.reason`
keeps compact `item_id:reason` pairs queryable, while provider exchange rows and
proof bundles carry the same frame fingerprint.

## Profiles

Prompt and context profiles are named data, not hidden prompt text. Experiment
runs may compare profiles such as `kernel-current`, `tool-card-strict`, or
`clean-context-small`. A profile is adopted only when docs, renderer, parser,
admission, tests, and proof evidence agree.

## Exclusions

Contaminated, stale, recovery-only, and owner-sensitive items are excluded from
normal fact cards. Conflict cards summarize contradictions without replaying raw
failed output. Repair cards quote bounded fault facts, not the full failed model
body.

## Acceptance Checks

Tests should prove card order is stable, contamination and conflict exclusions
are listed in card reasons, output cards are never truncated away, and profile or
card changes alter the frame fingerprint.
