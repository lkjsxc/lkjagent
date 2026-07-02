# Manuscript Template

## Purpose

Define how lkjagent writes long prose manuscripts.

## Objective Fields

The classifier extracts title, root, manuscript glob, requested chapter paths,
objective word target, and chapter count. For Aurora Ledger the fields include
`objective.root=stories/aurora-ledger`, `objective.chapter-count=10`, and
`objective.total-words=10000`.

## Initial Plan

- A plan step asks for chapter titles, beats, and word targets.
- A write step creates `settings.md` with premise, cast, and facts.
- Chapter write steps are materialized from the validated plan. Large chapters
  split into section writes sized by `template.manuscript.section-words-min=400`
  and `template.manuscript.section-words-max=700`.
- A verify step runs task checks from [../completion.md](../completion.md).
- A respond step reports measured paths and word counts.

## Assembly

The engine owns chapter paths and appends section content to the planned file.
Inputs for each section include the beat, named facts, and the continuity tail
bounded by `context.write.continuity-tail-words=150`.

## Checks

The task checks include `file_count` over requested chapter files and
`min_words_total` over the manuscript glob. Objective-specific absence checks
remove scaffold phrases when requested.

## Failure This Prevents

The manuscript topology matches the owner's requested paths before generation
starts, so successful prose cannot land in files that completion ignores.
