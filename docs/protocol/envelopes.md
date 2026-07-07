# Envelopes

## Purpose

Define the block envelopes accepted from the model for a persisted decision.

## Envelope Selection

Exactly one block is accepted per model turn. The `RuntimeDecision` fixes the
expected block, stop sequence, and tool view when a tool call is allowed.

| Block | Common operation | Content |
| --- | --- | --- |
| `content` | write or revise | full file or record body |
| `plan` | bridge planner | plan lines |
| `lkjagent_action` | tool operation | one XML-like tool call |
| `message` | respond or ask | owner-facing prose |
| `verdict` | judged check | pass or fail plus reason |

Known helpers use these blocks, but future operation keys may select the same
blocks without becoming a central enum.

## Action Shape

Tool calls use an attribute-less XML-like grammar. Tags have no attributes, no
JSON bodies, and no implicit fields.

```text
<lkjagent_action>
<decision_id>decision:case:001</decision_id>
<context_fingerprint>ctx:9ac4</context_fingerprint>
<tool_name>workspace.record</tool_name>
<argument>
<name>kind</name>
<value>journal</value>
</argument>
<argument>
<name>body</name>
<value>Today I used many AI tools before my Codex allowance reset.</value>
</argument>
</lkjagent_action>
```

`argument` elements may repeat. `decision_id`, `context_fingerprint`, and
`tool_name` are scalar and must appear once. The parser decodes XML entities,
preserves multiline values, rejects duplicate scalar tags, and returns typed
internal structs.

## Tool Call Cards

A tool-call prompt card shows the decision id, context-frame fingerprint,
decision-visible tools, required argument names, and a copyable XML-like
skeleton. Skeleton placeholders are labelled non-executable; unchanged
placeholder values must parse to a known admission rejection before effects run.

## Ask Semantics

An ask decision expects a `message` block. Accepting that message records a
question event, clean context item, and waiting state. A tool operation cannot
emit an ask block unless the decision selected that grammar.

## Rules

No prose may appear outside the selected envelope. Attributes, comments,
processing instructions, CDATA, crossed tags, unclosed tags, unknown required
tags, missing required tags, duplicate scalar tags, stale decision ids, tools
absent from the persisted decision view, and JSON-looking action bodies are
faults. Placeholder values such as `...`, `PATH`, `TODO`, `VALUE`,
`FIELD_VALUE`, `<path>`, or `[path]` are not executable and are rejected before
effects.

## Parser Contract

The parser is deterministic and pure. It either accepts the exact envelope for
the current decision or returns a typed fault with a bounded diagnosis and the
smallest repair hint. It never executes a partially parsed action and never
normalizes conflicting formats.

## Examples

Valid write:

```text
<content>
Daily report body.
</content>
```

Invalid write fault `wrong_block`:

```text
<message>The report is ready.</message>
```

Invalid action fault `attribute`:

```text
<lkjagent_action decision_id="d1"></lkjagent_action>
```

## Repair Cards

A retry prompt names the decision id, expected envelope, bounded fault,
invalid-excerpt hash, allowed tool, and required change. It never quotes the
full failed body into normal context.

## Failure This Prevents

The parser either accepts the expected decision block or returns a bounded fault;
it does not guess, repair, or normalize conflicting formats.
