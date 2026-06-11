# Protocol

## Purpose

This directory specifies the language between the harness and the model: the
tag-based action format the model writes, the parser that reads it, the
system prompt that teaches it, and the recovery rules when it goes wrong.
Decision: [../../decisions/xml-action-protocol.md](../../decisions/xml-action-protocol.md).
Owned by the lkjagent-protocol crate.

## Table of Contents

- [action-format.md](action-format.md): the grammar the model writes.
- [parsing.md](parsing.md): the strict line-oriented parser rules.
- [system-prompt.md](system-prompt.md): the prefix document that teaches the protocol.
- [recovery.md](recovery.md): the taxonomy of failures and their bounded responses.
