# Handoff

## Purpose

Define the final report format for a coding-agent session.

## Final Report

Name, in order:

- what changed and why;
- docs updated, as paths;
- implementation and tests touched, as paths;
- commands run, with actual results;
- commands not run, with reasons;
- the next executable step.

## Rules

`Tested` in the report must match commands that actually ran. Quiet gates are
quoted by their `ok ...` line or failing tail. A failure handoff names the exact
evidence, ranked hypothesis, and first file or command the next agent should
inspect.

## Continuity

Anything the next agent must know goes into repository files, usually
[../current-state.md](../current-state.md), evaluation fixtures, or task docs.
The chat transcript is not the durable handoff.
