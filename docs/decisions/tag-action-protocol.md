# Tag-Based Action Protocol

## Purpose

Fix the format in which the model expresses one runtime action.

## Decision

The live model protocol is a small tag-based action language. A valid model
turn contains exactly one singular action envelope:

```text
<action>
<tool>graph.state</tool>
</action>
```

The first child is `<tool>`. Every other child is an attribute-free parameter
tag from the dispatcher registry. Values live between opening and closing tags.
The model emits no prose outside the action envelope and stops immediately
after `</action>`.

The protocol is not XML. It has no attributes, namespaces, comments, CDATA,
entity decoding, or nested action envelopes. `<actions>` is invalid because a
turn can carry one side effect only. `<act>` is not a live runtime action
envelope.

Top-level JSON is not model-facing action output. JSON text may appear only as
a documented payload inside a parameter such as `<files>` when that tool
contract accepts it.

## Consequences

- The parser is strict, line-oriented, and tiny; its rules live in
  [../architecture/protocol/parsing.md](../architecture/protocol/parsing.md).
- The stop sequence is `</action>` and provider-stop closure repair is recorded
  per [../architecture/protocol/stop-token-policy.md](../architecture/protocol/stop-token-policy.md).
- Registry examples are executable action examples, not schematic parse shapes.
- Attribute-like tags such as `<path=stories/chronos-fracture</path>` are
  parser faults with contextual repair, not unknown parameters.
- Every recovery route is keyed by a parser, schema, admission, tool, endpoint,
  context, verification, completion, compaction, or maintenance fault.

## Rejected Directions

- `<actions>`: plural wording suggests independent side effects, while the
  product invariant is exactly one action per model turn.
- `<act>`: the terse name obscures recovery wording and is not accepted by live
  dispatch.
- XML: full XML adds attributes and decoding rules the runtime never needs.
- JSON tool calls: small local models malform braces, strings, and commas under
  pressure; the runtime needs a bounded tag grammar.
- Markdown sections as protocol: headers and fences appear naturally inside file
  content, making parses ambiguous.
- Line-only key-value records as the canonical prompt shape: they are compact
  but weak for multi-line payloads and contextual tag repairs.
