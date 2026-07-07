# Word Counting

## Purpose

Define the shared word counting rule for artifact and content checks.

## Rule

The implementation owner is `checks.word-count.algorithm=latin-cjk-sum`.

- Latin-script text counts whitespace-delimited tokens containing at least one
  letter or digit.
- Characters in the Han, Hiragana, and Katakana Unicode ranges each count as
  one word unit.
- Mixed text sums both counts.
- Markdown syntax is treated as text unless a check explicitly strips a region.

## Examples

`Daily report opened` counts as `checks.example.report-words=3`.

A string containing four characters from those ranges counts as
`checks.example.cjk-words=4`.

A mixed string with `Section`, `1`, and one Han character counts as
`checks.example.mixed-words=3`.

## Shared Use

The same function is used by templates, operation checks, matter checks, status,
benchmarks, replay, and proof bundles. A caller may not substitute a family-
specific counter.

## Failure This Prevents

Long-artifact completion cannot depend on matter-specific counting heuristics.
Every surface measures the same files the same way.
