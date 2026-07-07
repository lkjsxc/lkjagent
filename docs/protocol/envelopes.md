# Envelopes

## Purpose

Define the block envelopes accepted from the model for a persisted decision.

## Envelope Selection

Exactly one block is accepted per model turn. The `RuntimeDecision` fixes the
expected block, stop sequence, and tool view when a tool call is allowed.

| Block | Common operation | Content |
| --- | --- | --- |
| `<content>...</content>` | write or revise | full file body |
| `<plan>...</plan>` | plan helper | plan lines |
| `<tool_call>...</tool_call>` | tool operation | one decision-visible tool call |
| `<message>...</message>` | respond or ask | owner-facing prose |
| `<verdict>...</verdict>` | judged check | pass or fail plus reason |

Known helpers use these blocks, but future operation keys may select the same
blocks without becoming a new central step enum.

## Tool Call Completion

Finish-like operations are not separate envelopes. They are tool calls only when
the current `ToolSetView` exposes them:

```text
<tool_call>
<tool_name>finish</tool_name>
<summary>Enough evidence was gathered.</summary>
</tool_call>
```

## Tool Call Cards

A tool-call prompt card may show a safe filled example or a schema-only shape. A
safe filled example is copyable only when the same `RuntimeDecision` can parse
and admit it. A schema-only shape is labelled non-copyable; unchanged placeholder
values such as `FIELD_VALUE` must parse to a known admission rejection before any
effect runs.

## Ask Semantics

An ask decision expects `<message>...</message>`. Accepting that message records
a question event, clean context item, and waiting state. A tool operation cannot
emit an ask block unless the decision selected that grammar.

## Rules

No prose may appear outside the envelope. Tags have no attributes. The body may
not be empty. A `<tool_call>` starts with one `<tool_name>` whose value appears
in the selected tool view. Other field names inside `<tool_call>` must be unique
and legal for that tool. Required fields follow in deterministic order.
Placeholder values such as `...`, `PATH`, `TODO`, `VALUE`, `FIELD_VALUE`,
`<path>`, or `[path]` are not executable and are rejected before effects.
Unknown tools and unknown fields are faults relative to the persisted decision.
Implicit envelopes are not part of the active daemon prompt protocol.

## JSON Action Parser

A staged pure parser accepts one JSON action object inside its dedicated action
delimiter pair. The object carries the schema discriminator, the persisted
decision id, the tool name, the argument object, and the context frame
fingerprint. The parser rejects duplicate keys at every object level,
unknown top-level fields, stale decision ids, unknown tools, missing required
arguments, unknown arguments, and wrong primitive types before any effect can
run. Active daemon prompts still use `<tool_call>` until the bridge is wired.

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

## Repair Cards

A retry prompt names the decision id, expected block, bounded fault, allowed tool,
and required shape. It never quotes the full failed body into normal context.

## Failure This Prevents

The parser either accepts the expected decision block or returns a bounded fault;
it does not guess, repair, or normalize conflicting formats.
