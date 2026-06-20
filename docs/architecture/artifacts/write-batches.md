# Write Batches

## Purpose

Define bounded content writing after artifact planning.

## Contract

Long content is written in small semantic batches, not one raw giant
`fs.write`. Each batch names exact paths, required sections, content
expectations, and verification notes. The model writes the meaningful content;
the harness decides whether the batch is admissible and complete.

## Next Batch

`artifact.next` returns the next bounded batch from audit gaps. It prefers the
weakest semantic leaves, names exact paths, and produces an admitted
`fs.batch_write` skeleton for the model to fill.

The skeleton is not completion evidence. Completion still requires real content
and a passing artifact audit.

## Payload Faults

After max-token truncation or unclosed content tags, recovery blocks another
large raw write and routes to artifact planning or bounded batch writes.
