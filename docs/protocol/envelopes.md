# Envelopes

## Purpose

Define decision-bound, attribute-free model output grammars.

## Selection

One persisted decision accepts exactly one envelope family, stop sequence, and
optional tool view. Content operations use `content`, tool operations use
`lkjagent_action`, owner reports or questions use `message`, and bounded judges
use `verdict`. The selected family follows semantic operation data rather than a
second dispatcher enum.

## Action Shape

```text
<lkjagent_action>
<decision_id>matter-42-decision-7</decision_id>
<context_fingerprint>fnv1a64:84a9c6f20c3130e1</context_fingerprint>
<tool_name>workspace_read</tool_name>
<input>
<path>projects/lkjagent/README.md</path>
<max_tokens>384</max_tokens>
</input>
</lkjagent_action>
```

Tags have no attributes, comments, CDATA, processing instructions, or JSON.
The renderer uses stable field order. Admission accepts only the decision ID,
context fingerprint, visible tool, required typed fields, field constraints,
and total size recorded by the current decision.

## Content And Messages

Content bodies preserve multiline UTF-8 and escaped XML entities. A content
operation cannot return a readiness message. A report message becomes eligible
only after required checks pass. A question message records a waiting event and
bounded question rather than settling the matter.

## Parser

The pure parser accepts one complete expected envelope or returns a typed fault
with a bounded diagnosis. It never executes partial output, repairs values,
normalizes conflicting shapes, or treats prose outside the envelope as success.
Action parsing has no generic Explore fallback: it requires the persisted
runtime decision that owns the decision ID, context fingerprint, and tool view.

Placeholder values such as `...`, `PATH`, `TODO`, `VALUE`, `<path>`, or `[path]`
are non-executable and fail admission before any effect.
