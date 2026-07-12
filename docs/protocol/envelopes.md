# Model Envelopes

## Purpose

Define compact model output bound to one outstanding persisted decision.

## Binding

One provider exchange belongs to one `RuntimeDecision`. The model does not echo
decision IDs, context fingerprints, tool fingerprints, or JSON arguments. The
harness binds returned content to the outstanding exchange.

## Tool Call

```text
<tool_call>
<tool>edit_file</tool>
<input>
<path>notes/sample.md</path>
<old_text>alpha</old_text>
<new_text>beta</new_text>
</input>
</tool_call>
```

The action has one root, no prose outside it, no attributes, comments, CDATA, or
JSON argument object. Dynamic text uses XML entities. A workspace file may itself
contain JSON; it round-trips as escaped field or observation text.

## Final

```text
<final>
<message>Updated the requested file.</message>
</final>
```

Only a respond decision accepts final output. Its owner-visible form includes a
harness-generated path/check receipt. Unsupported or future-tense claims fail
admission. After bounded wording faults, the factual receipt is used alone.

## State Grammar

Orient and modify accept tool calls from their exact decision views. Review,
wait, and idle make no model call. Respond accepts final only. Recovery accepts
the intended grammar with a narrower view and one current valid example.

Direct requests omit provider stop strings so the endpoint returns the closing
root tag that strict parsing requires. The token budget remains the outer bound.

A tool-named root is an experiment cell, not a permanent second parser.
