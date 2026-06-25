# Action Format

## Purpose

The grammar of a live model turn. The model emits exactly one singular action
envelope whose fields are attribute-free tags.

## Canonical Turn

```text
<action>
<tool>graph.plan</tool>
<objective>Create a structured science-fiction story bible for Chronos Fracture.</objective>
<constraints>
Root directory: stories/chronos-fracture.
Root must contain README.md and catalog.toml.
Every directory must contain README.md and at least two children.
Every Markdown file must stay under 160 lines.
Do not write the full manuscript.
Do not write scaffold-only pages.
</constraints>
<steps>
1. Record the story-bible plan.
2. Create the root catalog and README.
3. Write bounded batches for setting, characters, plot, continuity, and checks.
4. Audit document structure.
5. Audit artifact readiness.
</steps>
<paths>
stories/chronos-fracture
</paths>
<reason>The owner requested a structured story bible with evidence-gated completion.</reason>
</action>
```

## Grammar

- A turn contains exactly one `<action>...</action>` envelope.
- `<action>` is singular. `<actions>` is invalid.
- `<act>` is not a live runtime action envelope.
- The first child is `<tool>known.tool</tool>`.
- Each later child is one registry parameter name.
- Parameter names are unique within the action.
- Values live between opening and closing parameter tags.
- Tag names contain names only; tag attributes are invalid.
- No XML features exist: no attributes, namespaces, comments, CDATA,
  entities, or nested action envelopes.
- No prose appears before or after the action envelope.
- No `<think>` tag, hidden reasoning tag, or free-form reasoning block is valid
  live output.
- The model stops immediately after `</action>`.

## Multi-Line Values

```text
<action>
<tool>fs.write</tool>
<path>notes/protocol-observation.md</path>
<content>
# Protocol Observation

The live action envelope is singular and closed by the provider stop sequence.
</content>
</action>
```

The value of `content` is every byte between `<content>` and `</content>`,
including blank lines, code fences, quotes, shell commands, and angle-bracket
text that is not a structural line for the current parameter. An opening
parameter tag may start the first content bytes on the same line, such as
`<content># Premise`; the value then continues until the matching closing tag.
For multiline values, the closing tag may appear alone on its own line or at the
end of the final value line.

## Batch File Values

```text
<action>
<tool>fs.batch_write</tool>
<files>
path: stories/chronos-fracture/catalog.toml
content:
[artifact]
root = "stories/chronos-fracture"
kind = "story"

-- lkjagent-next-file --
path: stories/chronos-fracture/README.md
content:
# Chronos Fracture

## Purpose

Navigate the story bible for Chronos Fracture.
</files>
</action>
```

`fs.batch_write` is one action with one `files` payload. The dispatcher
validates the whole payload before mutation. The full batch contract, accepted
payload formats, limits, and refusal cases live in [batch-write.md](batch-write.md).

## Control Actions

Task control uses the same envelope:

```text
<action>
<tool>agent.done</tool>
<summary>
The requested story bible is written, audited, and tied to artifact-readiness evidence.
</summary>
</action>
```

## Invalid Shapes

These shapes are faults, not partial actions:

- empty assistant content after provider anomaly classification.
- plain prose without an action body.
- `<think>...</think>` before, inside, or after an action envelope.
- missing `<action>` or missing `</action>` after provider-stop closure rules.
- more than one action envelope.
- `<actions>`, `<act>`, or any other top-level envelope.
- top-level JSON action output.
- tag attributes or attribute-like tags such as `<path=stories/chronos-fracture</path>`.
- repeated parameter tags for unique parameters.
- parameters absent from the active tool schema.
- nested action envelopes.

## Prompt History Hygiene

Provider request history must not replay invalid assistant turns as assistant
examples. Before rendering a new request, the runtime either drops or summarizes
assistant messages that contain prose before an action, `<think>` tags, hidden
reasoning, multiple action envelopes, top-level JSON actions, empty content, or
any parse fault shape. The summary is a user-visible notice or runtime fact, not
an assistant exemplar. Live prompt text must not grant permission to use
`<think>` tags.

## Implicit Envelope Normalization

A missing opening envelope is accepted only by the strict implicit-envelope path
in [parsing.md](parsing.md). The body must contain exactly one complete,
schema-valid, authority-admitted action body and no prose outside recognized
fields. The parse and provider exchange logs record the normalization.

## Design Properties

- The prompt teaches one side effect per turn.
- The parser can map every invalid shape to a structured recovery route.
- Model text supplies intent or file content; runtime data supplies authority.
- Examples are concrete task actions, not placeholder parse skeletons.

## Status

partially implemented. The protocol and LLM crates now use `<action>` and
`</action>` for live parsing, rendering, stop sequences, closure repair, and
oversize detection. Runtime prompt authority, recovery routes, and daemon
admission still need full decision-ledger wiring.
