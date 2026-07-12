# Current State

## Purpose

Separate observed behavior in this checkout from the direct-runtime contract and
name the next executable work.

## Evidence Boundary

This cutover is based exactly on `ee4de81536b1baac845a346103f14420cd41f45b`.
The focused public native-loop test passes in this checkout. No Docker or real
configured-model endpoint campaign has run for this change.

A synthetic 901-second run against the parent product source seeded
`notes/sample.md` with `alpha is the current value.` and asked for an exact edit,
verification, and truthful report. The daemon stayed alive and owner intake
returned success, but the file remained unchanged.

The fresh store had one blocked task, three blocked steps, and zero runtime
decisions, provider exchanges, admissions, effects, observations, checks,
artifacts, or workspace records. Startup also created nine README files and one
transcript unrelated to the requested edit.

The causal defect is current source: `owner_turn.rs` treats the substring
`verify` as a system operation, and `daemon_route_effects.rs` blocks that lane as
an unsupported executor before any model call. A second 901-second run at source
`97e00698f348fc2435d47a107b5b8453c98b9d1f` reproduced the same zero-decision
failure. Its sanitized bundle is tracked below
`evaluation/evidence/97e00698f348fc2435d47a107b5b8453c98b9d1f/`. This summary is
evidence of failure, not endpoint success.

## Implemented Primitives

The source contains useful low-level pieces:

- arbitrary state cells, events, edges, and runtime decisions;
- strict attribute-free action parsing and bounded tool field specs;
- provider intent and response logging;
- tool admissions, effect journals, target revisions, and observations;
- descriptor-relative no-follow file traversal and staged replacement pieces;
- deterministic checks, workspace inventory, and TUI composer reducers;
- flat JSON configuration validation and Docker build separation from data.

The repository now also has a source-bound acceptance incomplete mode with nine
negative fixtures and closed direct-runtime contract tables. A separate native
18-table schema rejects incompatible stores, descriptor-relative reads hold an
opened root, the compact envelope parser rejects echoed IDs and JSON actions, and
LLM transport preserves bounded request/outcome fields without repairing output.

The direct reducer and selector derive decisions from native cells, the five
native tools share one descriptor projection from prompt through admission and
dispatch, and native transactions cover intake, exchange, effect, observation,
checks, respond settlement, canonical message, and close. Exact edit and create
effects stage expected and intended bytes, fsync, preserve mode, reject stale
revisions, and retain the isolated crash-boundary coverage. Unknown executable
payload schemas remain inert.

Public `send`, `run`, `run --once`, and `status` now branch before legacy schema
setup and open only the fresh native 18-table store. The focused public test
demonstrates two closed exact-edit matters, restart idempotency, and stale owner
byte preservation with a scripted endpoint.

## Active Source Gaps

Legacy non-public daemon, inspection, record, workspace, and TUI APIs still use
retired schema and bridge modules for regression coverage. They are not called by
public send, run, or status. Public scheduling currently handles one open matter
at a time, blocks unfinished effects instead of completing every recovery phase,
and does not yet prove fairness across simultaneous open matters.

The public compiler attaches its context plan and frame fingerprints; native
context-item rows are not yet populated for every included source. List and
search observations remain orienting evidence until a read produces current
source bytes. These are bounded follow-up gaps, not task/step fallback.

There is no `conversation_messages` table. The TUI synthesizes owner and agent
messages from queue and selected event rows, then merges local drafts. It has two
viewport implementations and does not measure wrapped display rows consistently.

Configuration and Compose now separate the runtime data root from one visible
workspace root. The workspace is created only by an actual workspace operation,
startup no longer writes scaffold files, and diagnostics report both roots.
Record routing can still write canned diary text without a model, and several
accepted configuration keys still have no production consumer.

## Direct Contract

The target core is:

```text
RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState
RuntimeState + Policy + CurrentTime -> RuntimeDecision
```

The selected decision persists exact state, tool, grammar, context-need,
recovery, check, and exit specs. Context attaches to that immutable selection
before provider intent. The effect edge is admitted and journaled. Fresh native
checks follow committed edits automatically. Final wording cannot add unchecked
claims, and the close transaction owns the canonical final message.

## Work State

| Node | State | Evidence |
| --- | --- | --- |
| docs-reset | complete | compact contracts, plans, and authority gate pass |
| acceptance-checker | complete | rejects nine fixtures and derives seventeen source or static predicates |
| baseline-tracked | complete | tracked 901-second run has zero non-missing checker errors |
| interfaces | complete | closed vocabularies and direct five-tool descriptors pass core tests |
| effects-safe-read | complete | opened-root listing, search, and revision reads pass safety tests |
| protocol-compact | complete | strict decision-bound tool/final parser passes contract tests |
| store-native | complete | exact 18-table schema reopens and rejects altered or retired stores |
| llm-wiring | complete | configured public probe produced one provider exchange; semantics were not evaluated |
| evaluation-runner | complete | confined commands validate the tracked blocked baseline without synthetic success |
| tool-registry | complete | one descriptor projection drives prompt, parser, admission, and effects |
| reducer-selector | complete | direct state reduction and deterministic selection pass core and bridge-continuity tests |
| store-transactions | complete | native intake through close boundaries pass restart tests |
| exact-edits | complete | crash-safe exact edit and create pass race, mode, symlink, and crash tests |
| automatic-checks | complete | public exact edits immediately reduce to three current passed checks in the focused test |
| app-public-loop | complete | public send/run/status use only the native store and direct selector in focused scripted tests |
| context-compiler | complete | selection precedes compilation and current source revision and bytes enter the next prompt |
| conversation-canonical | complete | native intake and checked close allocate stable ordered owner and final messages atomically |
| workspace-root | complete | public send is workspace-free and direct work opens the separate configured root lazily |
| public file edit | complete | focused exact edit, second matter, restart, and stale-revision tests pass |
| configured-model file proof | active | no real endpoint or Docker proof ran for this cutover |
| final campaigns | blocked | no frozen binary, live file proof, or PTY proof |

## Honesty Rules

- Target prose is not an implementation claim.
- Process health and elapsed time are not task completion.
- A historical summary is not current-source acceptance evidence.
- A command that did not run did not pass.
- Missing raw data creates a blocker, not a success.
- Completion belongs to fresh checks reduced from durable facts.
