# Bootstrap Prompt For The Coding Agent

## Purpose

Point the next coding pass at the current temporary handoff package.

## Prompt

Work on `lkjsxc/lkjagent`. Improve documentation first, then improve the
implementation. Read `AGENTS.md`, `docs/current-state.md`, and
`tmp/lkjagent-state-runtime-redesign-report.md` before editing. If the structured
package is available as `tmp/lkjagent-state-runtime-redesign-report.zip`, extract
it under `tmp/` and read its `README.md`.

The main target is a durable state-ledger runtime with persisted runtime
decisions, decision-specific tool views, durable context items, contradiction and
contamination handling, loose endpoint timeouts, small checked artifact units,
and proof bundles. Keep authored source and docs under 200 lines, prefer small
files, avoid fake success, and record only gates that actually ran.
