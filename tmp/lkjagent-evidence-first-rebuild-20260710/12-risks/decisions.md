# Decisions

## Fresh Native Store

Choose a fresh event and projection schema. Preserve workspace files and rebuild
document projections. Do not maintain production bridge compatibility.

## Separate Workspace

Choose one host-visible workspace mount separate from runtime data. This matches
owner expectations and makes backup, inspection, and project work clear.

## Workspace And Ledger Authority

Workspace owns owner-readable content. SQLite owns runtime control and derived
search metadata. Effect journal reconciles the two.

## Attribute-Free Model Exchange

Keep XML-like model exchange and internal JSON serialization separated. Test
direct field tags against the current name/value argument shape.

Use one canonical tool registry. Questions and reports are envelopes, not tools;
all model- or harness-selected effects receive the same typed admission lineage.

## Scalar Configuration

Use one tracked data/lkjagent.json whose complete registry contains only string,
integer, and boolean values. Reject arrays, nested data, null, unknown keys, and
diagnostics-only settings. Keep secret values in named environment variables.

## Managed Token Measure

Enforce the 512-token page ceiling with the larger of provider tokens and a
deterministic Japanese-aware conservative count. Large owner project files stay
in place as external source and receive bounded retrieval rather than rewriting.

## No Model Finish

Replace finish with progress reporting and harness-computed completion.

## Event-Driven Quiescence

Keep the daemon running, but do not burn endpoint calls or count idle polls.
Continue active matters until verified completion, explicit waiting, or
exhausted visible recovery.

## Measured Adoption

Retain a broad idea bank in evaluation, but ship only combinations that win
real scenarios and deterministic safety gates.

## Frozen Evidence Boundary

Keep packet introduction, final source, raw evidence material, and verifier
receipt as four distinct ordered Git commits. After source freeze, permit only
the source-keyed evidence tree and final progress receipts to change; after
material freeze, permit only the four verifier files. Final acceptance
self-locates the immutable packet and
never executes a caller-selected replacement gate.
