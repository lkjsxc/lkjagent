# Final DoD Check

## Purpose

Record the item-by-item acceptance check for the workspace-first harness packet.

## Result

Checked on 2026-07-09 after corrective commits through `3c3fd2ef`.
Residual risk: a human-operated live TUI session was not run in this terminal,
but reducer, snapshot, pane, session-shadow, follow, quiet, and Docker gates
passed. The unstaged `data/README.md` deletion predates this final check and is
not part of the acceptance work.

## Items

| Item | Commits | Evidence | Result |
| --- | --- | --- | --- |
| Product docs describe behavior | a9a9fe08; 7b2ae344 | `102800-check-docs-final.log` | pass |
| Source and docs line limits | 7b2ae344; 3c3fd2ef | `102900-check-lines-final.log`; `104000-ledger-line-counts.log` | pass |
| Ordinary record turns write files and rows | 72fa3e2c; 8a6ea73a | `102000-app-tests.log`; finance index evidence | pass |
| Every owner turn writes transcript or inbox evidence | 9cd8069c | `workspace_evidence` in `102000-app-tests.log` | pass |
| Artifact creation writes files, rows, checks, response paths | 9cd8069c; e4c17d42 | `091000Z-artifact-size`; `102000-app-tests.log` | pass |
| Observed file-work failure has focused regression | 2dc0c841 | completion safety and runtime cell evidence in ledger | pass |
| Earlier blocked work prevents later response work | 2dc0c841 | docs-tree and completion safety evidence in ledger | pass |
| Prompt context deduped, sourced, bounded, non-JSON | 757ca459 | `084000Z-context-contract` | pass |
| Tool views selected by state, no full catalog default | 9cd8069c; e99919ef | `094700-core-tests.log`; protocol matrix | pass |
| XML-like action grammar and admission focused tests | 9cd8069c; e99919ef | `094200-core-admission.log`; tool_call evidence | pass |
| Recovery states cover parse, admission, effect, endpoint, checks | 9cd8069c; d8cfedfb; e99919ef | recovery tests and `094400-app-admission-rejection.log` | pass |
| TUI duplicate and bottom-follow regressions have tests | 97a8803d; 7b2ae344 | `101600` through `101900` TUI logs | pass |
| Deterministic replay passes | 7b2ae344 | `103200-smoke-replay-final.log` | pass |
| Quiet verify passes | 7b2ae344 | `103300-quiet-verify-final.log` | pass |
| Docker Compose verify passes | 7b2ae344 | `103400-docker-compose-verify-final.log` | pass |
| Live campaigns run or honest skips committed | 44a791ec; 72fa3e2c | `tmp/live-runs/20260708Tstandardenv/`; daily-use data | pass |
| Final handoff names commits, commands, evidence, risks | 3c3fd2ef | this file plus final response | pass |
