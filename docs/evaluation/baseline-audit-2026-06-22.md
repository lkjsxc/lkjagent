# Baseline Audit 2026-06-22

## Purpose

Record the repository state before the current redesign slice. This is an
honest input ledger, not proof that later behavior is implemented.

## Case State

- Objective: harden lkjagent against the owner-uploaded Japanese cookbook,
  parser, parameter, compaction, repeated action, and queue-isolation failures.
- Constraints: docs are the contract, every file remains at or below 200
  lines, effects stay outside pure reducers, Docker Compose gates are
  authoritative, and no behavior is claimed without a gate.
- Assumptions: existing uploaded-run fixtures are useful but not sufficient;
  code and docs may be replaced when they contradict the contract.
- Risks: broad redesign can leave docs ahead of implementation; each slice
  must name that state truthfully.
- Evidence requirements: documentation topology, line limits, focused tests,
  replay coverage, and Docker Compose verification.

## Commands Run

```text
git status --short
find docs -maxdepth 5 -type f | sort
find . -maxdepth 4 -type f | sort
docker compose run --rm verify
```

## Observed Results

- `git status --short` reported one pre-existing untracked file:
  `tmp/prompt01.md`.
- `find docs -maxdepth 5 -type f | sort` produced 219 tracked and untracked
  documentation files at the start of the slice.
- `find . -maxdepth 4 -type f | sort` produced 2742 files at the start of the
  slice, including repository internals and build output.
- `docker compose run --rm verify` exited 0 and printed `ok verify`.

## Initial Gap Summary

- The docs already formed a recursive tree with README files, but weighted
  state guard semantics needed a single explicit contract page.
- The cookbook scaffold profile still allowed bread-specific paths to act as
  the default cookbook shape.
- The artifact next-batch example still emitted bread-specific cookbook prose.
- The Japanese cookbook drift fixture existed, but the implementation needed a
  stronger objective-match guard for Japanese cookbook roots.

## Completion Check

This baseline is complete when later handoffs cite this file, preserve the
pre-existing untracked file, and report any gate failures separately from this
initial passing Docker Compose verification.
