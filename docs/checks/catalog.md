# Catalog

## Purpose

Define the deterministic check vocabulary.

## Check Set

The check count is `checks.catalog.count=11`.

| Check | Parameters | Pass rule |
| --- | --- | --- |
| `file_exists` | `path` | file exists and is non-empty |
| `min_words` | `path`, `n` | counted words in file are at least `n` |
| `min_words_total` | `glob`, `n` | summed counted words are at least `n` |
| `max_lines` | `path`, `n` | line count is at most `n` |
| `file_count` | `glob`, `min`, `max?` | match count is inside range |
| `contains` | `path`, `needle` | literal string occurs |
| `absent` | `path`, `needle` | literal string does not occur |
| `readme_coverage` | `root` | each directory has README links to children |
| `links_resolve` | `root` | relative Markdown links resolve |
| `command` | `cmd` | command exits success before timeout |
| `judged` | `criterion`, `path` | verify verdict is pass |

`checks.command.timeout-seconds=30` bounds command checks.
`checks.judged.max-tokens=300` bounds judged verify output.

## Placement

Step checks run immediately after the step effect. Task checks run when no
runnable steps remain. Benchmarks and replay use the same check evaluator.

## Diagnosis

A failed check records name, parameters, structured measured value, pass flag,
and timestamp. Renderers may format that data as text, for example
`min_words chapter-02.md: 312 < 500`, but the retry ladder consumes structured
values rather than parsing prose.

## Failure This Prevents

A task cannot close because a model says it is complete; closure reads measured
check rows.
