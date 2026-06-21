# Escape Hatches

## Purpose

Define legal runtime correction when graph-selected tools contradict required
evidence or recovery needs.

## Contract

An escape hatch is deterministic authority correction, not an unsafe shell
escape. If graph state says completion but artifact evidence is missing, the
runtime must transition to verification or repair and admit read, audit, and
repair tools. If graph recovery blocks the only productive escape tool, runtime
authority records a policy contradiction and selects a mission that admits the
escape.

## Invariants

- Graph policy is advisory when it conflicts with completion evidence.
- Runtime correction must record active node, blocked tool, missing evidence,
  contradiction reason, and next valid action.
- The correction must not use shell unless the selected recovery node admits
  shell and no safer native tool can continue.
- `agent.done` remains refused until the completion gate passes.

## Failure Cases

- Completion node blocks `artifact.audit` while content readiness is missing.
- Recovery node blocks `fs.batch_write` after payload-too-large recovery.
- Maintenance policy renders while owner repair is still pending.
- Compaction asks for `memory.save` while graph policy blocks memory tools.

## Verification

Tests cover graph-completion-with-missing-artifact, recovery-with-blocked-batch,
maintenance-during-owner-work, and compaction-without-model-memory-save.

## Related Files

- [policy.md](policy.md)
- [tool-affordances.md](tool-affordances.md)
- [completion.md](completion.md)
- [../runtime/authority/tool-admission.md](../runtime/authority/tool-admission.md)
