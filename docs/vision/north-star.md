# North Star

## Purpose

This file describes the end state lkjagent moves toward. Every task should
make the repository measurably closer to this state.

## The End State

A single container runs forever. Inside it, one small Rust daemon drives one
agent loop against a local 32k-context model. The owner talks to it by
dropping messages into a queue from a thin CLI and reading results from the
transcript. The agent never stops: when the queue is empty it maintains
its daemon heartbeat and waits for more owner work.

The harness is so small that an agent can hold the entire design in one
context window. Every behavior is written down in a file of at most 200
lines, and every file is reachable from a README within three links. The
documentation is the program; the Rust code is its faithful translation.

## What Good Looks Like

- A new agent reads AGENTS.md, three contracts, and ships a correct change
  without asking a human anything.
- The model's prefix cache hits on every turn between compactions; turn
  latency is dominated by generation, never by prompt re-evaluation.
- The context window never contains a byte the contracts did not allow in.
- A week of continuous operation leaves a durable transcript, clear task
  summaries, and an inspectable queue with no fabricated outcomes.
- The whole system rebuilds and verifies from a clean checkout with one
  docker compose command.

## What This Is Not

Not a coding-agent product for many users. Not a framework. Not a gateway to
messaging platforms. Not a model server. One owner, one brain, one box.
