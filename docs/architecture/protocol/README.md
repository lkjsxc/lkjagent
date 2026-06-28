# Protocol

## Purpose

This directory specifies the language between the harness and the model: the
singular tag-based action format the model writes, the parser that reads it,
the system prompt that teaches it, and the recovery routes when it goes wrong.
Decision: [../../decisions/tag-action-protocol.md](../../decisions/tag-action-protocol.md).
Owned by the lkjagent-protocol crate.

## Table of Contents

- [action-format.md](action-format.md): the grammar the model writes.
- [parsing.md](parsing.md): the strict parser and parse fault rules.
- [batch-write.md](batch-write.md): the exact accepted `fs.batch_write` payloads.
- [compact-context.md](compact-context.md): the compact runtime card and exact next action prompt.
- [stop-token-policy.md](stop-token-policy.md): provider stop closure and parser logging.
- [system-prompt.md](system-prompt.md): the prefix document that teaches the protocol.
- [recovery.md](recovery.md): the taxonomy of failures and their bounded routes.
