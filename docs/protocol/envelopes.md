# Envelopes

## Purpose

Define the block envelopes accepted from the model for a persisted decision.

## Envelope Selection

Exactly one block is accepted per model turn. The `RuntimeDecision` fixes the
expected block, stop sequence, and tool view when an action is allowed.

| Block | Common operation | Content |
| --- | --- | --- |
| `<content>...</content>` | write or revise | full file body |
| `<plan>...</plan>` | plan helper | plan lines |
| `<action>...</action>` | tool operation | one decision-visible tool call |
| `<message>...</message>` | respond or ask | owner-facing prose |
| `<verdict>...</verdict>` | judged check | pass or fail plus reason |

Known helpers use these blocks, but future operation keys may select the same
blocks without becoming a new central step enum.

## Action Completion

Finish-like actions are not separate envelopes. They are actions only when the
current `ToolSetView` exposes them:

```text
<action>
<tool>finish</tool>
<summary>Enough evidence was gathered.</summary>
</action>
```

## Ask Semantics

An ask decision expects `<message>...</message>`. Accepting that message records
a question event, clean context item, and waiting state. A tool operation cannot
emit an ask block unless the decision selected that grammar.

## Rules

No prose may appear outside the envelope. Tags have no attributes. The body may
not be empty. Parameter names inside `<action>` must be unique and legal for the
selected tool view. Unknown tools and unknown parameters are faults relative to
the persisted decision. JSON and implicit envelopes are not part of the protocol.

## Examples

Valid write:

```text
<content>
The vault opened under the aurora.
</content>
```

Invalid write fault `wrong_block`:

```text
<message>The vault opened.</message>
```

## Failure This Prevents

The parser either accepts the expected decision block or returns a bounded fault;
it does not guess, repair, or normalize conflicting formats.
