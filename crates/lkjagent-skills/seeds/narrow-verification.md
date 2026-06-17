# Skill: Narrow Verification

## Purpose

Pick the smallest command that proves the changed behavior before spending
time on broader gates.

## Trigger

A change needs verification and the full gate is not the first useful signal.

## Context

- Task files often contain a Focused Gate block.
- README files and operations docs name broader project gates.
- Recent failure output is the current source of truth.

## Procedure

1. Run `rg -n "Focused Gate" docs/execution/tasks` when a task file is not already open.
2. Run the focused command that directly exercises the changed crate or behavior.
3. Read the output and fix the first real failure before adding broader commands.
4. Run `cargo run -p lkjagent-xtask -- quiet verify` after focused checks pass.
5. Record every command that ran and every command intentionally not run.

## Checks

- The focused command exits 0 and its output names the tests or gate that ran.
- `cargo run -p lkjagent-xtask -- quiet verify` prints `ok verify` before broad completion.
- Any skipped command has a concrete reason such as a missing later blocker.

## Must Not

- Do not claim a gate passed without running it.
- Do not run broad gates first when a focused failure is cheaper and more informative.
- Do not ignore newer failure output in favor of an older hypothesis.
