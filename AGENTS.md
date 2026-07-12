# AGENTS.md

## Purpose

Direct automated coding agents working on lkjagent.

## Product

lkjagent is a durable state-ledger daemon for one owner, one local LLM, and one
visible workspace. Owner turns become matters, events, state cells, persisted
runtime decisions, admitted effects, observations, checks, and truthful messages.

## Non-Negotiable Rules

1. `docs/` is the implementation contract. Update docs and implementation in
   the same change, including `docs/current-state.md` when behavior moves.
2. Every authored file stays at or below 200 lines.
3. Docs use ASCII prose, kebab-case filenames, one H1, then a Purpose section.
4. Every docs directory has one README table of contents and at least two children.
5. No release shorthand, release labels, or migration framing.
6. Use a pure functional core and effectful edges. Product crates have no panic paths.
7. Record honest state only. Do not ship fake success, placeholders, or mocks as
   product behavior, and do not claim an unrun gate.
8. Durable state rows and persisted `RuntimeDecision` rows are the single control
   plane. Do not add another graph authority, MCP, runtime sub-agents, prompt-only
   policy, or a dispatcher-only tool registry.
9. Completion is reducer-computed through fresh checks. The model never decides
   that a matter is done.
10. Commit small coherent slices with `Tested` and `Not-tested` trailers matching
    commands actually run.
11. Use Docker Compose for final verification when behavior is claimed complete.

## Read Order

1. `docs/current-state.md`
2. `docs/vision/README.md`
3. `docs/state/README.md`
4. `docs/runtime/README.md`
5. `docs/product/README.md`
6. `docs/workspace/README.md`
7. `docs/context/README.md`
8. `docs/protocol/README.md`
9. `docs/tools/README.md`
10. `docs/store/README.md`
11. `docs/evaluation/README.md`
12. `docs/operations/verification.md`
13. `docs/agent/README.md`

## Work Selection

When the owner names a task, do that task. Otherwise read `evaluation/workgraph.tsv`
and execute every dependency-satisfied required row. A row is complete only when
its predicate is recomputed from source and evidence.

Before editing, state the objective, constraints, assumptions, risks, evidence
requirements, candidate files, and next action. Use isolated worktrees for
independent coding-agent work. The parent agent owns architecture, shared types,
merges, and verification.

## Verification

A gate that did not run did not pass. Prefer the small substantive suites named in
`docs/operations/verification.md`. Real endpoint and PTY evidence carry semantic
acceptance. Final success requires the tracked acceptance command and independent
review described in `docs/evaluation/live-proof.md`.

## Handoff

Name changes and reasons, docs updated, exact commands and results, unrun commands
and reasons, evidence paths, blockers, and the next executable step.
