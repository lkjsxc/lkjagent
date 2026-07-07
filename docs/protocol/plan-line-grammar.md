# Plan Line Grammar

## Purpose

Define the exact grammar for model-authored plan steps.

## Grammar

```text
plan        = line *(LF line)
line        = write-line / explore-line / respond-line
write-line  = "write" SP path SP "|" SP title SP "|" SP "words=" number
explore-line = "explore" SP "|" SP goal SP "|" SP "budget=" number
respond-line = "respond" SP "|" SP summary
path        = 1*path-char
path-char   = ALPHA / DIGIT / "/" / "-" / "_" / "."
title       = 1*text-char
goal        = 1*text-char
summary     = 1*text-char
number      = 1*DIGIT
text-char   = any non-LF character except "|"
```

The grammar id key is `protocol.plan-line.grammar-id=plain-pipe`. The maximum
materialized steps are capped by `engine.plan.max-steps=80`.

## Valid Examples

```text
write reports/status.md | Weekly Status | words=500
explore | Find the file that names the target root | budget=5
respond | Report created paths and measured checks
```

## Invalid Examples

```text
write /tmp/report.md | Outside root | words=500
```

Fault: `bad_plan_line`, because absolute paths are rejected after grammar parse.

```text
write reports/x.md | Missing words
```

Fault: `bad_plan_line`, because the `words=` field is absent.

```text
explore | Search | budget=many
```

Fault: `bad_plan_line`, because `budget` is not digits.

## Failure This Prevents

Plan output is data the engine validates before mutation. Invalid lines produce
specific retries instead of partially materialized plans.
