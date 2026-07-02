# Explore Toolset Of Ten

## Purpose

Record the decision to cap model-chosen tools.

## Context

Scripted steps do not need model-selected tools. Discovery still needs bounded
workspace and memory actions.

## Decision

Explore steps expose `tools.registry.count=10` tools: read, list, tree, search,
write, shell, memory find, memory save, plan note, and finish.

## Consequences

Most tasks run without tool choice. When exploration is needed, the prompt can
list every legal form inside the budget.

## Rejected Alternatives

A broad registry would require admission policy and refusal recovery, recreating
the legality maze the engine removes.
