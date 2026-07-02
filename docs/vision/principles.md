# Principles

## Purpose

List the ranked invariants that every implementation and documentation change
must preserve.

## Ranked Invariants

1. Honesty. No fake success, no placeholder output presented as work, and no
   claimed gate that did not run. A task closes only through passed checks.
2. The harness directs; the model authors. Deterministic Rust owns control
   flow, file paths, retry policy, and completion judgment. The model writes
   bounded content inside the envelope chosen for the current step.
3. Every turn changes something. A retry changes instruction, scope, or budget;
   the engine never renders the same prompt twice for the same failed step.
4. Smallness. Every authored file stays at or below 200 lines. Product crates
   and docs also obey file-count budgets so splitting creates ownership rather
   than sprawl.
5. Context is engineered per turn. A prompt is a projection of durable state,
   not a transcript dump. Each region has an owner and a token budget.
6. Everything observable is durable. Tasks, steps, attempts, events, checks,
   usage, and exchange logs survive crashes and support proof bundles.
7. Pure core, effectful edge. Planning, rendering, parsing, checking, and
   escalation are pure functions over plain data. File IO, SQLite, clocks, and
   endpoint calls sit at the boundary.
8. One rule, one owner. Each behavior is specified in one contract page and
   implemented to that page.
9. The container is the safety model. The daemon runs YOLO inside the container;
   no permission prompt is part of the product.

## Enforcement

Line limits, topology, file-count budgets, link resolution, and banned language
are enforced by repository gates. Engine tests enforce prompt variation, finite
retry ladders, deterministic completion, and crash resume. Commit trailers and
handoffs name only commands that actually ran.
