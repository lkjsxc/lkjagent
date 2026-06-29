# Root Identity

## Purpose

Define the first write contract for a missing or identity-incomplete artifact
root.

## Contract

Root identity is created by a model-authored `fs.batch_write` action that is
validated against a stored runtime contract. No scaffold writer creates the
root. No one-file README seed satisfies identity.

For a story root, the initial exact paths are:

```text
{root}/catalog.toml
{root}/README.md
{root}/objective.md
{root}/setting-overview.md
{root}/cast.md
```

The paths are flat. They avoid single-child subdirectories and can satisfy the
document topology check immediately when the README is a valid navigation page.

## Story Content Rules

Each Markdown leaf except `README.md` must contain:

- exactly one H1;
- at least one `##` section;
- at least 25 words;
- story signals such as `reference detail`, `continuity note`,
  `verification note`, or `story bible`.

The catalog names the artifact kind, such as `kind = "story"`. The README
names the purpose and links the catalog and leaves.

## Other Profiles

Cookbook and generic artifacts use the same stored contract shape with
profile-specific required sections. The resolver still prefers flat identity
paths before any nested batches unless the profile contract names a stronger
reason to create subdirectories.
