# Case State

## Purpose

Record the working state for this coding-agent pass.

## Objective

Improve documentation first, then make a small implementation slice match the
updated contract. Prioritize command-surface truth, prompt/tool protocol
clarity, workbench intent, and experiment evidence.

## Constraints

- Repository docs are the implementation contract.
- Keep authored files at or below 200 lines.
- Do not add runtime subagents.
- Do not claim gates that did not run.
- Preserve existing owner changes and historical tmp evidence.

## Assumptions

- The many deleted tmp paths shown by git status are pre-existing owner state.
- `tmp/lkjagent-yolo-redesign-handoff/` is advisory until docs/code adopt it.
- A reversible docs-first slice is preferable to broad unverified runtime churn.

## Risks

- Current docs may overclaim command support, especially owner CLI proof/help.
- Prompt protocol changes can break parser tests if docs and code diverge.
- Docker or live endpoint checks may be unavailable in this session.

## Evidence Requirements

- Save bootstrap inventory under `tmp/agent-runs/`.
- Run docs and line gates for docs-only changes.
- Run focused Rust tests before implementation claims.
- Use Docker Compose for final verification only if environment permits.

## Candidate Files

- `docs/current-state.md`
- `docs/product/cli.md`
- `docs/product/status-and-console.md`
- `docs/protocol/envelopes.md`
- `docs/context/prompt-assembly.md`
- `docs/tools/toolset-view-and-admission.md`
- `docs/evaluation/protocol-experiments.md`
- `README.md`
- `Dockerfile`
- `crates/lkjagent-app/src/args.rs`
- `crates/lkjagent-app/src/cli.rs`
- `crates/lkjagent-core/src/render.rs`

## Next Action

Patch docs to state the intended command surface, workbench contract, prompt
kernel cards, tool-call card constraints, and experiment evidence policy.
