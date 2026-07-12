# Current State

## Purpose

Separate observed behavior in this checkout from the direct-runtime contract and
name the next executable work.

## Evidence Boundary

The public-loop work started from `ee4de81536b1baac845a346103f14420cd41f45b`.
The focused native-loop test passes in the current checkout, whose source is not
yet frozen. A tracked configured-model file campaign has passed as described
below; Docker has not run after the latest loop changes.
The original planning comparison used source
`5604ec89af3ba9dbfb287bd869971781fdcf2fad` and parent product source
`28bdaacca4a6d7c779057893e3d48bfbd9f2ccea`.

Configured-model smoke runs exposed and corrected stripped closing tags,
recovery cells bound to the wrong key, omitted orient observations, unlabeled
listing paths, pending dispatch errors, repeated read-only calls, and unbounded
final-wording faults. Sources `5b643de8b10ce0ee9bd2cbe63b53c1dbed8f3798`
and `1e6de0d40acec2563ff2578cb41862481a9892cf` first proved an exact edit and
checked read-only continuation.

Four 903-second tracked attempts then progressed honestly. Source
`7f47763540e7042b3cf111997e082c72832d5c6a` exited on malformed-output recovery.
Source `607465201258253de06a649c5f0d96334cc2d690` reached two checked closes.
Source `ee738f6a46ab7e40481524d0cbe0a4f6b7e73977` reached three but repeated
inventory listing. Source `f5535af5a8b2237dfbaabfb9678ccb93954707fa`
also reached three and showed that requiring later sustained-intake matters to
close exceeded the declared edit, second-turn, and retrieval checks. None was
recorded as success.

A fifth tracked 903-second run at source
`689b48ec577e35315f632d5ab008f8eb9218ab18` passed its semantic evaluator with
exact bytes, one file, 18 tables, 24 provider exchanges, ten admissions, one
effect, 12 current passed checks, five owner messages, and four checked closes.
The source-bound sanitized facts are tracked below
`evaluation/evidence/689b48ec577e35315f632d5ab008f8eb9218ab18/`.

A synthetic 901-second run against the parent product source seeded
`notes/sample.md` with `alpha is the current value.` and asked for an exact edit,
verification, and truthful report. The daemon stayed alive and owner intake
returned success, but the file remained unchanged.

The fresh store had one blocked task, three blocked steps, and zero runtime
decisions, provider exchanges, admissions, effects, observations, checks,
artifacts, or workspace records. Startup also created nine README files and one
transcript unrelated to the requested edit.

The baseline causal defect remains in the retired non-public path:
`owner_turn.rs` treats the substring `verify` as a system operation, and
`daemon_route_effects.rs` records an unsupported executor before any model call. Public
`send` and `run` no longer call that path. A second 901-second run at source
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

Public `send`, `run`, `run --once`, `status`, and `doctor` now branch before
retired schema setup and open only the fresh native 18-table store. The focused public test
demonstrates two closed exact-edit matters, restart idempotency, and stale owner
byte preservation with a scripted endpoint.

## Active Source Gaps

Retired non-public daemon, record, workspace, and TUI APIs still use
retired schema and projection modules for regression coverage. They are not called by
public send, run, or status. Public scheduling currently handles one open matter
at a time, blocks unfinished effects instead of completing every recovery phase,
and does not yet prove fairness across simultaneous open matters. Scripted and
configured-model runs cover checked read-only continuation, but the full tracked
schedule has not yet run.

The public compiler attaches its context plan and frame fingerprints; native
context-item rows are not yet populated for every included source. List and
search observations remain orienting evidence until a read produces current
source bytes. Final model wording is not yet checked for unsupported claims,
although close and the owner receipt remain check-derived. These are bounded
follow-up gaps, not task/step fallback.

The native `conversation_messages` table is canonical for public turns, but the
TUI still synthesizes owner and agent messages from retired queue and event rows,
then merges local drafts. It has two viewport implementations and does not
measure wrapped display rows consistently.

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
| acceptance-checker | complete | rejects nine fixtures and derives source, static, and exact-campaign predicates |
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
| public scripted file edit | complete | focused exact edit, checked read-only second matter, restart, and stale-revision tests pass |
| configured-model file proof | complete | tracked 903-second exact-file campaign passed with edit, second-turn, and retrieval continuity |
| final campaigns | active | exact-file campaign passed; recovery, daily-life, multi-project, and PTY proof remain |

## Honesty Rules

- Target prose is not an implementation claim.
- Process health and elapsed time are not task completion.
- A historical summary is not current-source acceptance evidence.
- A command that did not run did not pass.
- Missing raw data creates a blocker, not a success.
- Completion belongs to fresh checks reduced from durable facts.
