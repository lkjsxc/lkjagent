# Registry

## Purpose

Define the explore tools available to the model.

## Tool Set

The registry size is `tools.registry.count=10`.

| Tool | Parameters | Bound |
| --- | --- | --- |
| `fs.read` | `path`, `offset?`, `count?` | `tools.fs-read.default-lines=200` |
| `fs.list` | `path?`, `depth?` | `tools.fs-list.max-entries=200` |
| `fs.tree` | `path?`, `depth?` | `tools.fs-tree.max-entries=150` |
| `fs.search` | `query`, `path?` | `tools.fs-search.max-hits=50` |
| `fs.write` | `path`, `content` | one file inside workspace |
| `shell.run` | `command` | `tools.shell.timeout-seconds=30` |
| `memory.find` | `query` | `tools.memory-find.max-hits=10` |
| `memory.save` | `topic`, `content` | `memory.distill.words=120` |
| `plan.note` | `note` | one bounded proposal |
| `finish` | `summary` | ends explore step |

`ask` and `done` are not tools. Asking the owner belongs to the retry ladder;
completion belongs to checks.

## Tool Form

```text
<action>
<tool>fs.read</tool>
<path>data/logs/current-model-run.md</path>
<count>20</count>
</action>
```

## Failure This Prevents

The model explores with a small registry and cannot enter a broad legality maze
while scripted steps wait.
