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
These pieces do not yet form the direct authority loop.

## Active Source Gaps

Production still hydrates `TaskSnapshot`, task rows, step rows, fixed templates,
and bridge cells. Context is prepared before final decision selection. The daemon
maps decisions back into the retired step engine and writes both authorities.

The direct five-tool descriptor catalog is separate, but the production prompt
renderer still emits the retired action grammar until the tool-registry node.
The current model view hides write, list, tree, shell, and any valid review
transition. Whole-file model writes use a direct writer that bypasses exact target
revisions. Replacement staging changes file mode and validates captured bytes
after exchange, so it is not yet safe for the target edit path.

There is no `conversation_messages` table. The TUI synthesizes owner and agent
messages from queue and selected event rows, then merges local drafts. It has two
viewport implementations and does not measure wrapped display rows consistently.

The runtime data root and visible workspace are not separate in Compose. Startup
creates a broad tree. Record routing can write canned diary text without a model.
Several accepted configuration keys still have no production consumer.

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
| acceptance-checker | complete | source-bound incomplete mode rejects nine negative fixtures |
| baseline-tracked | complete | tracked 901-second run has zero non-missing checker errors |
| interfaces | complete | closed vocabularies and direct five-tool descriptors pass core tests |
| effects-safe-read | complete | opened-root listing, search, and revision reads pass safety tests |
| protocol-compact | complete | strict decision-bound tool/final parser passes contract tests |
| store-native | complete | exact 18-table schema reopens and rejects altered or retired stores |
| llm-wiring | active | mechanics pass; real public endpoint probe waits for evaluation runner |
| second parallel wave | ready | exact edits, registry, transactions, reducer, and root can start |
| public file edit | blocked | depends on exact edits, transactions, context, and app loop |
| final campaigns | blocked | no frozen binary, live task proof, or PTY proof |

## Honesty Rules

- Target prose is not an implementation claim.
- Process health and elapsed time are not task completion.
- A historical summary is not current-source acceptance evidence.
- A command that did not run did not pass.
- Missing raw data creates a blocker, not a success.
- Completion belongs to fresh checks reduced from durable facts.
