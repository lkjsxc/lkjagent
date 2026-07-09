# Requirement Map

## Use

This table prevents a coherent partial rewrite from being mistaken for the
whole assignment. The owning node cannot pass until its contract and named gate
cover the requirement. Final acceptance reruns every node.

| Required outcome | Owning contract | Workgraph node | Independent proof |
|---|---|---|---|
| Docs precede related source and remain current | 08-implementation/docs-first-program.md | docs-authority, docs-reconciliation | docs and source gates |
| No file exceeds 200 lines; recursive README maps | 10-acceptance/repository-gates.md | repository-determinism | packet, docs, and source lint |
| No compatibility bridge or old task/step authority | 03-runtime/authority-model.md | selector-runtime-cutover | task/step-free fresh-store run |
| Several concurrent durable state dimensions | 03-runtime/state-vector.md | event-reducer | replay and transition properties |
| State changes prompt, context, tools, grammar, recovery, and exit | 03-runtime/state-program-matrix.md | context-prompt | table-driven prompt fingerprints |
| Harness JSON never becomes model protocol | 03-runtime/prompt-composition.md | context-prompt | prompt corpus rejection |
| Model emits attribute-free XML-like envelopes | 05-protocol-tools/envelope-grammar.md | protocol-tools | real endpoint action corpus |
| Exact tool fields, bounds, and admission | 05-protocol-tools/field-constraints.md | protocol-tools | parser/admission/effect lineage |
| Model sees only a small state tool view | 05-protocol-tools/tool-view.md | protocol-tools | hidden-reason matrix |
| Strict scalar settings at data/lkjagent.json | 02-product/configuration-registry.md | repository-determinism | focused configuration suite |
| Data and externally visible workspace are separate | 02-product/authority-boundaries.md | workspace-root-transactions | relative/absolute and Compose roots |
| Workspace stores life, knowledge, projects, artifacts, and activity | 06-workspace/filesystem-grammar.md | workspace-records | scenario workspace manifests |
| No eager empty hierarchy or bulk placeholder output | 06-workspace/filesystem-grammar.md | workspace-records | empty-start and creation traces |
| Managed files and navigation stay at most 512 tokens | 06-workspace/token-budget.md | workspace-records | independent conservative count |
| Owner can inspect files without asking the agent | 06-workspace/record-contract.md | workspace-records | direct workspace byte checks |
| Journal uses local YYYY/MM/DD and composed meaning | 06-workspace/diary.md | workspace-records | daily semantic gate |
| Command text stays in activity, not semantic journal | 06-workspace/record-contract.md | workspace-records | owner-message/body comparison |
| TODO, calendar, finance, note, decision, and session are typed | 06-workspace/daily-record-schemas.md | workspace-records | schema and transition scenarios |
| Writes are crash-safe and exactly once | 06-workspace/transaction-protocol.md | workspace-root-transactions | phase crash matrix |
| External edits, indexes, import, archive, and rebalance preserve bytes | 06-workspace/indexes-retrieval.md | workspace-retrieval-maintenance | rebuild and manifest equivalence |
| Context is relevant, bounded, deduplicated, and source-linked | 04-context/selection-pipeline.md | context-prompt | shuffled 10,000-item and live checks |
| Material contradictions block only dependent work | 04-context/contradictions.md | context-prompt | temporal conflict scenarios |
| Failure recovery changes strategy and retains partial work | 03-runtime/recovery.md | recovery-continuity | fault lineage and long artifact |
| More than four useful decisions can run without false idle | 03-runtime/continuity.md | recovery-continuity | anchored decision floors |
| Ready, plan, or generic message cannot close action work | 03-runtime/completion.md | selector-runtime-cutover | readiness false-close fixture |
| Daemon stays alive but does not count idle polling as progress | 03-runtime/loop-engineering.md | recovery-continuity | active/quiescent raw timing |
| Native tools are preferred; shell is bounded fallback | 05-protocol-tools/shell.md | protocol-tools | tool-view and shell-share evidence |
| Owner and agent messages render exactly once in causal order | 07-tui/transcript-model.md | canonical-conversation-tui | SQLite-to-PTY identity replay |
| Bottom follow, manual anchor, resize, and blank bounds are correct | 07-tui/scrolling.md | canonical-conversation-tui | 15-minute cast replay |
| Queue/state debug text is absent from ordinary conversation | 07-tui/diagnostics.md | canonical-conversation-tui | body and screen scan |
| Ideas are tested alone and in combinations before adoption | 09-evaluation/experiment-design.md | domain-experiments | matrix, metrics, raw repeats |
| Rejected and conditional interactions remain documented | 09-evaluation/adoption-ledger.md | integrated-adoption-cleanup | exact adoption coverage |
| Several real 15-minute daily-use runs are committed | 09-evaluation/anchored-scenarios.md | live-evidence | SQLite, events, workspace, binary |
| Final source, evidence, PTY, Docker, and public CI agree | 10-acceptance/final-gate.md | evidence-commit-ci | four-commit anchored controller |

## Closure Rule

If a row lacks its observed proof, the corresponding node receipt is invalid.
If a later patch changes its owning source or contract, invalidate that receipt
and every dependent node before continuing. No severity, elapsed time, or agent
confidence can waive a row.
