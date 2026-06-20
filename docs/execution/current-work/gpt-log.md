# GPT Log

## Purpose

This file owns the single synthesized Markdown handoff file that the owner can
send to ChatGPT or another stronger external model.

## Contract

- Maintain one current Markdown file under `data/logs/`.
- Rewrite the file as a bounded current snapshot, not an append-only transcript.
- Do not imply that ChatGPT monitors the file; it is printed or opened and sent
  manually by the owner.
- Allow roughly 100,000 characters so the file carries enough transcript
  evidence for a manual ChatGPT handoff.
- Include objective, constraints, ranked tracks, plan, evidence, faults,
  recent transcript, touched paths, token usage, and verification.
- Expose the path through status and the `lkjagent gpt-log` command.

## Implementation Hooks

- Source: `crates/lkjagent-runtime/src/gpt_log.rs`
- Source: `crates/lkjagent-cli/src/gpt_log.rs`
- Tests: `crates/lkjagent-cli/tests/gpt_log.rs`
- Tests: `crates/lkjagent-runtime/tests/daemon_loop.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The only available run record is scattered across raw transcript rows.
- The handoff log omits faults or active state tracks.
- The handoff file is too short to carry useful transcript evidence.
- Status does not show the current log path.

## Status

implemented
