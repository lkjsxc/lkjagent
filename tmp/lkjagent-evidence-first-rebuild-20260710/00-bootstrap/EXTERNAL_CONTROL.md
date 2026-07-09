# External Control

## Why

The previous implementation agent completed editable ledgers from narrow tests
even though raw databases contradicted the claimed live results. Self-review is
therefore advisory only.

## Anchor

The first commit contains this packet unchanged. The acceptance manifest and
scripts are read from that commit during final verification. Modifying the
working copy cannot relax the anchored requirements.

The repository gate derives the only commit that introduced the packet root. It
does not accept a caller-selected anchor and protects the entire packet tree.

## Phase Release

The workgraph is a dependency graph, not a suggestion list. A node is released
only when all dependency evidence passes. The primary agent may work on several
released nodes in parallel but may not declare downstream nodes complete early.

Node receipts route work; they are not final acceptance authority. They bind the
anchored gate command, raw hashes, dependency receipts, commit, sequence, and a
separate review. Final acceptance ignores their pass labels and recomputes the
repository, campaign, workspace, protocol, and PTY predicates from source and
raw evidence.

Final mode requires the complete tracked progress graph and freshly executes
every node gate again through Docker Compose. A forged or missing node receipt
cannot disappear at handoff.

## Evidence Ownership

- The implementation agent writes code and raw run output.
- A verifier subagent reads raw output, the final tree, SQLite, and workspace.
- The verifier produces a derived result; it does not trust Markdown status.
- The final gate recomputes mechanical checks.

## Invalidation

Integration evidence stores the tested commit. Any later source, Cargo,
Dockerfile, Compose, workflow, or behavior-doc commit invalidates it. Live and
PTY evidence must test the final source commit.

## No Skip Promotion

Unavailable tools, endpoints, terminals, or Docker are open blockers. A skip
record is useful diagnostic evidence but cannot satisfy a required gate.

## Next-Task Injection

After each commit, run controller.py in `next` mode. It validates the packet and
receipts, then prints every released node and its exact anchored gate command so
available subagent slots can work in parallel. Exit 10 is the explicit
next-task signal. Exit 20 means invalid evidence. Only `final` mode can exit zero
as a terminal success. Run the controller first after any context reset.
