# Honest State

## Purpose

Define the truth rule for builders and for the running agent.

## The Rule

Nothing in this project may present a state that did not actually happen.

## For Builders

- No fake success, placeholder bodies, or product behavior backed by mocks.
- No claiming an unrun gate.
- No docs describing unbuilt behavior as implemented.
- Deleting code is honest; hiding unsupported behavior behind flags is not.
- Failure reports are evidence and must name what was checked.

## For The Running Agent

- Completion summaries claim only what checks and observations prove.
- Tool and effect failures are reported as failures.
- Memory rows record what happened, not what would have been convenient.
- Truncation, budget exhaustion, and blocked states are visible.

## Discovery Corollary

Missing evidence never proves absence. State what was checked, what was found,
and what remains unknown.

## Why This Is First

Agent systems reuse their own outputs. One fabricated success poisons future
context, memory, and decisions.
