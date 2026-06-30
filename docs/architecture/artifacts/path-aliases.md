# Path Aliases

## Purpose

Specify short semantic roots for generated artifacts. Paths are addresses, not
owner sentences.

## Rules

- Root segments are short, usually under 24 characters.
- Use semantic role names such as `novel`, `cookbook`, `dictionary`, `guide`,
  or one short qualifier.
- Do not repeat the full owner sentence in path segments.
- Store full owner wording, title, assumptions, and semantic artifact id in
  artifact metadata under the root.
- If a short root belongs to a different semantic artifact, add one short
  qualifier such as `stories/novel-moon` or `stories/novel-city`.
- Do not use release labels, numbered product-line names, or ordinal-only
  roots.

## Examples

| Owner objective | Root |
| --- | --- |
| `Create a SF novel. with detailed structured settings.` | `stories/novel` |
| `Write a big bread cookbook.` | `cookbooks/bread-cookbook` |
| `Create a detailed bread dictionary.` | `dictionaries/bread-dictionary` |
| `Write a moon colony novel.` | `stories/novel-moon` |

## Artifact Card

The root stores the full request in a small card:

```text
<artifact-card>
<root>stories/novel</root>
<label>Create a SF novel. with detailed structured settings.</label>
<kind>story</kind>
<scale>long</scale>
<semantic-id>story:novel</semantic-id>
</artifact-card>
```

This card is content metadata. It is not a model action and is not used as an
alternate tool-call syntax.

## Planner Contract

The root planner is a pure function of the objective and artifact kind. Runtime
collision handling may add a qualifier after checking the workspace, but it may
not fall back to the full owner sentence.

## Verification

Tests assert that the observed SF-novel objective maps to `stories/novel`, path
segments stay short, collisions use short qualifiers, and the full owner
objective remains present in artifact metadata.

## Status

implemented for current artifact root planning and short semantic aliases. Open
profile-generalization work belongs to
[../../execution/tasks/content-atom-graph.md](../../execution/tasks/content-atom-graph.md).
