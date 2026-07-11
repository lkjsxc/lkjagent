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

`checks.command.timeout-seconds=30` bounds command checks. Each command check has
a harness admission and prepared effect journal before execution, then one bounded
observation. Each shell tree carries a unique Linux process scope. Background and
session-detached descendants in that scope are frozen and killed through PID file
descriptors. Pipe capture is nonblocking, and a 500 ms userspace cleanup deadline
is checked during process discovery and pipe draining, so a descendant that
discards the scope cannot hold the check open. Command checks
are trusted catalog entries; containment is not claimed after deliberate scope
removal. Repeated command text consumes outcomes by declaration ordinal.
`checks.judged.max-tokens=300` bounds judged verify output.

## Placement

Operation checks run immediately after the effect. Matter checks run when no
runnable decisions remain. Benchmarks and replay use the same check evaluator.

## Diagnosis

A check records name, parameters, structured measured value, pass flag,
decision id, evidence fingerprint, artifact refs when applicable, and timestamp.
Completion matches rows back to their required check parameters and requires
fresh artifact refs for artifact-backed checks before accepting them as evidence.
Renderers may format that data as text, for example `min_words chapter-02.md:
312 < 500`, but the retry ladder consumes structured values rather than parsing
prose.

## Failure This Prevents

A matter cannot close because a model says it is complete; closure reads
measured check rows.
