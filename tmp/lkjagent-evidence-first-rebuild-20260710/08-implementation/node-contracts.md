# Node Contracts

## Receipt Rule

Each node owns one nonempty Docker integration suite named by its workgraph ID.
Its receipt cites at least the raw command log and a machine-readable result,
plus a separate verifier note. Passing means the whole predicate below is
observed; compilation, an empty test binary, or a status label never suffices.

## Foundation Nodes

| Node | Primary owner | Complete predicate |
|---|---|---|
| baseline-capture | evaluation operator | Attached failures are reproduced or honestly bounded, SQLite and source facts are saved, and no source behavior changed. |
| docs-authority | docs cartographer | Docs state current failures, target authority, exact schemas, removal map, and no false live claim; source remains behavior-identical. |
| repository-determinism | evaluation operator | Cargo.lock and every Docker input are tracked; isolated no-cache Compose build, lint, test, and verify pass. |
| evaluation-harness | evaluation operator | Fake clock, fault injector, scenarios, online backup, manifest, PTY recorder, and false-positive fixtures fail the old runner. |
| store-foundation | store-workspace engineer | Fresh native schema, constraints, immutable content, lineage, sequences, and two-transaction effect journal pass crash/property tests. |

## Runtime And Workspace Nodes

| Node | Primary owner | Complete predicate |
|---|---|---|
| event-reducer | runtime engineer | Pure replay is deterministic; invalid concurrent cells and transitions reject; projected state matches durable events. |
| workspace-root-transactions | store-workspace engineer | Relative/absolute roots, no-follow containment, atomic file/DB effects, idempotency, compensation, and external edits pass. |
| workspace-records | store-workspace engineer | Journal, TODO, calendar, finance, note, project, and activity schemas write meaningful bounded visible files and indexes. |
| selector-runtime-cutover | runtime engineer | Production intake and selection use only native matters/events; decisions persist before prompt/effect; old task/step authority is absent. |
| context-prompt | context engineer | Decision-routed discovery, source validation, conflicts, deduplication, budget optimization, prompt order, and fingerprints pass shuffled and scenario tests. |
| protocol-tools | protocol engineer | One canonical tool registry, state views, envelopes, parser, constrained decoding, repair, and exact admission meet endpoint thresholds. |
| recovery-continuity | runtime engineer | Fault ladders change causal conditions, long work exceeds four useful decisions, waits remain wakeable, crashes resume, and ready prose cannot close. |
| workspace-retrieval-maintenance | store-workspace engineer | Old body recall, project isolation, stable scan, indexes, debt, import, tombstones, archive, and rebalance preserve bytes and links. |
| canonical-conversation-tui | TUI engineer | One transactional transcript feeds both backends with exact identity/order, visual-row scroll, follow/manual anchors, quiet view, and responsive PTY behavior. |

## Decision And Evidence Nodes

| Node | Primary owner | Complete predicate |
|---|---|---|
| domain-experiments | context and protocol engineers | Baseline, isolated, pairwise, and integrated real candidates run with predeclared metrics and at least three independent repeats. |
| integrated-adoption-cleanup | primary | Only measured defaults remain; conditional/rejected interactions stay documented; old authority and unused alternatives are deleted. |
| docs-reconciliation | docs cartographer | Every current claim maps to source/tests, all retired prose is removed, README maps are complete, and line/topology gates pass. |
| source-freeze | primary plus evaluation operator | Focused suites, locked clean checkout, no-cache Compose, all deterministic scenarios, and docs pass on one clean commit. |
| live-evidence | evaluation operator | All three anchored endpoint scenarios and experiment repeats meet raw 15-minute, causal, semantic, workspace, and recovery gates on frozen source. |
| pty-evidence | TUI engineer | Real 15-minute cast and SQLite replay prove input, ordering, scroll bounds, follow/manual behavior, resize, restart, quiet view, and latency. |
| evidence-commit-ci | primary | Raw evidence and final node receipts are tracked, packet and source are unchanged, and the public workflow succeeds before material freeze. |

## Shared Interfaces

The primary assigns one owner to schema migrations, decision identity, effect
journal, conversation sequence, workspace path service, prompt card types, and
tool registry before parallel writers start. Other agents import those
interfaces and return patches from isolated worktrees. Any interface change
invalidates dependent node receipts and reruns their gates.
