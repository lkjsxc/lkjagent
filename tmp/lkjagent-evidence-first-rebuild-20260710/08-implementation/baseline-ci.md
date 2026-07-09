# Baseline And CI

## Task A: Track Build Inputs

Remove Cargo.lock from ignore rules, commit it, build with locked dependencies,
and add a check that every Docker COPY source is tracked.

## Task B: Clean Archive

Add an xtask clean-checkout gate that exports HEAD to a temporary directory and
runs docs, lines, tests, and Docker Compose there. Do not inherit ignored local
files or environment secrets.

## Task C: Failure Fixtures

Create redacted fixtures from:

- impossible long content cap and identical retry;
- blocked matter summarized as closed;
- relative-root prefix error;
- diary command becoming canned body;
- readiness-only response closing work;
- duplicated logical TUI message and bottom-follow drift.

Each fixture names the original commit and expected failing predicate.

## Task D: CI

Make workflow configuration match Compose keys. Run the same clean archive gate
on pull request and main. Preserve bounded logs as artifacts when failing.

## Acceptance

Demonstrate failure before fixes where possible, then green clean archive and
Docker runs from the committed tree.
