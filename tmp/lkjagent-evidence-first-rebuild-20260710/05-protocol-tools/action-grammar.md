# Action Grammar

## Canonical Shape

Use one attribute-free envelope. This concrete example is non-executable because
the decision fingerprint belongs only to the documented example:

    <lkjagent_action>
    <decision_id>matter-42-decision-7</decision_id>
    <context_fingerprint>fnv1a64:84a9c6f20c3130e1</context_fingerprint>
    <tool_name>workspace_read</tool_name>
    <input>
    <path>projects/lkjagent/README.md</path>
    <max_tokens>384</max_tokens>
    </input>
    </lkjagent_action>

Compare this direct-field grammar against the current repeated name/value
argument grammar in live trials. Keep the grammar with higher first-pass
admission and lower accidental finish rate.

## Other Envelopes

Content-authoring states use a state-specific content envelope. Report states
use a message envelope. Ask states use a question envelope. A decision accepts
exactly one expected envelope family.

## Rules

- no attributes, CDATA, processing instructions, comments, or JSON;
- one outer envelope and one executable action;
- balanced known tags; the renderer uses a stable conventional order, while
  admission accepts semantically equivalent field order unless experiments
  prove stricter ordering improves complete task success;
- multiline UTF-8 values and escaped XML entities;
- bounded field and total sizes;
- no prose outside the envelope.
