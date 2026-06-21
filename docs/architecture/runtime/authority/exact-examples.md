# Exact Examples

## Purpose

Define how valid action examples are rendered for refusals and recovery.

## Decision Owner

`lkjagent-protocol` owns action rendering. Runtime authority selects the tool
and intent; protocol renders the example from the same schema dispatch accepts.

## Inputs

The renderer reads the current tool registry, active policy, required fields,
mode-specific field constraints, and recovery fault class.

## Output

The output is one exact action example plus a round-trip result. The example
must parse, normalize, and dispatch under the same active policy.

## Required Coverage

Canonical examples cover `memory.save`, `queue.list`, `fs.write`,
`fs.batch_write`, `graph.transition`, `artifact.next`, and `doc.audit`.
Examples differ by mode only when dispatch accepts the same difference.

## Prohibited States

- Runtime hand-writes examples separate from protocol schemas.
- Examples include unknown fields or omit required fields.
- Batch examples leave content tags unclosed.
- Recovery gives several competing examples after one parameter fault.

## Fixture

`parameter_fault_memory_save` and `parse_fault_unclosed_content` prove examples
repair the observed fault without creating a new dispatch refusal.

## Verification

Run `cargo test -p lkjagent-protocol schema_examples` and
`cargo test -p lkjagent-tools registry_examples`.

## Status

partially implemented.
