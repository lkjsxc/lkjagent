# Manuscript Lifecycle

## Purpose

Define story manuscript work as a chapter-prose lifecycle distinct from a
story-bible lifecycle.

## Lifecycles

`story-bible` means the owner asked for setting, cast, plot, continuity, style,
or other reference files. These files support later prose but are not chapter
manuscript evidence.

`manuscript` means the owner asked for chapter or scene prose, an exact
`stories/.../manuscript/*.md` path, a prose word target, or a complete draft.

`story-bible-then-manuscript` means reference files are useful, but chapter
prose remains the owner deliverable. Root identity can come first; optional lore
must not delay the first requested chapter path after identity exists.

## Objective Facts

A manuscript objective records these deterministic facts when present:

- lifecycle;
- target words and the accepted word floor;
- chapter count and per-chapter word range;
- exact manuscript paths;
- inferred path pattern;
- forbidden generic roots such as `structured-output`.

For approximate word targets, readiness accepts at least 85 percent of the
requested total. Exact lower-bound wording uses the stated lower bound.

## Progress Facts

Manuscript progress counts finished prose only under `manuscript/` chapter
paths. Scene atoms under `manuscript/scenes/<chapter>/` are draft units until
deterministic assembly writes the chapter path. README, catalog, cast, setting,
outline, lore, scene atoms, and audit files do not count as final manuscript
words. The progress projection records total manuscript words, complete chapter
paths, missing chapter paths, unassembled scene atoms, the next exact write
path, and remaining words.

## Write Contracts

A manuscript write contract names prose, not reference detail. It names one
exact chapter path or scene-atom path, the safe byte budget for the current
write, and weak classes such as scaffold-only, outline-only, story-bible-only,
placeholder, owner-terms-only, and generic-example.

Provider anomalies preserve the same next manuscript path. Recovery shrinks the
same exact path to a smaller scene or subsection contract; it must not block
with a broad handoff or reroute missing chapter prose to optional story-bible
repair.

## Assembly

Scene atoms are stored under `manuscript/scenes/<chapter>/`. When every scene
atom in a chapter directory is strong enough and their combined prose satisfies
the chapter floor, `artifact.audit` deterministically assembles them in lexical
path order into `manuscript/<chapter>.md` before the readiness check. The audit
output records `manuscript_assembly=assembled`, `assembled_target`,
`assembled_word_count`, and `source_atom_paths`.

Assembly is not completion. Completion still requires the chapter file to exist,
real manuscript word counts to meet the floor, artifact readiness evidence, and
the central completion gate.

## Readiness

Manuscript readiness fails until requested exact paths exist, requested chapter
count is met when supplied, and manuscript prose reaches the required word
floor. Story-bible-only output is an explicit readiness failure.
