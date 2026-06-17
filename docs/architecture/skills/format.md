# Format

## Purpose

The canonical skill shape. One format serves the running agent and the
coding agents that build lkjagent; the builders' instances live under
[../../agent/skills/](../../agent/skills/README.md). skill.save and the
repository checks enforce the same rules.

## Shape

```
# Skill: <Name>

## Purpose
One paragraph: the capability this skill grants and when it pays off.

## Trigger
One sentence. This line is the skill's entry in the prefix index, so it
must let a model decide relevance without loading the body.

## Context
What to read, load, or check before acting: files, memory queries, prior
observations. Bullets, each with a reason.

## Procedure
Numbered steps in imperative voice. Steps name exact commands, exact paths,
and exact tools. A step that cannot fail is decoration; remove it.

## Checks
How to verify the procedure worked: commands and the exact evidence of
success. A skill without checks teaches guessing.

## Must Not
The failure modes this skill exists to prevent, as prohibitions.
```

An optional final section `## Handoff` is allowed when the skill produces
work someone else continues (builder skills use it; runtime skills rarely
need it).

## Rules

- Filename is the kebab-case skill name plus .md; the H1 carries the
  display name.
- At most 120 lines, ASCII, prose lines at most 120 characters.
- Exactly the headings above, in that order; no extra headings besides the
  optional Handoff.
- No YAML frontmatter. Structure lives in headings the model already reads.
- Links and commands must be real: a skill naming a path that does not
  exist fails validation where the path is checkable.
- One capability per skill. A skill teaching two things becomes two skills.

## Why This Shape

Trigger feeds the index ([loading.md](loading.md)) so relevance costs one
line. Context fights the duplicate-read rule by telling the model what is
already known. Checks encode the honesty principle: a skill is not done
when the procedure ran, it is done when the evidence appeared. Must Not
carries the scars of past failures forward without retelling the stories.

## Status

implemented.
