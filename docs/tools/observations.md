# Observations

## Purpose

Define the bounded observation returned after an admitted tool or effect.

## Shape

Every observation records decision id, admission id when present, tool or effect
name, status, bounded content, artifact refs, source fingerprints, created time,
and contamination class. Prompt text renders only the bounded content and compact
source refs selected by the context frame.

```text
<observation>
<status>ok</status>
<content>
...bounded text...
</content>
</observation>
```

The content cap is `tools.observation.max-tokens=1500`. Large output is
head-and-tail truncated with `context.truncation.marker=[...]`. Raw large output
may live in an artifact or exchange file, but the durable observation owns the
bounded resumable fact.

## Example

```text
<tool_call>
<tool_name>fs.read</tool_name>
<path>data/logs/current-model-run.md</path>
<count>20</count>
</tool_call>
```

Observation:

```text
<observation>
<status>ok</status>
<content>
Historical failure fixture for the manuscript run. The active case is in
recovery, the observed root is stories/novel-named, and final verification is
pending.
</content>
</observation>
```

## Prompt Rule

Only current relevant observations enter normal prompts. Old observations remain
source evidence and may be summarized as clean context items or excluded as
stale, superseded, raw-tool-log, or recovery-only material.

## Failure This Prevents

Tool output cannot grow into a transcript. The model sees current evidence, not
every previous refusal or raw dump.
