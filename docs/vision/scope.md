# Scope

## Purpose

Define what belongs in the active product and what waits for measured need.

## In Scope

- One owner queue processed sequentially by one daemon authority.
- One OpenAI-compatible content endpoint.
- One SQLite ledger below the runtime data root.
- One separately configured visible workspace root.
- Arbitrary state cells and persisted runtime decisions.
- Decision-specific XML-like model envelopes and tool views.
- Safe bounded file inspection, creation, exact editing, and checks.
- Canonical owner/final conversation messages and a terminal workbench.
- Dated records, sourced workspace memory, projects, and bounded reports.
- Docker Compose verification and source-bound real endpoint/PTY evidence.

## Deferred

Shell execution, path moves, archive policy, FTS, embeddings, relation graphs,
and rich artifact graphs require a tracked experiment and complete task evidence.

## Excluded

- Multi-user hosting, web UI, messaging gateways, and remote service control.
- MCP, runtime sub-agents, and permission-prompt workflow.
- Prompt-only policy or a second tool registry.
- Model-selected matter completion.
- Raw transcript replay as context.
- Unscheduled self-work without a durable obligation or wake.

## Deletion Rule

Source outside this boundary is removed after its useful failure fixture or safe
primitive is extracted. Git history is the archive.
