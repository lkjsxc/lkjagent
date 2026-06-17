# Skill: Workspace Survey

## Purpose

Map an unfamiliar repository before changing it, using only cheap local
commands and the workspace's own instructions.

## Trigger

A task starts in a workspace whose layout, rules, or gates are not known.

## Context

- `AGENTS.md` holds local agent rules and overrides.
- `README.md` and `docs/current-state.md` usually name the project state.
- `docs/execution/current-blockers.md` names ordered implementation work when present.

## Procedure

1. Run `pwd` to anchor the workspace root in the transcript.
2. Run `find . -name AGENTS.md -print` and read every applicable file.
3. Run `rg --files` to map source, docs, tests, and configuration.
4. Run `git status --short` to separate existing work from your changes.
5. Read the current-state or task docs before deciding what to edit.

## Checks

- `pwd` prints the active workspace path.
- `find . -name AGENTS.md -print` prints each instruction file that must govern edits.
- `git status --short` exits 0 and shows whether the tree is clean.

## Must Not

- Do not edit before reading applicable AGENTS.md files.
- Do not assume the first README is the full contract when docs/ exists.
- Do not overwrite existing uncommitted work.
