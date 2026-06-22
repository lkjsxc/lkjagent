# Write Batches

## Purpose

Define bounded content writing after artifact planning.

## Contract

Long content is written in small semantic batches, not one raw giant
`fs.write`. Raw `fs.write` is capped at 1,800 bytes. `fs.batch_write` is capped
at 1,800 bytes per file and 6,000 bytes per batch. Each batch names exact paths,
required sections, content expectations, and verification notes. The model
writes the meaningful content; the harness decides whether the batch is
admissible and complete.

## Next Batch

`artifact.next` returns the next bounded batch from audit gaps. It walks weak
semantic leaves through a durable root cursor, names exact paths, and produces
an admitted `fs.batch_write` example only when that example contains
profile-specific content.

If the current cursor has no next weak path, `artifact.next` requests
`artifact.audit` or focused reads instead of repeating a placeholder write.
Completion still requires real content and a passing artifact audit.

## Payload Faults

After max-token truncation, unclosed content tags, or a payload-limit refusal,
recovery blocks another large raw write and routes to artifact planning or
bounded batch writes.
