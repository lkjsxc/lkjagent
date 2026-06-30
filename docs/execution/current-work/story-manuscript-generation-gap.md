# Story Manuscript Generation Gap

## Purpose

Record the live novel-generation gap so a later implementation can focus on
manuscript-scale story output rather than re-solving completed root, routing,
and readiness repairs.

## Owner Request

The owner asked whether lkjagent can actually write a larger English high
school romance novel of about 10,000 words. Live runs were attempted against the
configured endpoint model `google/gemma-4-26b-a4b-qat` with
`LKJAGENT_CONTEXT_LENGTH=24576`.

## Observed Evidence

The evidence below lives in `/tmp` and is not a checked-in fixture.

- `/tmp/lkjagent-high-school-romance-20260630T071652Z` asked for an English
  high-school romance novella named `The Bell Rings Twice` of about 10,000
  words. It created `stories/bell-rings-twice` and story-bible files, but the
  summary reported `file_count=11`, `english_word_count=473`, and no manuscript
  chapter files. Final status was still `daemon_state=working`,
  `active_mode=recovery`, `evidence_gaps=artifact-readiness`, and
  `authority_allowed_tools=fs.batch_write`.
- `/tmp/lkjagent-high-school-romance-long-20260630T072532Z` repeated the task
  with a larger endpoint reserve and explicit ten chapter files of 900 to 1100
  words each. The initial run again produced story-bible files only:
  `file_count=11`, `english_word_count=453`, and no manuscript directory.
- Continuing that same data directory with owner guidance to write
  `stories/bell-rings-twice/manuscript/chapter-one.md` advanced the story-bible
  weak paths but still did not create the manuscript file. The later summary
  reported `english_word_count 741` and
  `check stories/bell-rings-twice/manuscript/chapter-one.md False`.
- `/tmp/lkjagent-romance-chapter-test-20260630T073614Z` asked for exactly one
  700 to 900 word chapter at
  `stories/the-bell-rings-twice/manuscript/chapter-01.md`. Starting the daemon
  before enqueueing let maintenance open first; the task did not execute the
  requested chapter and only `structured-output/README.md` existed.
- `/tmp/lkjagent-romance-chapter-prequeued-20260630T074037Z` queued the same
  single-chapter request before starting the daemon. It still created
  `structured-output/README.md` with 269 words and did not create the requested
  chapter path.

## What Works

- Story-title routing preserved a stable root, `stories/bell-rings-twice`,
  instead of degrading to a generic novel root.
- `doc.audit`, `artifact.audit`, `artifact.next`, and `fs.batch_write` continued
  to advance bounded weak paths without the older same-root audit loop.
- The completion gate did not falsely close the 10,000 word story task after a
  small story-bible seed.

## What Fails

- The artifact planner fills story-bible role files before creating long
  manuscript chapter files, even when the owner explicitly asks for complete
  chapters.
- `artifact.next` weak-path batches do not yet carry a manuscript-scale cursor
  that demands chapter prose until requested word count is met.
- Direct single-chapter requests can be misclassified as counted document
  scaffolds because numeric ranges such as 700 to 900 words look like file-count
  signals.
- Negative owner instructions such as `Do not create structured-output` are not
  represented as typed constraints that can block generic counted scaffolds.
- Provider anomalies, especially `reasoning_only_response` and max-token
  completions, are recorded, but the recovery route does not force a smaller
  manuscript chunk such as one scene or one chapter subsection.
- Owner work can coexist with an already opened maintenance cycle, leaving a
  visible owner task while `active_mode=maintenance` until the next safe turn.

## Preferred Direction

Implement manuscript-scale story obligations as typed artifact facts instead of
prompt-only guidance:

- classify explicit story paths and prose word counts as story manuscript work,
  not counted documentation scaffolds;
- store requested manuscript target words, chapter count, chapter path pattern,
  and forbidden generic roots as artifact identity facts;
- make `artifact.next` choose manuscript chapter paths before optional lore
  files when the owner asks for a complete manuscript;
- make readiness require actual prose word count and chapter-file evidence for
  manuscript-scale tasks;
- split a large chapter into bounded write contracts when endpoint output hits
  max tokens or reasoning-only anomalies;
- expose status fields for manuscript target, manuscript words written,
  chapter files complete, and next manuscript path;
- add a focused live smoke that proves at least one requested chapter path is
  written before attempting the full 10,000 word route.

## Resolution

The implementation adds manuscript lifecycle facts, exact manuscript root and
path classification, counted-scaffold vetoes, chapter-priority `artifact.next`,
manuscript-only readiness word counts, completion refusal fields, provider
anomaly path preservation, and benchmark fixtures for story-bible-only and
counted-scaffold regressions.

Focused tests cover direct chapter routing, 10,000 word target facts,
count-guard non-regression, chapter-priority `artifact.next`, readiness refusal
and pass cases, completion refusal, provider anomaly shrink and blocked handoff,
and daemon absence of `structured-output` for the direct manuscript request.

## Handoff Notes

Do not reopen the completed root-identity or dense-authority work. Future smoke
can replace the historical failure evidence only when an endpoint is configured
and the owner chooses to archive new live logs.
