# Registry

## Purpose

Define the canonical descriptor fields for the tool catalog.

## Catalog Rule

There is one tool catalog. Docs, prompt rendering, parser shape checks, action
admission, dispatcher wiring, and tests derive from the same descriptor set.
Fixed explore-only lists are helper views, not independent law.

## Descriptor Fields

Each descriptor stores:

- stable tool name;
- one-line purpose;
- input fields with type, required flag, and limits;
- observation contract and output bound;
- effect boundary;
- workspace path requirements;
- timeout or count budget;
- state affordance predicates;
- safety notes; and
- denial diagnostics for status, not prompt text.

## Prompt Form

The runtime renders exact action shapes only for tools in the active
`ToolSetView`:

```text
<action>
<tool>fs.read</tool>
<path>data/logs/current-model-run.md</path>
<count>20</count>
</action>
```

If no tools are available, the decision renders an output contract that does not
ask for an action.

## Failure This Prevents

A tool list cannot drift between documentation, prompt text, parser validation,
and effect dispatch.
