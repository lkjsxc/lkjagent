# Progress

## Purpose

Define progress keys that prevent loops while allowing legitimate retries with
new facts.

## Progress Key

`ProgressKey` contains:

- `target`: normalized root, path, case id, or verification target.
- `action_class`: audit, identity-write, content-write, inspection, evidence,
  verification, completion, compaction, or handoff.
- `fact_digest`: digest of the facts that justified the action.

The repeated-action guard compares progress keys, not raw model action text.
Two actions with different paths, contracts, or facts are different progress.
Two same-root audits after the same `missing_root` fact are the same stalled
progress and must change shape.

## Advancement

A progress key advances when one of these facts changes:

- a write records matching contracted paths;
- an audit reports a new status or failure set;
- a weak-path cursor advances;
- verification passes or fails with new evidence;
- recovery records a new fault class or exhausted route;
- blocked handoff records exact blockers.

## Loop Guard

If the latest fact digest still says root missing and no write or handoff has
advanced, the resolver forces the root identity write and blocks same-root
`doc.audit`. If the identity write itself repeats without mutation progress,
the resolver changes to blocked handoff with exact blockers.
