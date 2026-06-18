# Hygiene

## Purpose

The allowlist of everything that may ever enter the context window. The
window stays clean because admission is enumerated, budgeted, and owned;
anything not listed here is forbidden by default.

## The Allowlist

| Frame | Admitted when | Budget owner |
| --- | --- | --- |
| prefix sections | compaction or restart only | [budgets.md](budgets.md) |
| owner frame | queue delivery at a turn boundary | [../runtime/queue-intake.md](../runtime/queue-intake.md) |
| model turn | every completion, verbatim | reserve in [budgets.md](budgets.md) |
| observation | exactly one per executed action | [../tools/registry.md](../tools/registry.md) |
| notice | enumerated kinds below | harness |
| skill body | explicit skill.use only | [../skills/loading.md](../skills/loading.md) |

## Notice Kinds

Notices are the only free-form harness voice, and even they are enumerated:

| Kind | Carries |
| --- | --- |
| truncation | what was cut and the retrieval path for the rest |
| budget | task turn budget exhausted; lawful next actions |
| error | parse, tool, or endpoint failure in one bounded line set |
| compaction | the log-head task summary after a rebuild |
| maintenance | cycle directive per [../runtime/self-maintenance.md](../runtime/self-maintenance.md) |

## Explicitly Forbidden

- Raw logs, build spew, or any unbounded command output: shell observations
  are head-and-tail capped per [../tools/shell.md](../tools/shell.md).
- Echoing file content on writes: an fs.write observation confirms path and
  byte count, never the content the model already has.
- Tool schemas beyond the one registry block in the prefix.
- Timestamps, counters, or any per-turn varying decoration: variance poisons
  the cache and feeds the model noise.
- Duplicate delivery of anything already in the window: the harness tracks
  what was sent; repeated fs.read of an unchanged file returns a notice
  pointing at the earlier observation instead of the bytes.
- Apologies, hedges, or commentary from the harness: notices state facts.

## Pressure Toward Narrowness

Hygiene is also an incentive design: because observations are capped and
duplicates are refused, the cheapest path for the model is ranged reads,
filtered shell commands, and memory.find before re-reading. The rules make
the lazy path and the clean path the same path.

## Status

implemented.
