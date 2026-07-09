# Anchored Contract

## Anchor

Commit this packet unchanged before implementation. Final verification reads the
acceptance files and scripts from that Git commit. The implementation may add
stronger gates but cannot weaken the anchored requirements.

The gate derives the single packet-introduction commit from Git history and
compares the entire packet tree. It never trusts an anchor supplied at runtime.

## Evidence

Each gate is derived from source, tests, commands, raw SQLite, workspace bytes,
PTY trace, and Git timestamps. Editable status tables and model prose are not
accepted evidence.

## Independent Verification

A verifier that did not author the implementation reads the raw artifacts and
runs the gate. The implementation agent may fix failures, then requests another
verification.

## Freshness

All integration, Docker, live, and PTY evidence names the frozen source commit.
Raw evidence is committed in one or more material commits. Only the four
verifier files may follow in a distinct verification commit. Final post-freeze
workgraph receipts may join raw material under `tmp/lkjagent-progress`; no
source, docs, build, or other path may change. The receipt binds
the prior material commit, while the gate derives all four ordered boundaries
and rejects every post-freeze non-evidence change.

## Unavailable

Unavailable Docker, endpoint, PTY, or public CI remains a failed required gate.
The coding agent may report the blocker but may not issue a successful handoff.
