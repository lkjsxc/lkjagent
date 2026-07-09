# Lanes And Budgets

## Lanes

- kernel: stable product and safety invariants;
- objective: owner goal and explicit constraints;
- state: current operation, obligations, and exit rule;
- workspace: selected record or project excerpts;
- conversation: causally relevant owner and agent messages;
- evidence: observations, checks, and artifact refs;
- conflict: unresolved claim summary when relevant;
- recovery: bounded fault and repair history;
- tools: current admitted schemas and one concrete example.

## Dynamic Allocation

Lane caps depend on prompt state. Project inspect emphasizes workspace evidence.
Act emphasizes the current target and content constraints. Recover reserves
space for repair and excludes unrelated history. Report emphasizes verified
outputs and citations.

## Reserve

Reserve endpoint output tokens before context selection. Reserve parser and stop
sequence margin. A long target triggers semantic operation splitting; it does
not consume the reserve by asking for impossible output.

## Feedback

Provider usage updates estimates for later calls. Track estimated versus actual
tokens by lane and model. Configuration changes must measurably alter the
compiled cap.
