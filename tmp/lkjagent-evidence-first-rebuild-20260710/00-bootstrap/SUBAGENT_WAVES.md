# Subagent Waves

## Dispatch

Use the exact role prompts in 11-subagents. Reserve one concurrency slot for the
primary and dispatch at most the remaining slots. The primary owns conflict
resolution, docs truth, integration, tests, commits, and final behavior.

The default is read-only analysis or patch handback; children do not commit in
the shared checkout. A writing child instead receives an isolated worktree,
base commit, owned paths, imported interfaces, gate ID, and evidence location.

## Initial Read-Only Wave

- docs cartographer: authority and stale-contract map;
- runtime engineer: event reducer, state vector, selector, recovery;
- store-workspace engineer: root, transactions, diary, retrieval, maintenance;
- evaluation operator: failure fixtures and clean-checkout proof.

The primary synthesizes and commits docs authority before any behavior writer
starts.

## Middle Parallel Wave

- context engineer: candidate pool, budgets, conflicts, compaction;
- protocol engineer: grammar, constrained decoding, admission, tools;
- TUI engineer: canonical transcript, ordering, scroll, PTY;
- store-workspace engineer: operation journal, projections, crash recovery.

## Evidence Wave

First the docs cartographer performs reconciliation and the primary freezes
source. Then the evaluation operator runs live scenarios while the TUI engineer
runs the PTY scenario and the runtime engineer runs deterministic regressions.
After all raw evidence is committed and public CI succeeds, the independent
verifier runs last. No docs or source edits occur during evidence capture.

## Merge Rules

- No subagent report is completion evidence by itself.
- Shared domain contracts merge before dependent source patches.
- Do not allow two agents to create competing control planes.
- Rebase or re-run focused tests after every overlapping merge.
- A verifier must not verify code it authored.

Every handback names base commit, node, owned and inspected files, patch or
findings, exact commands and exit codes, raw evidence paths, unresolved faults,
interface changes, and the next executable action.
