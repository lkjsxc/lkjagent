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

Manuscript progress counts finished prose only under `manuscript/` chapter or
scene paths. README, catalog, cast, setting, outline, lore, and audit files do
not count. The progress projection records total manuscript words, complete
chapter paths, missing chapter paths, the next chapter path, and remaining
words.

## Write Contracts

A manuscript write contract names chapter prose, not reference detail. It names
one or more exact chapter paths, a target range, larger prose byte limits than
story-bible files, and weak classes such as scaffold-only, outline-only,
story-bible-only, placeholder, owner-terms-only, and generic-example.

Provider anomalies preserve the same next manuscript path. Recovery may shrink
the requested chunk to a scene section or block with an exact handoff, but it
must not reroute missing chapter prose to optional story-bible repair.

## Readiness

Manuscript readiness fails until requested exact paths exist, requested chapter
count is met when supplied, and manuscript prose reaches the required word
floor. Story-bible-only output is an explicit readiness failure.
