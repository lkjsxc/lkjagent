# Skill: Git Checkpoint

## Purpose

Commit a verified local slice with an honest message once the work reaches a
reviewable boundary.

## Trigger

Work reached a coherent local state with tests or checks that can be named.

## Context

- `git status --short` shows the intended commit scope.
- `git diff --stat` shows whether the slice stayed reviewable.
- The repository commit protocol controls trailers and tested claims.

## Procedure

1. Run `git status --short` and confirm every changed path belongs to this slice.
2. Run `git diff --stat` and inspect any unexpectedly large file.
3. Run the focused gate and the required repository gate for the slice.
4. Stage with `git add -A` only after the diff and gates match the intended scope.
5. Commit with an intent line plus Tested and Not-tested trailers matching real commands.

## Checks

- `git status --short` before staging names only intended paths.
- The focused gate exits 0 and the repository gate prints its ok line.
- `git log -1 --pretty=oneline` shows the new commit subject.

## Must Not

- Do not commit unrelated dirty work.
- Do not put commands in Tested that did not run.
- Do not hide known gaps; use Not-tested with the reason.
