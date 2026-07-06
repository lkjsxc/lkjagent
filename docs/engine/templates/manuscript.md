# Manuscript Template

## Purpose

Define the manuscript template that is kept for existing checks now that generic
artifact manifests are the structured-artifact target.

## Objective Fields

The extractor reads title, root, chapter count, total word target, manuscript
glob, and fallback notes. Explicit roots under `stories/` or `manuscripts/`
win. Aurora Ledger resolves to `stories/aurora-ledger`, ten chapters, and
10,000 words. Simple digits and limited kanji numerals for ten, thousand, two
thousand, and ten thousand are recognized; unparsed word targets default to
10,000 words with a note in the plan state.

## Current Role

The template remains for existing Aurora Ledger tests and old manuscript input.
New structured work should prefer an artifact manifest with nested units, source
refs, and generic checks.

## Initial Plan

The snapshot starts with these steps:

- completed `plan` outline step seeded with one engine-owned write line per
  chapter;
- `write` settings step for `<root>/settings.md`;
- one `write` step for each chapter path;
- `verify` step carrying the task checks;
- `respond` step for measured paths and word counts.

The engine materializes chapter write steps from extracted objective fields
before any endpoint call. Each chapter path is
`<root>/manuscript/chapter-NN.md`. Chapter instructions ask for the target word
count plus a small ceiling so the endpoint stops near the needed length. The
path is in the step record before any model-authored content can be written.

## Assembly

Chapter content is written through the effects edge. When a later manuscript
section targets an existing chapter file, the engine appends the new content to
that file. Continuity comes from the existing file tail at the effects boundary,
not from trusting conversational history.

## Checks

The verify step runs:

- `file_count` for the manuscript glob with exact chapter count;
- `min_words_total` for the manuscript glob and objective target;
- `absent` checks for scaffold phrases and task-marker placeholders on every
  planned chapter file.

## Recovery

A `min_words_total` shortfall skips the failed verify step, inserts an extension
write step targeting the last matching chapter, then inserts a fresh verify step.
A manuscript write step that faults three times is marked blocked and followed by
a smaller continuation write for the same chapter path. Earlier failed check
results do not block closure after the fresh verify passes.

## Failure This Prevents

The manuscript topology matches the owner's requested paths before generation
starts, so successful prose cannot land in files that completion ignores.
