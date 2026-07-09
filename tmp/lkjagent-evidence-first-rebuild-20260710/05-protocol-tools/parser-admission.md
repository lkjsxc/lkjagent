# Parser And Admission

## Parser

The parser returns a typed action or a typed fault with byte range, tag, and
repair class. It rejects unknown tags, duplicate scalars, duplicate fields,
crossed tags, bad entities, attributes, extra roots, missing closing tags,
empty executable values, and oversized content.

## Decision Binding

Admission requires exact decision ID, context fingerprint, expected grammar,
tool-view fingerprint, and current operation lease. A stale response becomes a
durable stale-decision observation and can never execute.

## Tool Validation

Validate tool membership, field set, primitive types, ranges, path bounds,
allowed operation, budget, idempotency key, repeat guard, and evidence need.

## Exactly Once

Every parsed action creates one admission row, including rejected actions.
Accepted actions create one prepared effect journal entry before execution.
Repeated accepted calls with the same idempotency key return the durable prior
result.

## Finish Removal

Remove the special finish shortcut. A model may report progress through a normal
admitted action, but only the harness creates completion candidates.
