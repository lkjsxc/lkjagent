# Envelope Grammar

## Lexical Rules

UTF-8 text uses XML entity escaping for `&`, `<`, and `>`. Tags contain lowercase
ASCII letters and underscores. Attributes, comments, CDATA, processing
instructions, namespaces, self-closing tags, JSON, and prose outside the root
are invalid. Unknown, missing, duplicate scalar, or out-of-decision fields are
rejected. Repeatable fields are declared by the active tool schema.

## Abstract Grammar

    envelope = action | content | question | message
    action = open_action decision context tool input close_action
    input = open_input field+ close_input
    content = open_content decision context kind title body source+ close_content
    question = open_question decision context reason prompt close_question
    message = open_message decision context summary evidence* close_message
    field = open_known_tag escaped_text close_same_tag

The concrete root tags are `lkjagent_action`, `lkjagent_content`,
`lkjagent_question`, and `lkjagent_message`. `decision_id` and
`context_fingerprint` are the first two scalar fields in rendered examples;
the parser accepts other semantic field order unless the grammar trial proves a
strict order improves full-task success.

## Cardinality

An action names exactly one admitted tool and one input block. Content is legal
only for a decision whose grammar declares its document kind. A question names
one blocking reason and one answerable prompt. A message carries no operation;
it can report only evidence already present in the decision.

## Limits

Each decision persists maximum scalar bytes, repeated field count, total bytes,
and output tokens. Parsing is streaming with bounded nesting depth. Decoded
values are validated for type, range, path containment, source freshness, tool
visibility, and decision identity before an admission row is committed.

## Observation Grammar

The harness, never the model, renders `lkjagent_observation` with operation ID,
outcome, bounded facts, evidence refs, and omitted-byte count. Failed output is
stored as recovery-only evidence and cannot be mislabeled `ok`.

## Repair

One parser repair may show the expected root, field names, current decision ID,
and bounded error position. It never copies untrusted trailing prose. The repair
attempt receives a new exchange ID but the same decision and cannot execute
until the complete envelope passes admission.
