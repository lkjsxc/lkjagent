# Diary

## Intent Split

An owner statement containing concrete lived facts may use journal capture. An
owner command meaning "write today's journal" requests journal composition and
must not be stored as the entry body.

## Composition State

The journal-compose decision retrieves:

- owner turns and explicit records from the local date;
- completed todos and calendar activity;
- finance or project events admitted by owner policy;
- the existing same-day journal, if any;
- relevant prior reflection only when budget allows.

The model writes a bounded entry with known facts, concise reflection, and
uncertainty-safe wording. It must not invent events, purchases, people, or
feelings as fact.

## Path

Use life/journal/YYYY/MM/DD/entry.md in the configured timezone. Repeated writes
merge through expected fingerprint and preserve both existing content and new
reflection. Crash retry produces one update.

## No Evidence

If the day has little evidence, write a modest reflection about the available
context rather than canned missing-detail text. Ask one question only if the
owner's requested style or factual content truly cannot be inferred.

## Acceptance

The file contains neither the command nor canned filler, stays within budget,
has source refs, is indexed for the date, and can be recalled later.
