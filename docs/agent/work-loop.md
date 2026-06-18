# Work Loop

## Purpose

The session loop for a coding agent in this repository, from first read to
handoff. It is the agent-facing rendering of
[../repository/workflow.md](../repository/workflow.md); the gate table
lives there and is not repeated here.

## The Loop

1. Read [../current-state.md](../current-state.md). It says what is real.
2. Take the work: the user-named task, or the first open blocker in
   [../execution/current-blockers.md](../execution/current-blockers.md),
   then open its task file under [../execution/tasks/](../execution/tasks/README.md).
3. Match a skill in [skills/README.md](skills/README.md) by trigger line;
   load it and read every file its Context section names before editing
   anything.
4. Write the contract change first: the architecture or product doc, the
   decision record when a settled decision moves, and
   [../current-state.md](../current-state.md) in the same change.
5. Make the narrowest implementation that satisfies the contract. Honor
   [../repository/functional-style.md](../repository/functional-style.md)
   and the 200-line cap as you go, not as a cleanup pass.
6. Prove it: the focused test from the task file's gate, run for real.
7. Update the task file Status and the blocker row; a task is complete only
   when docs, implementation, tests, and recorded evidence all moved.
8. Run the pre-handoff gate for the change class, commit per
   [../repository/commit-protocol.md](../repository/commit-protocol.md),
   and hand off per [handoff.md](handoff.md).

## Session Discipline

- One blocker per session unless they are trivially coupled; depth beats
  breadth.
- When a contract is ambiguous, fix the contract first; implementing an
  interpretation buries the ambiguity, and
  [honest-state.md](honest-state.md) forbids burying.
- When blocked, write the open question into the task file and hand off
  honestly; a clean stop is a good outcome.
- Never leave the tree dirty at handoff: committed, or reverted, with the
  choice stated.

## Self-Maintenance Sessions

When explicitly asked to maintain rather than build, mirror the runtime's directives
from [../architecture/runtime/self-maintenance.md](../architecture/runtime/self-maintenance.md):
sharpen a stale skill against recent session evidence, prune contradictions
between docs, or distill recurring friction into a skill. The
[skills/agent-maintenance.md](skills/agent-maintenance.md) skill owns the
procedure.
