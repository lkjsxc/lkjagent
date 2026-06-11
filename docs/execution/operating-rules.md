# Operating Rules

## Purpose

Defaults for autonomous sessions: the decisions an agent makes alone, the
decisions it must surface, and the default answer when a contract is
silent. These rules keep unattended work safe without permission prompts.

## Decide Alone

- Module and function naming, file splits under the cap, test case
  selection beyond the contract minimum.
- Choosing among implementations that all satisfy the written contract;
  prefer the one with fewer states, then fewer lines.
- Fixing any doc defect found in passing: dead link, drifted table, banned
  token; include the fix in the session commit.
- Extending a test table with cases discovered during implementation.

## Surface to the Owner

- Any change to a decision record's Decision section.
- Any new dependency beyond those named in
  [../decisions/rust-workspace.md](../decisions/rust-workspace.md).
- Any change to the scope boundaries in [../vision/scope.md](../vision/scope.md).
- Any budget change in [../architecture/context/budgets.md](../architecture/context/budgets.md)
  that alters a cap by more than a factor of two.

Surface means: write the open question into the task file, state it in the
handoff, and stop that line of work; it never means guess and continue.

## Default Answers

| Question | Default |
| --- | --- |
| The contract is ambiguous; implement my reading? | No. Fix the contract first, per [../agent/work-loop.md](../agent/work-loop.md). |
| Synthetic data to make a behavior demonstrable? | Tests only, labeled; never in product paths, per [../agent/honest-state.md](../agent/honest-state.md). |
| Keep a failing path alive with a stub? | No. Leave the task open and say so. |
| Two contracts disagree? | The more specific file wins on detail; the decision record wins on direction; fixing the disagreement becomes the first task. |
| Should this work preserve existing stored data? | No. Reset semantics are policy; say so in the handoff. |
| A gate fails on something unrelated to my change? | Fix it if one session-step suffices; otherwise record it as a new blocker row and continue. |
| Commit now or batch further? | Commit now, per [../repository/commit-protocol.md](../repository/commit-protocol.md). |

## Sequencing

Take blockers in order; the order encodes dependencies, not preference.
Skipping ahead requires stating in the handoff why the skipped blocker did
not block, and updating its row with that evidence.
