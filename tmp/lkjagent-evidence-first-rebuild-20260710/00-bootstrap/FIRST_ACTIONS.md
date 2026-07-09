# First Actions

## Establish An Immutable Anchor

1. Confirm repository root, branch, status, and current commit.
2. Extract this directory directly under tmp.
3. Force-add only this packet and commit it unchanged.
4. Run packet_lint.py against the committed packet.
5. Confirm `git show --name-only --format= HEAD` contains only the packet and
   `git log --diff-filter=A --format=%H -- tmp/lkjagent-evidence-first-rebuild-20260710/README.md`
   prints exactly that commit.

Do not modify source before the anchor exists.

## Capture The Baseline

Run and save exact output:

    git status --short
    git log --oneline --decorate -n 30
    git ls-files Cargo.lock data/README.md
    docker compose --profile verify run --rm lint
    docker compose --profile verify run --rm test
    docker compose --profile verify run --rm verify

Record the clean-checkout script's expected initial failure without changing
source. Repair repository determinism only after the first docs-authority
commit. A local pass with an ignored dependency is not a repository pass.

## Reproduce The Three Critical Failures

Before redesign, preserve focused fixtures for:

- a 1,500-word write paired with a 768-token output cap, identical retry, then
  block;
- the live runner reporting closed after the real matter is blocked;
- a readiness-only message closing an action request without effect evidence.

Also preserve the relative workspace-root fs.tree prefix failure and the diary
command producing canned content.

## First Product Commit

Rewrite docs to make event-reduced state the only authority. Remove claims that
the attached live campaigns passed. Name all known gaps. Commit this docs-only
change before runtime code.
