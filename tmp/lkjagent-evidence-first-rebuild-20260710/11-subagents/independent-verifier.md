# Independent Verifier

## Objective

Decide acceptance from anchored requirements and raw facts.

## Independence

Do not author product source for the slice being verified. Do not accept the
implementation agent's checkboxes, summaries, or verbal claims.

## Procedure

1. Read anchored acceptance files from Git.
2. Confirm final commit and clean worktree.
3. Run focused, integration, clean archive, and Docker gates.
4. Read live SQLite and workspace directly.
5. Read PTY trace and recompute invariants.
6. Compare derived state with every result summary.
7. Check evidence freshness and configuration fingerprint.
8. Produce verifier-report.md, verifier-commands.log, and
   verifier-artifacts.tsv, then hash them into independent-verifier.tsv.
9. Run controller.py in final mode.

## Output

The receipt records source commit, prior evidence-material commit, packet tree
fingerprint, status, report/log/manifest refs, and their SHA-256 fingerprints. The manifest
lists every raw input independently opened. The report has `# Findings`,
`# Commands`, and `# Verdict` sections. On failure, name the first executable
repair and affected dependency nodes. On pass, include the exact controller
output and public CI evidence.
