# Write Batches

## Purpose

Define bounded content writing after artifact planning.

## Contract

Long content is written in small semantic batches, not one raw giant
`fs.write`. Raw `fs.write` is capped at 1,800 bytes. `fs.batch_write` is capped
at 1,800 bytes per file and 6,000 bytes per batch. Each contract names exact
paths, required sections, content expectations, forbidden weak phrase classes,
and verification notes. The model writes the meaningful content; the harness
validates the batch against the stored contract before mutation.

## Next Batch

`artifact.next` returns the next bounded batch contract from audit gaps. It
walks weak semantic leaves through a durable root cursor and names exact paths,
limits, required sections, and forbidden weak phrase classes. It does not
return generated body prose.

`artifact.next` is fact-only: `next_decision_required=true` means the candidate
cannot dispatch directly. The observation creates a runtime event, then the next
decision may force the canonical `fs.batch_write` action for those weak paths.
If the current cursor has no next weak path, `artifact.next` requests
`artifact.audit` or focused reads instead of repeating a placeholder write.
Successful `fs.write` and `fs.batch_write` calls mark matching planned cursor
paths complete only after contract validation. Completion still requires real
content and a passing artifact audit.

## Empty Roots

A missing or empty artifact root is not success. The next route records a root
identity contract for `catalog.toml`, a bounded navigation README, and the
smallest meaningful semantic leaves for the profile. A story bible never
recovers from an empty root by writing one monolithic README.

## Payload Faults

After invalid JSON-in-`files`, child `<file>` tags inside `<files>`, max-token
truncation, unclosed content tags, or a payload-limit refusal, recovery blocks
another large raw write and routes to `artifact.next`, `artifact.audit`, or a
concrete bounded line-protocol `fs.batch_write` under the current root. The
next example must be parseable, registry-valid, and below the existing limits.
