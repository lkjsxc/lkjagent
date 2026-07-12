# Current State

## Purpose

Separate observed behavior in this checkout from the direct-runtime contract and
name the next executable work.

## Evidence Boundary

This reset is based on product source
`5604ec89af3ba9dbfb287bd869971781fdcf2fad`. Its parent product source is
`28bdaacca4a6d7c779057893e3d48bfbd9f2ccea`; the only later tree change removes
tracked files below `tmp/`. Product crates, active docs, Cargo inputs, and Docker
inputs are otherwise identical.

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

The direct reducer and selector now derive decisions from state cells, the five
native tools share one descriptor projection from prompt through admission, and
native store transactions cover intake, exchange, effect, observation, checks,
and close. Exact edit and create effects stage expected and intended bytes,
fsync, preserve mode, reject races, and recover every tested crash boundary.
Unknown executable payload schemas remain inert. These pieces pass the isolated
workspace test suite but do not yet form the production authority loop. A
configured public probe at source `cb27f80114cc263db515242def059c91bcc2abef`
produced one durable provider exchange; its sanitized hash-only evidence is
tracked as connectivity evidence with `semantic_status=not-evaluated`.

## Active Source Gaps

Production still hydrates `TaskSnapshot`, task rows, step rows, fixed templates,
and bridge cells. Context is prepared before final decision selection. The daemon
maps decisions back into the retired step engine and writes both authorities.

The direct five-tool descriptor catalog, parser, admission view, and effect keys
now agree, but the production prompt renderer still reaches them through the
retired task bridge. The direct edit transaction is safe in isolation; the public
owner path still invokes the existing whole-file writer and does not select the direct
edit from native state. The app cutover must connect persisted native decisions,
tool admissions, exact effects, automatic checks, and final close without a
second task or step authority.

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
| context-compiler | complete | selected decisions bind escaped deduplicated sources under lane and agent-file budgets |
| conversation-canonical | complete | native intake and checked close allocate stable ordered owner and final messages atomically |
| workspace-root | complete | data and workspace roots are separate, lazy, diagnosed, and Compose-mounted |
| public file edit | blocked | depends on context assembly and production app-loop cutover |
| final campaigns | blocked | no frozen binary, live task proof, or PTY proof |

## Honesty Rules

- Target prose is not an implementation claim.
- Process health and elapsed time are not task completion.
- A historical summary is not current-source acceptance evidence.
- A command that did not run did not pass.
- Missing raw data creates a blocker, not a success.
- Completion belongs to fresh checks reduced from durable facts.
