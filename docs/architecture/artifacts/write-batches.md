# Write Batches

## Purpose

Define bounded content writing after artifact planning.

## Contract

Long content is written in small semantic batches, not one raw giant
`fs.write`. Each batch names exact paths, required sections, content
expectations, and verification notes. The model writes the meaningful content;
the harness decides whether the batch is admissible and complete.

## Next Batch

`artifact.next` or an equivalent planner returns the next bounded batch from
audit gaps. It should prefer missing or weakest semantic leaves and produce an
admitted `fs.batch_write` skeleton for the model to fill.

## Payload Faults

After max-token truncation or unclosed content tags, recovery blocks another
large raw write and routes to artifact planning or bounded batch writes.
