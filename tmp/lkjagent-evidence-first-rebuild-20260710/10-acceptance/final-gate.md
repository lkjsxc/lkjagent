# Final Gate

## Required Command

Run the anchored acceptance script from the packet anchor against the final
repository and evidence root. It must exit zero and print:

    ALL_REQUIRED_ACCEPTANCE_GATES_PASSED

From the repository root, after source freeze, raw-material commit, verifier
receipt commit, push, and public CI, run:

    SOURCE=<frozen-source-commit>
    python3 tmp/lkjagent-evidence-first-rebuild-20260710/13-scripts/acceptance_gate.py \
      "$PWD" "$PWD/tmp/lkjagent-acceptance/$SOURCE" \
      tmp/lkjagent-evidence-first-rebuild-20260710

## Required Inputs

- packet anchor commit;
- final source commit;
- later raw-material and verification commits;
- clean worktree;
- final focused and integration command logs;
- clean archive and Docker logs;
- repeated live campaign roots;
- PTY campaign root;
- independent verifier result;
- public CI result.

## Rejection

The final gate rejects untracked evidence and any missing, skipped, stale,
blocked, self-contradictory, or summary-only evidence. It derives the anchor,
runs clean checkout and public CI checks, and rejects modification of any packet
file.

It also requires the complete tracked pre-freeze node receipt graph, then
reruns all 21 anchored xtask node gates plus eight named nonempty integration
suites through Docker Compose. Receipt status is process evidence, not a
substitute for these fresh executions.

## Final Handoff

Name:

- packet anchor, frozen source, raw-material, and verification commits;
- major docs and source changes;
- removed old authority;
- adopted and rejected experiment combinations;
- exact commands and results;
- live and PTY evidence paths;
- public CI link;
- residual non-blocking risks.

Do not call the project complete if this command cannot run.
