# Principles

## Purpose

List the ranked invariants that every implementation and documentation change
must preserve.

## Ranked Invariants

1. Honesty. No fake success, no placeholder output presented as work, and no
   claimed gate that did not run. A case closes only through fresh passed checks.
2. Durable authority. State rows and persisted `RuntimeDecision` rows direct
   runtime behavior. Prompt-only policy, dispatcher-only policy, and model-owned
   completion are invalid.
3. The harness directs; the model authors. Deterministic Rust owns state
   selection, tool admission, file paths, retry policy, and completion judgment.
   The model writes bounded content or requests an operation exposed by the
   current decision.
4. Prompt and admission match. The active state vector derives the output
   grammar and `ToolSetView`; admission accepts only actions from that same
   persisted view.
5. Context is engineered per turn. A prompt is a projection of durable
   source-tagged context items, not a transcript dump. Contaminated material is
   excluded from normal prompts and contradictions render as unresolved
   conflicts.
6. Every turn changes durable state. A retry changes instruction, scope, budget,
   state, or recovery policy; the runtime never renders the same failed prompt as
   if nothing happened.
7. Smallness. Every authored file stays at or below 200 lines. Product crates
   and docs obey file-count budgets so splitting creates ownership rather than
   sprawl.
8. Everything observable is durable. Cases, events, state cells, decisions,
   admissions, observations, checks, usage, context edges, and exchange refs
   survive crashes and support proof bundles.
9. Pure core, effectful edge. Reduction, selection, rendering, parsing,
   admission, checking, and completion are pure functions over plain data. File
   IO, SQLite, clocks, shells, and endpoint calls sit at the boundary.
10. One rule, one owner. Each behavior is specified in one contract page and
    implemented to that page.
11. The container is the safety model. The daemon runs YOLO inside the
    container; no permission prompt is part of the product.

## Enforcement

Line limits, topology, file-count budgets, link resolution, and banned language
are enforced by repository gates. Focused tests enforce state preservation,
fingerprint stability, prompt/admission agreement, context hygiene, finite retry
ladders, deterministic completion, and crash resume. Commit trailers and
handoffs name only commands that actually ran.
