# Field Constraints

## Global Bounds

- decision and ledger IDs: 1..128 lowercase ASCII letters, digits, underscore,
  hyphen, or colon as declared by the owning ID type;
- normalized path: 1..1024 UTF-8 bytes, no absolute root, parent component,
  NUL, control character, symlink traversal, or platform collision;
- fingerprint: exactly `sha256:` plus 64 lowercase hexadecimal digits;
- title: 1..160 conservative tokens and no control characters;
- ordinary text field: 0..4096 UTF-8 bytes unless the decision records a lower
  bound; artifact body uses the persisted semantic-unit output budget;
- count: canonical decimal with no sign or leading zero, within the rendered
  minimum and maximum;
- date: real ISO `YYYY-MM-DD`; instant: RFC 3339 with offset;
- repeatable values use repeated tags and retain order, never comma parsing.

## Target Modes

Record update tools carry `target_mode` equal to `create` or `update`. Create
requires title and forbids document_id. Update requires document_id and current
expected_fingerprint; title is optional. The table abbreviation "document ID or
title" expands only through this rule.

Workspace create forbids an existing path. Replace, append, patch, and move
require expected_fingerprint. A patch contains bounded exact old/new hunks and
must apply once; it is not a shell diff command. Move requires distinct source
and target paths and a precomputed link-repair plan.

## Enums

- TODO state: open, waiting, done; priority: low, normal, high, urgent;
- finance direction: income, expense, transfer; currency: three uppercase ASCII
  letters; amount: positive decimal with at most 18 integral and 8 fractional
  digits;
- Boolean text: exactly true or false;
- check_set and allowed_command_id come from decision-rendered closed catalogs;
- conflict choice must equal one of the current conflict's source IDs;
- strategy must equal the next eligible ladder entry stored in the decision.

## Content And Question Envelopes

Content requires decision ID, context fingerprint, declared document kind,
title, body, and one or more current source refs. Question requires one reason
code and one direct prompt of at most 120 conservative tokens. Message requires
summary plus only verified evidence refs. No envelope may add a path, claim,
tool, or unit absent from the persisted decision.
