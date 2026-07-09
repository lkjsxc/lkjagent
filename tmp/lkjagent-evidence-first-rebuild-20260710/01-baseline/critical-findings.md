# Critical Findings

## Severity One

1. State cells and RuntimeDecision rows decorate a task-and-step engine instead
   of replacing it. Production does not call the typed transition validator.
2. Recovery handles do no useful repair. The next call can repeat the same
   impossible request, and the second same-kind failure blocks the matter.
3. Generic finish and message paths can complete work without a tool admission,
   observation, artifact, or objective evidence.
4. The live runner overwrites real blocked states with a synthetic closed idle
   snapshot and counts idle polling as turns.
5. The clean Docker build copies Cargo.lock, but Cargo.lock is ignored and not
   tracked. Local evidence can pass while GitHub checkout fails immediately.

## Severity Two

6. Prompt state changes mostly append prose. Context is selected before the
   state decision, so state does not truly select its context.
7. Context ranks and lane budgets are metadata; all clean items are rendered
   before generic truncation.
8. Tool exposure is broad and static for exploration. The first rendered tool
   skeleton can bias the model toward premature finish.
9. Workspace writes flow through several incompatible systems with different
   ledger, index, and recovery behavior.
10. TUI rows are synthesized from queue and selected event tables instead of a
    canonical conversation table.

## Product Consequence

The current harness can look durable and well-tested while still doing only a
few useful decisions, losing long generated output at the token cap, recording
commands instead of intended content, and presenting blocked work as closed.
