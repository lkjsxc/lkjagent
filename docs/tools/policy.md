# Policy

## Purpose

Define the layered policy that derives tool availability for one decision.

## Layers

Tool access is computed through ordered layers:

1. global safety constraints;
2. owner configuration;
3. workspace boundary constraints;
4. active state affordances;
5. case and artifact constraints;
6. retry and budget suppressors;
7. evidence requirements; and
8. recovery constraints.

Each layer returns allow, suppress, or deny with a structured reason. Suppressed
and denied tools do not appear in normal prompt text. Denial reasons are kept for
status, logs, proof bundles, and recovery decisions.

## Workspace Rule

Workspace policy canonicalizes paths before admission. Absolute paths and `..`
escapes are denied even if another layer would otherwise allow the tool.

## Evidence Rule

When a state cell requires evidence, the tool view exposes only operations that
can gather or validate that evidence. Tools irrelevant to the active evidence
need are hidden rather than baiting refused actions.

## Failure This Prevents

Tool permission can flex by state and recovery need without creating a broad
registry that the model must probe by trial and error.
