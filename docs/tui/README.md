# TUI

## Purpose

Define the terminal view over canonical conversation and runtime evidence.

## Table of Contents

- [transcript-model.md](transcript-model.md): durable message identity,
  ordering, replacement, and diagnostic separation.
- [scrolling.md](scrolling.md): wrapping, viewport anchors, input preservation,
  resize, and PTY evidence.

## Boundary

The TUI reduces store-backed view events into presentation state. It does not
route work, infer completion, synthesize conversation, or become another
runtime authority.
