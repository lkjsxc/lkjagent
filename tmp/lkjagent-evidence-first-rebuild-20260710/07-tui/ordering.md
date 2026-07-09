# Ordering

## Sequence

Allocate transcript sequence in the same SQLite transaction that commits the
message. Do not merge independent table IDs or timestamp strings.

## Causality

For one matter:

- owner message precedes derived decision;
- question precedes its owner answer;
- final agent message follows its verified effects;
- replacement draft occupies the same logical position.

Across matters, sequence reflects commit order. Matter priority affects runtime
selection, not historical transcript reordering.

## Pagination

Load a stable sequence window with a cursor. New rows append without reshuffling
existing rows. Older-page loading anchors the current top visible message.

## Tests

Cover tied timestamps, simultaneous matters, queued input during endpoint work,
restart, draft replacement, more than forty rows, and identical text with
different logical IDs.
