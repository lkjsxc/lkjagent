# Handoff

## Purpose

How a session ends: the final report format and the rules that make it
trustworthy. A handoff is evidence, not narrative.

## The Final Report

Name, in order:

1. What changed and why, in two sentences or fewer.
2. Docs updated, as paths.
3. Implementation and tests touched, as paths.
4. Commands run, with their actual one-line results. A quiet gate is quoted
   as its ok line or its failure tail, per
   [../operations/verification.md](../operations/verification.md).
5. Commands not run, each with the reason.
6. The next executable step: task file, the files it touches, its gate, and
   its acceptance line.

## Rules

- Never record under Tested anything that did not run in this tree; it
  belongs under Not-tested with its reason, per
  [honest-state.md](honest-state.md).
- Failure is a first-class handoff: what was attempted, the exact evidence
  of failure, the hypothesis ranking, and where the next agent should
  start. A documented dead end saves a session; a hidden one costs several.
- Numbers over adjectives: "check-lines fails on 3 files" beats "mostly
  passing".
- The report and the commit trailers
  ([../repository/commit-protocol.md](../repository/commit-protocol.md))
  must agree; divergence between them is itself a defect to fix before
  ending.

## Continuity

The next session starts from [../current-state.md](../current-state.md) and
the blocker queue, not from this report; anything the next agent must know
goes into those files or the task file, never only into the chat. The
report is for the human checking the work; the repository is for the next
agent.
