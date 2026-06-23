# Dictionary Profile

## Purpose

Define the dictionary content artifact profile used by dictionary failure
fixtures such as the bread dictionary logs.

## Contract

A dictionary artifact contains an index plus entry groups. Each entry has a
term, class, definition, and evidence-bearing usage. Requested fields such as
pronunciation, etymology, examples, variants, or cross-references are required
for the configured readiness threshold.

Example root:

```text
dictionaries/<semantic-title>/
  README.md
  manifest.md
  entries-a-f.md
  entries-g-l.md
  entries-m-r.md
  entries-s-z.md
  audit.md
```

## Invariants

- A term list is not a detailed dictionary.
- Definitions must be non-trivial and not repeat the term as the explanation.
- Examples must exist when the owner requests examples.
- Alphabetical organization or a searchable index must exist.
- Verification must not claim unsupported fields such as IPA or etymology.

## Failure Cases

- A 32-entry bread terminology list is treated as complete.
- A dictionary entry file omits requested pronunciation, etymology, or examples.
- Entries are duplicated across sections without cross-reference intent.
- Completion evidence claims fields that are not present in the observed root.

## Verification

Readiness tests include shallow bread terminology as bad, meaningful bread
dictionary entries as good. Unsupported verification-claim detection remains
open.

## Related Files

- [content-readiness.md](content-readiness.md)
- [completion-gates.md](completion-gates.md)
- [write-batches.md](write-batches.md)

## Status

partially implemented for directory dictionary readiness in
`artifact.audit kind=dictionary`; manifest adoption, alphabetical organization,
and unsupported verification-claim checks remain open.
