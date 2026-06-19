# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
design-only, and what comes next. Every change that moves behavior updates
this file in the same commit. Statuses used across docs: implemented,
design-only, not implemented, out of scope, open question.

## Summary

The repository contains the documentation contract, a compiling Cargo
workspace, local verification gates, the action parser, the pure state graph,
the pure context engine, the SQLite store boundary, the LLM endpoint client,
the tool dispatcher/adapters, the runtime step/daemon core, and the CLI for
send, status, log, console, memory, startup checks, repository-root .env
loading, JSON runtime config, a /data/workspace working tree, and a resident
daemon that delivers queued owner work to the endpoint and tool dispatcher
until graph-gated agent.done. The console is transcript-first with a bottom
status/control deck and display-width wrapping for mixed English and Japanese
terminal text.

Owner messages create or resume a graph case before endpoint execution. The
graph classifies the task family, enters planning, records constraints,
risks, evidence requirements, selected context packages, and legal next
transitions, then renders a graph state notice into the first endpoint-visible
context. Runtime state persists graph cases, graph events, and graph evidence
so restart and compaction reconstruct structured state. Tool observations are
recorded as graph evidence. Completion is refused until the active graph gate
has its required evidence; a graph completion refusal names the missing kind
and points the next action to `graph.evidence`.

Recursive docs, counted structured content, and knowledge-base tasks are
graph task families that select document construction. They create
README-indexed scaffolds before endpoint work and cannot close without graph
evidence for the required document structure. Counted artifact wording,
English body, draft, and manuscript wording, and Japanese seikabutsu and
honpen phrasing select document construction when paired with file-count
wording, and counted creation deliverables preempt incidental architecture or
test wording before endpoint work starts. Implementation, fix, debug, patch,
and refactor action wording plus Japanese shusei, jisso, debug, and refactor
wording veto that preemption so code-change tasks keep their specialized
graph family. Japanese bug and shusei wording also select bug-fix routing.
File and markdown-count requests remain deterministic control guards,
including when combined with a recursive or knowledge-base task family;
English file, document, and docs wording plus common Japanese file wording
are recognized, ASCII digits,
full-width digits, common Japanese numeric kanji, common English number
words including hyphenated compounds and scale words such as hundred and hundreds,
and comma-like digit separators are accepted, the target number is chosen by
proximity to file or document wording, and exact or approximate mode wording
is scored near that chosen target. Direct exact wording is exact, approximate
wording including ish, or-so, hodo, and zengo forms uses a bounded tolerance,
and negated exact wording such as not-exactly, no-need-exact, and
does-not-have-to-be-exact keeps the approximate mode instead of forcing an
exact count. Exact wording attached to a smaller docs, outline, or design
subcount does not make an approximate overall count strict. Aggregate wording
such as total, combined, in-all, and gokei can carry the overall count when
the prompt also contains smaller docs or outline file counts, while nearby
non-file units such as words or pages keep their numbers from becoming
file-count targets. Counted completion prefers
README-indexed roots and falls back to clean top-level output directories.
The active graph prefix renders count guards and tells the model to use one
compact `shell.run` command with direct `/bin/sh` loops and `printf`
templates for bulk creation and count verification, keep the act payload
under about 1200 characters, and avoid hardcoded `/workspace` paths, brace
expansion, cat heredocs, bash scripts, literal bodies, or one `fs.write` per
file.
For counted documentation tasks that are not recursive, knowledge-base, or
benchmark scaffolds, the daemon also writes a generic `structured-output/`
tree with the requested count before the first endpoint turn and records graph
evidence for the scaffold, then closes only when the graph completion gate
admits that evidence; otherwise it waits with the missing gate reason.
That scaffold profiles the owner's objective by detected language and broad
deliverable kind, so the root, docs, and main directories are README-indexed
within the selected count guard, and the root README records the count
breakdown, machine-readable audit manifest, acceptance audit, and whether the
guard is exact or approximate.
Kind profiling recognizes explicit guide and report synonyms before
narrative terms, ignores negated story-specific constraints, and covers
manuscript, screenplay, playbook, runbook, course, training, whitepaper,
dossier, and Japanese task terms.
The graph evidence for that scaffold records `structured-output`, the target
file count, index file count, design memo count, main file count, root index,
file-budget status, machine-readable audit-manifest status including
restart-guide, design-owner-link, local-verification, reading-path, and sequence-path
requirements, restart-guide status, directory index, count-linked
acceptance-audit status, coverage-map status, first and last main, `index_scope=all`,
`section_scope=all`, content-block status, required design-section status,
required main-section status, part-ledger status, explicit design-owner-link
status, explicit local-verification status, explicit root reading-path status,
explicit sequence-path status, deterministic scaffold closure reason, and
`verification=ok`, so completion can be audited after the case closes. The
same normalized evidence is saved into task-summary memory for startup and
compaction recovery.
The root index includes a restart guide that names README.md, docs/README.md,
main/README.md, design-owner links, local verification, sequence ledgers, and
the stable file-count rule when content exists. It also includes a reading
path that names the first main file, last main file, and numeric read order.
The docs index maps design memos to covered main ranges. The main index maps
stage ranges from the same per-part stage assignment plus a role ledger that
links each main file back to its design memo owner for direct part-file restart.
Every design index entry, progress-map line, main ledger entry, main-file
design-owner link, and explicit main-file previous/current/next path is verified
before closure. Every main file also carries a local verification checklist
covering design-owner status, sequence paths, draft content blocks, and continuity handoff. Every
design and main file is section-verified and content-block
verified before scaffold closure. Design and main files carry matching
headings, section roles, objective anchors that
preserve model-number decimal tokens, main-range coverage, kind-aware segment
briefs, sequence ledgers, anchor-linked body spines, per-part content
details, specific-detail variation blocks, draft passage blocks, and
continuity handoffs while preserving the exact count. Root indexes keep
operational clauses and raw counted-create wording for traceability, while
main-file body anchors skip model-thrift, budget-thrift, English count mechanics,
composition parentheticals, allocation-only clauses, count-only structural
clauses, and generic meta constraints.
Docs-side scaffolds carry twenty distinct planning focus labels before
falling back, so common 20-outline requests do not collapse into repeated
supplemental planning notes.
When the objective includes a design, memo, viewpoint, outline, planning,
appendix-note, lore/story-planning sheets, Japanese design-file, or setting-reference
count, the scaffold uses it for design memos when the exact total can still retain main content.
File-count wording stays stronger than design wording when scoring allocation
hints, so total-count numbers are not reused as design memo counts.
Runtime recovery uses bounded notices for parse errors, repeated actions,
tool errors, endpoint max-token exits, budget exhaustion, and context
pressure. Three consecutive parse faults, repeated actions, or tool errors
move the task to waiting with a concrete owner handoff instead of spending the
remaining turn budget on the same recovery loop. Graph recovery nodes are
present as the structured target for the next runtime expansion. Endpoint
outages record failed attempts, set a capped
retry deadline, and keep later polls from hitting the endpoint or appending
more error events until that deadline. The LLM client sends `</act>` as a stop
sequence and restores the stripped close tag before parsing so one endpoint
completion stays bounded to one action envelope. Length completions with a
closed act are accepted; true oversize completions record a bounded preview in
recovery.

The runtime context window defaults to 24,576 tokens, accepts 16,384 tokens as
the lower supported value, derives safe soft/hard compaction triggers from the
configured window, and uses the 2,048-token reserve as endpoint max_tokens.
Owner task turn budgets load from `task.turn-budget` and apply to new tasks,
explicit continuations after budget exhaustion, and restart summaries.
Compaction is graph-aware: it preserves the active case, phase, node, plan,
evidence, missing evidence, touched paths, selected packages, recovery
strategy, and completion guard before rebuilding the prefix. Memory retrieval
uses graph memory links to prefer active-case and active-node rows within the
same memory kind, and task-summary saves link the memory row to the active or
just-closed graph case and node.

Memory remains durable retrieval, but graph cases link evidence and memories.
Empty queues open bounded graph maintenance cycles in rotation when directives
are due: distill, refine-skills, prune-memory, and audit-self. Saving a user
task summary stamps all maintenance directives, so the daemon shows idle after
task completion until the cooldown passes or owner work arrives. Compose
wiring is implemented.
The implementation queue is [execution/current-blockers.md](execution/current-blockers.md).

## Area Status

| Area | Status | Contract |
| --- | --- | --- |
| Documentation tree and policies | implemented | [repository/](repository/README.md) |
| Vision and scope | implemented | [vision/](vision/README.md) |
| Decision records | implemented | [decisions/](decisions/README.md) |
| Agent manual | implemented | [agent/](agent/README.md) |
| Execution queue and tasks | implemented | [execution/](execution/README.md) |
| Cargo workspace and crates | implemented | [repository/layout.md](repository/layout.md) |
| Verification xtask and quiet gates | implemented | [operations/verification.md](operations/verification.md) |
| Docker compose services | implemented | [operations/compose.md](operations/compose.md) |
| Container image skeleton | implemented | [architecture/sandbox/container.md](architecture/sandbox/container.md) |
| Daemon and agent loop | implemented | [architecture/runtime/](architecture/runtime/README.md) |
| State graph and task cases | implemented | [architecture/state-graph/](architecture/state-graph/README.md) |
| Context engine and graph-aware compaction | implemented | [architecture/context/](architecture/context/README.md) |
| Action protocol and parser | implemented | [architecture/protocol/](architecture/protocol/README.md) |
| Toolset | implemented | [architecture/tools/](architecture/tools/README.md) |
| SQLite store, transcript, and memory access | implemented | [architecture/memory/](architecture/memory/README.md) |
| LLM endpoint client | implemented | [architecture/llm/](architecture/llm/README.md) |
| Container and sandbox | implemented | [architecture/sandbox/](architecture/sandbox/README.md) |
| User message queue and CLI | implemented | [product/](product/README.md) |
| Automatic idle self-maintenance | implemented | [architecture/runtime/self-maintenance.md](architecture/runtime/self-maintenance.md) |
| Mechanical benchmark evaluation | implemented | [evaluation/](evaluation/README.md) |

## Out of Scope

Messaging channels, web UI, MCP, sub-agents, plan mode, heartbeat schedules,
and cron schedules. The boundaries are stated in [vision/scope.md](vision/scope.md).

## Next Step

No open blocker remains in
[execution/current-blockers.md](execution/current-blockers.md). Future changes
use the compose final gate per [operations/verification.md](operations/verification.md).

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
