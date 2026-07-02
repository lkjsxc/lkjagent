# Docs Tree Template

## Purpose

Define how lkjagent writes structured Markdown documentation trees.

## Objective Fields

The extractor reads root, topic, page count, and whether the count is exact.
Roots named under `docs/` or `documentation/` win; otherwise the root defaults
to `docs/guide`. Exact count phrasing creates an exact file-count check for the
root README plus the requested pages. Approximate phrasing, including `about`,
`roughly`, `approx`, `around`, and Japanese approximate wording, records the
count for budget only.

## Initial Plan

The snapshot starts with these steps:

- `plan` step named `docs tree plan`;
- `verify` step carrying `readme_coverage`, `links_resolve`, and exact
  `file_count` when requested;
- `respond` step.

The plan step accepts plan-line grammar write lines. Directories are implied by
paths. Materialization rejects any path outside the root, any directory without
a README write step, and any exact-count mismatch. The diagnosis is recorded as
a notice and the plan step remains active for another attempt.

## README Rule

Every directory under the requested root receives a README page. The README is
an ordinary write step produced by the plan and checked by the engine. No hidden
scaffold creates navigation files.

## Link Repair

A failed `links_resolve` verify inserts a revise step targeted at a Markdown file
that contains links, then inserts a fresh verify step. Earlier failed check
results do not block closure after the fresh verify passes.

## Failure This Prevents

The tree closes only when link and README checks pass, preventing document sets
that look complete in prose but are not navigable.
