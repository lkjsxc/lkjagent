# Maintenance

## Purpose

Define the active-mode boundary for idle maintenance.

## Admission

Maintenance starts only when no owner row is pending, no owner case is active,
no owner case is recoverable, compaction is not required, and a maintenance
directive is due or already active.

## Policy

Maintenance does not inherit graph task policy. It may use only tools admitted
by maintenance mode: bounded memory work, queue inspection when useful, and
explicitly admitted state inspection. It cannot write workspace files, run
shell commands, mutate owner queue rows, or ask the owner about internal
uncertainty.

## Outcomes

A real maintenance outcome records actual changed row IDs or evidence. A no-op
outcome sets cooldown and writes no memory row merely saying nothing happened.
Merge, rewrite, and prune claims must name source and changed IDs.
