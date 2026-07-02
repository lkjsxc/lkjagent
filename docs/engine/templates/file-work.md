# File Work Template

## Purpose

Define create, revise, organize, and summarize tasks over workspace files.

## Selection

The classifier selects this template when the objective names concrete files,
directories, or edit verbs. If target paths are uncertain, the template starts
with an explore step capped by `template.file-work.explore-budget=8`.

## Plan

- Known write targets become write steps with exact output paths.
- Known revisions become revise steps with the current file as input.
- Summaries become respond steps, optionally preceded by bounded reads.
- A verify step attaches objective-specific checks from
  [../completion.md](../completion.md) when the task names measurable criteria.

## Edits

Revisions are whole-file for model-authored content. The engine may provide an
excerpt when the file exceeds `context.revise.input-tokens=4000`, but the model
still returns a full replacement for the planned target or a blocked diagnosis.

## Failure This Prevents

The model does not guess patch hunks or write paths during scripted edits, so a
format mistake cannot corrupt unrelated files.
