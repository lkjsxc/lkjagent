# Story Profile

## Purpose

Define the creative writing scaffold profile for story, novel, manuscript,
character, setting, outline, and scene artifacts.

## Example

```text
stories/<semantic-title>/
  README.md
  catalog.toml
  request/
    README.md
    objective.md
    constraints.md
  project/
    README.md
    premise.md
    themes.md
  setting/
    README.md
    cosmology.md
    timeline.md
    locations.md
    society.md
    factions.md
    technology.md
  characters/
    README.md
    protagonist.md
    antagonist.md
    supporting-cast.md
    relationships.md
  plot/
    README.md
    conflict-lattice.md
    act-structure.md
    chapter-spine.md
  continuity/
    README.md
    rules.md
    glossary.md
    open-questions.md
  style/
    README.md
    tone.md
    motifs.md
  manuscript/
    README.md
    draft-boundary.md
    scene-seeds.md
  relations/
    README.md
    character-plot.md
    setting-plot.md
  checks/
    README.md
    structure-audit.md
    readiness-audit.md
```

## Selection

The creative profile is selected when `kind`, title, or a `stories/` artifact
root names story, novel, manuscript, fiction, character-profile, setting,
outline, scene, SF, sci-fi, or science fiction, unless the request is clearly
for technical documentation.

## Lifecycles

Story work uses the lifecycle contract in
[manuscript-lifecycle.md](manuscript-lifecycle.md). `story-bible` files are
reference material. `manuscript` files are finished chapter or scene prose under
`manuscript/`. `story-bible-then-manuscript` allows identity and reference seed
files, but requested manuscript paths outrank optional lore after root identity
exists.

Exact `stories/.../manuscript/*.md` paths are preserved as owner targets. Word
counts next to `word` or `words` are prose targets, not file-count requests.
Chapter ordinals such as `chapter-01.md` are path ordinals, not counted-document
scaffold signals.

## Audit

A story-bible audit requires premise, setting or world rules, timeline or
continuity rules, cast, relationships or conflict, plot or act structure, style
or tone, manuscript boundary, README links, and catalog metadata. Catalog
metadata may come from the story profile or from a handwritten catalog that
states `kind = "story"` or describes a story bible. It refuses a full manuscript
when the owner asked for a bible only. Concise reference pages are content
when they carry headed story facts and verification notes. Scaffold-only
creative labels, bracket placeholders, README-only trees, and owner-term-only
pages are not content evidence.

Long-novel story-bible readiness also requires profile-scale content groups. A
compact seed page is refused with `story_scale_missing` facts such as
`profile-scale-content-groups` or `profile-scale-word-count` until enough
separate story-bible groups exist. Live repair uses bounded flat files such as
`act-structure.md`, `cosmology.md`, and `completion-evidence.md`; the path label
counts as the semantic role when the file has enough concrete words.

Manuscript readiness counts only finished prose under `manuscript/`. It reports
`manuscript_word_count`, `manuscript_target_words`,
`missing_manuscript_paths`, and `next_manuscript_path`; story-bible files do not
satisfy those facts.

## Status

implemented through story root selection, bounded story repair batches,
structure-only content checks, and artifact audit readiness.
