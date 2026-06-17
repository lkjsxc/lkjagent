# Skill: Owner Report

## Purpose

Give the owner a compact, honest status update or handoff during long work.

## Trigger

A long task needs a concise progress answer, blocker report, or final handoff.

## Context

- `git status --short` separates committed work from pending edits.
- `git log -1 --pretty=oneline` names the latest checkpoint.
- Gate output is stronger than adjectives.

## Procedure

1. Run `git status --short` to check whether pending edits remain.
2. Run `git log -1 --pretty=oneline` when a commit was made.
3. Name the target result, changed files or commits, and validation evidence.
4. Name any commands not run and the concrete reason.
5. Keep the report short enough that the next action is obvious.

## Checks

- The report names the latest commit when one exists.
- The report includes exact gate evidence such as `ok verify` or the failing command.
- The report states the next executable step when work remains.

## Must Not

- Do not say complete while pending work or known errors remain.
- Do not bury blockers behind general progress language.
- Do not ask the owner to run ordinary local commands you can run.
