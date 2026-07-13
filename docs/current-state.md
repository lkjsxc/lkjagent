# Current State
## Purpose
Separate observed behavior in this checkout from the direct-runtime contract and
name the next executable work.
## Evidence Boundary

The public-loop work started from `ee4de81536b1baac845a346103f14420cd41f45b`.
Focused native-loop and scripted journal tests pass in this unfrozen checkout.
Changed-source local and rebuilt Compose test, lint, and verify pass; native
doctor and configured-model journal campaigns have not run. The original
comparison used source
`5604ec89af3ba9dbfb287bd869971781fdcf2fad` and parent product source
`28bdaacca4a6d7c779057893e3d48bfbd9f2ccea`.

Configured smoke runs corrected protocol, observation, dispatch, repetition, and
final-wording faults. Four tracked 903-second attempts made partial progress but
were not successes. Those runs remain historical diagnostics, not evidence for
the changed journal source.

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

The baseline causal defect was in the retired non-public path at the recorded
source: `owner_turn.rs` treated the substring `verify` as a system operation,
and `daemon_route_effects.rs` recorded an unsupported executor before any model
call. Those core owner-turn and engine files are now deleted, and public `send`
and `run` use only the native loop. A second 901-second run at source
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
- deterministic checks and workspace inventory;
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

The public CLI now contains only `help`, `send [--new] TEXT`, `run [--once]`,
`status`, and `doctor [--json]`. Its parser rejects retired console, workbench,
workspace, inspection, context, record, memory, watch, alias, and category
commands before filesystem or SQLite setup. Public stateful commands open only
the fresh native 18-table store. The focused public test demonstrates two closed
exact-edit matters, restart idempotency, and stale owner byte preservation with
a scripted endpoint.

The product crates no longer contain task, step, template, plan, classification,
owner-turn, retired check, docs-tree, plan-engine, daemon, bridge, record,
inspection, workbench, or old TUI authority. Direct parsing accepts only tool-call
and final envelopes selected by a persisted decision, and transport `Prompt`
lives in `prompt.rs`.

## Active Source Gaps

Record, memory, and artifact behavior must now be rebuilt only on native state.
Numbered file reads persist only the requested page and
continuation metadata, never unrequested whole-file bytes. The store has a
bounded read-only TUI frame projection:
canonical messages, whitelisted activity, and status counts share one deferred
read transaction and expose no runtime bodies beyond bounded conversation text.
The application has identity merge, Unicode composer and wrapping, separated
conversation and activity rendering, anchored pagination, a public native
terminal command, and unwind-safe terminal cleanup over that projection. Frame
reads and input stay on one UI thread while configured endpoint cycles run on a
bounded-wake background worker. Public scheduling runs one matter per cycle,
prefers an unrelated runnable matter over one with unfinished work, and rotates
the runnable tier by oldest durable selection across reopen. It still blocks
instead
of reconciling every effect recovery phase, and configured-model simultaneous-
matter fairness remains unproven. A 64-call model budget now creates a durable
visible blocked matter, and owner resume starts a fresh call-budget epoch while
preserving history. Missing provider usage now remains SQL null rather than fake
zero; known tokens and conservative unknown components now drive a durable
post-response token block. A 16-effect epoch limit blocks before exact-edit
preparation, 16 persisted rejected outputs exhaust recovery cost, and 900,000
settled/current-cycle active milliseconds block without charging daemon downtime.
The tracked schedule retained later active intake without claiming it closed.

The public compiler attaches its context plan and frame fingerprints, filters
canonical history by exact project token, and persists every selected owner,
history, memory, or measured source. Memory candidates are bounded current
managed `knowledge/notes/*.md` revisions with settled effects and closed source
matters, never arbitrary files. Exact `forget <slug>:` and `correct <slug>:`
forms suppress one matching key. List and search remain orienting evidence until
a read provides current source bytes. Final admission rejects future-tense and
unsupported command or test claims. Close and receipt remain check-derived.

The native `conversation_messages` table is canonical for public turns. Typed
TUI intake uses the exact returned durable message identity, and failed intake
keeps composer bytes; activity is bounded. A native PTY smoke covers entry,
Ctrl-C, and cleanup. Strict source-bound evaluators measure daily-life,
project, artifact, and slow Japanese PTY facts. The tracked runner drives the
real native TUI into a bounded cast; probe-only evidence fails and runs are unrun.

Configuration and Compose now separate the runtime data root from one visible
workspace root. The workspace is created only by an actual workspace operation,
startup no longer writes scaffold files, and diagnostics report both roots. The
flat registry now accepts seven keys with current consumers, including strict
`workspace_timezone` selection for journals.

One descriptor-derived `write_record` admits journal, memory, short report, and
long report map or child. Short reports keep `artifacts/reports/<slug>.md`; long
maps write `artifacts/documents/<slug>/README.md`; long children write
`artifacts/documents/<slug>/<unit>.md`; and pending resume stays on native
obligations plus one durable `report:pending` cell. All families share safe parents,
revisions, checks, receipts, collision protection, and placeholder rejection;
short files stay at 512 units, long files at 2,048, and memory retrieval stays
limited to current `knowledge/notes`; configured semantic proof is unrun.

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
| llm-wiring | complete | configured public campaign produced durable exchanges and evaluated exact-edit semantics |
| evaluation-runner | complete | confined commands validate the tracked blocked baseline without synthetic success |
| tool-registry | complete | one descriptor projection drives prompt, parser, admission, and effects |
| reducer-selector | complete | direct state reduction and deterministic selection pass core and bridge-continuity tests |
| store-transactions | complete | native intake through close boundaries pass restart tests |
| exact-edits | complete | crash-safe exact edit and create pass race, mode, symlink, and crash tests |
| automatic-checks | complete | public exact edits immediately reduce to three current passed checks in the focused test |
| app-public-loop | complete | public send, run, status, and doctor use only native state in focused tests |
| context-compiler | complete | selection precedes compilation and included owner or measured sources persist with the rendered frame |
| conversation-canonical | complete | native intake and checked close allocate stable ordered owner and final messages atomically |
| matter-budgets | complete | call, token, effect, recovery, and active-time epoch limits visibly block and resume from owner input |
| workspace-root | complete | public send is workspace-free and direct work opens the separate configured root lazily |
| native journal mechanics | complete | focused descriptor, timezone, parent safety, effect retry, managed revision, structural check, and public scripted tests pass |
| native memory mechanics | complete | focused slug, lineage, managed revision, retrieval, correction, contamination, and prompt-occurrence tests pass |
| native report mechanics | complete | focused short slug, long map and child topology, pending resume, managed replacement, structural checks, collision, bound, and receipt tests pass |
| configured-model journal proof | active | scripted mechanics cannot establish factual groundedness; the configured journal campaign has not run |
| configured-model memory proof | active | deterministic W06/C04 portions are derived; semantic recall and filler judgment remain unrun |
| configured-model report proof | active | deterministic W08 mechanics now cover short and linked long files, not configured semantic quality |
| public scripted file edit | complete | focused exact edit, checked read-only second matter, restart, and stale-revision tests pass |
| configured-model file proof | complete | tracked 903-second exact-file campaign passed with edit, second-turn, and retrieval continuity |
| retired-source-deletion | complete | old controller, bridge, record, inspection, workbench, and TUI source is absent from product crates |
| native-tui | complete | one-transaction frames, pure Unicode viewport, typed intake, bounded worker, rendering, PTY, and RAII tests pass |
| final campaigns | active | exact-file campaign passed; recovery, daily-life, multi-project, and PTY proof remain |

## Honesty Rules

- Target prose is not an implementation claim.
- Process health, elapsed time, and historical summaries are not current-source completion evidence.
- A command that did not run did not pass.
- Missing raw data creates a blocker, not a success.
- Completion belongs to fresh checks reduced from durable facts.
