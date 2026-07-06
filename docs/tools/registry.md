# Registry

## Purpose

Define the canonical descriptor fields for the tool catalog.

## Catalog Rule

There is one tool catalog. Docs, prompt rendering, parser shape checks,
tool-call admission, dispatcher wiring, and tests derive from the same
descriptor set. Fixed explore-only lists are helper views, not independent law.

## Descriptor Fields

Each descriptor stores:

- stable tool name;
- one-line purpose;
- input fields as `ToolFieldSpec` values with name, required flag, value class,
  and limits;
- safe example parameters when the prompt may show a copyable filled call;
- observation contract and output bound;
- effect boundary;
- workspace path requirements;
- timeout or count budget;
- state affordance predicates;
- safety notes; and
- denial diagnostics for status, not prompt text.

## Field Value Classes

The catalog assigns each field a value class before rendering a `ToolSetView`.
Current classes are text, workspace path, shell command, count, and query.
Admission uses the same field spec to reject placeholder values, path escapes,
and non-numeric count values before effects.

## Prompt Form

The runtime renders safe filled examples only when the active `ToolSetView`
carries example parameters. Schema-only shapes are labelled non-copyable and
remain rejected by admission when placeholders are unchanged:

```text
<tool_call>
<tool_name>fs.read</tool_name>
<path>README.md</path>
<count>20</count>
</tool_call>
```

If no tools are available, the decision renders an output contract that does not
ask for a tool call.

## Failure This Prevents

A tool list cannot drift between documentation, prompt text, parser validation,
and effect dispatch.
