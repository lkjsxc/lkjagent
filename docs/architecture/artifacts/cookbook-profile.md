# Cookbook Profile

## Purpose

Give an example semantic tree for a cookbook artifact. This profile is an
example, not the only valid cookbook output.

## Example

```text
cookbooks/<semantic-title>/
  README.md
  manifest.md
  foundations/
    README.md
    flour-water-salt-yeast.md
    gluten-development.md
    fermentation.md
    shaping.md
    baking.md
  recipes/
    README.md
    country-loaf.md
    sourdough-basic.md
    ciabatta.md
    focaccia.md
    baguette.md
    brioche.md
    challah.md
    rye-bread.md
  troubleshooting.md
  equipment.md
  audit.md
```

## Contract

A cookbook audit requires recipe content, foundations or technique content,
troubleshooting or equipment coverage, README links, and a manifest.

Structural audit and content readiness are separate. A scaffold with many files
can be well shaped and still fail readiness when recipes lack ingredients,
procedure steps, timing, yield, and bread-specific coverage.

## Invariants

- Recipes must include ingredients and procedure steps.
- Technique leaves must include signals and corrective action.
- Empty leaves, heading-only files, and generic status prose fail readiness.
- Completion cannot rely on file counts or manifests alone.

## Failure Cases

- A 100-file cookbook scaffold is marked ready while recipe leaves are empty.
- A bread cookbook has generic project documentation instead of recipes.
- Audit claims recipe coverage without observing ingredients or methods.

## Verification

Cookbook readiness tests include scaffold-only output as bad and meaningful
bread recipe content as good.

## Status

partially implemented through cookbook scaffold and content audit checks.
