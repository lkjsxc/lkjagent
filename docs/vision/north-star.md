# North Star

## Purpose

Define what lkjagent is for and how the owner recognizes useful behavior.

## Product

lkjagent is one continuously running local agent for one owner, one configured
LLM endpoint, one visible workspace, and one SQLite state ledger.

The workspace is an auxiliary storage device that the owner can inspect without
asking the agent. It can hold daily records, notes, project work, reports,
session receipts, and sourced memory.

## Capability Order

1. Inspect, create, and exactly edit UTF-8 workspace files.
2. Verify actual bytes and report truthful receipts.
3. Recover from protocol, stale-file, endpoint, effect, and check faults.
4. Maintain one canonical conversation and usable TUI.
5. Write grounded daily records at human date paths.
6. Recall sourced owner-readable memory across matters.
7. Work across multiple projects under the same root.
8. Produce bounded checked reports and session projections.
9. Add richer retrieval or organization only after measured need.

## Weak Model Assumption

The model may have a modest context window, unreliable native tool calling, and
a tendency to copy prompt examples. The harness therefore selects state, tools,
context, recovery, effects, and completion. The model authors bounded content or
one admitted XML-like action.

## First Proof

A configured real model must inspect one existing file, apply one exact
revision-bound edit through the public daemon, pass native checks, emit a factual
message, and complete a second owner turn. Tables, plans, and process health are
not substitutes for those bytes and rows.
