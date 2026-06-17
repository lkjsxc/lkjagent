# File Tools

## Purpose

The contracts for fs.read, fs.write, and fs.edit: direct file access inside
the workspace. The three tools cover ranged reading, whole-file writing,
and single-match editing; listing, searching, and bulk transforms run
through [shell.md](shell.md). Canonical parameter table:
[registry.md](registry.md).

## fs.read

| Parameter | Rule |
| --- | --- |
| path | required |
| start | optional line number, default 1 |
| count | optional line count, default 200 |

Returns raw file content for the range, prefaced by one header line
stating the path, the range returned, and the total line count. Content
lines carry no line-number prefixes: the bytes are exact, so a span can be
pasted into an fs.edit find string without cleanup.

Reading an unchanged file region already in the window is refused with a
notice pointing at the earlier observation, per the duplicate rule in
[../context/hygiene.md](../context/hygiene.md).

## fs.write

| Parameter | Rule |
| --- | --- |
| path | required |
| content | required |

Writes the file, creating parent directories as needed. The observation
confirms the path and byte count only; it never echoes content, because
the model already holds the bytes it just wrote
([../context/hygiene.md](../context/hygiene.md)).

## fs.edit

| Parameter | Rule |
| --- | --- |
| path | required |
| find | required, must match exactly once |
| replace | required |

If find matches zero times or more than once, the tool errors and reports
the match count; nothing is written. On success the observation states the
path and the line number of the replacement.

Example, in the action format of
[../protocol/action-format.md](../protocol/action-format.md):

```
<act>
<tool>fs.edit</tool>
<path>crates/lkjagent-protocol/src/render.rs</path>
<find>
    out.push_str(block);
</find>
<replace>
    out.push_str(block);
    out.push('\n');
</replace>
</act>
```

## Maintenance Behavior

During a maintenance cycle, fs.write and fs.edit have the same workspace
authority they have during task work. The boundary is the container blast
radius in [../sandbox/safety.md](../sandbox/safety.md), not a
maintenance-specific restriction.

## Status

design-only.
