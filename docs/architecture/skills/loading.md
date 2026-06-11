# Loading

## Purpose

Specify progressive disclosure: every skill costs one index line until the
moment it is needed, then its whole body and nothing else. Loading is
designed around the cache discipline in
[../context/caching.md](../context/caching.md).

## The Index

The prefix carries one line per skill in the library:

```
fix-flaky-test: A test passes alone but fails in the suite.
```

Name, colon, trigger sentence, within the 512-token index budget from
[../context/budgets.md](../context/budgets.md). When the library outgrows
the budget, the entries with the oldest use stamps degrade to name-only
until usage promotes them again; degradation order is deterministic so the
prefix stays byte-stable between compactions.

## Loading a Body

skill.use appends the file verbatim as one immutable skill frame:

```
<skill>
# Skill: fix-flaky-test
...
</skill>
```

- A body is loaded at most once per window; a second skill.use gets a
  notice pointing at the existing frame
  ([../context/hygiene.md](../context/hygiene.md)).
- Concurrent loaded bodies are capped (6,144 tokens total); past the cap,
  skill.use is refused with a notice naming what is loaded, and the model
  chooses what matters after the next compaction drops all bodies.
- Compaction drops every body; the model reloads what the resumed task
  still needs. Reloading is cheap exactly because bodies are appended,
  cache-friendly frames.

## Visibility Rules

- A skill saved or refined mid-session becomes visible in the index at the
  next compaction, never immediately: editing the prefix mid-session would
  invalidate the cache for one line of text.
- The body the model just wrote with skill.save is already in its window;
  deferral costs nothing in practice.
- Startup indexes the whole library fresh; restart is the other lawful
  index refresh point.

## Failure Cases

| Case | Response |
| --- | --- |
| unknown skill name | tool error listing the index names |
| oversized skill file | refused at save time per [format.md](format.md), never at load time |
| index over budget | deterministic degradation, notice in transcript |

## Status

design-only.
