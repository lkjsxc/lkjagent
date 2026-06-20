# Registry

## Purpose

The documented table of every tool: parameters, contracts, and primary error
cases. The executable registry in `lkjagent-protocol` validates incoming
actions and renders the system prompt from the same Rust table, so prompt and
behavior cannot drift. Other tool documents link here.

## The Table

Parameters are marked req or opt; a default follows opt where one exists.

| Tool | Parameters | Contract | Primary errors |
| --- | --- | --- | --- |
| fs.read | path req; start opt 1; count opt 200 | ranged raw read, one header line | missing file; duplicate read |
| fs.read_many | paths req; start opt 1; count opt 80; total opt 400 | bounded multi-file ranged read | empty paths; workspace escape; total cap |
| fs.write | path req; content req | write file, create parent directories | write failure |
| fs.edit | path req; find req; replace req | replace exactly one match of find | zero or many matches, count reported |
| fs.patch | path req; patch req | apply exact find/replace edit blocks | zero or many matches; too many edits |
| fs.list | path opt .; depth opt 2; kind opt all; limit opt 200 | sorted bounded workspace listing | workspace escape; invalid depth or limit |
| fs.tree | path opt .; depth opt 3; limit opt 200 | sorted bounded tree output | workspace escape; invalid depth or limit |
| fs.search | query req; path opt .; include opt; case opt insensitive; context opt 1; limit opt 50 | bounded substring search | workspace escape; invalid limit |
| fs.stat | path req | kind, bytes, lines, stable checksum | missing path; workspace escape |
| fs.mkdir | path req | create a workspace directory | workspace escape; create failure |
| fs.batch_write | files req | write several files from line protocol | duplicate path; empty path; workspace escape |
| shell.run | command req; timeout opt 60, max 600 | escape hatch /bin/sh -lc in the workspace | graph policy refusal; non-zero exit; timeout |
| queue.list | status opt all; limit opt 20 | list queue rows by id, status, source, and preview | unknown status |
| queue.enqueue | content req; reason req | append a pending queue row | empty content |
| queue.edit | id req; content req; reason req | replace content of a pending queue row | missing row; non-pending |
| queue.delete | id req; reason req | tombstone a pending queue row | missing row; non-pending |
| queue.redeliver | id req; reason req; content opt | create a pending row linked to a source row | missing row |
| memory.save | kind req; title req; tags opt; content req | insert one memory row, return its id | unknown kind |
| memory.find | query req; limit opt 5 | ranked full-text search over memory | none; empty results are ok |
| memory.prune | none | delete exact duplicate memory rows | store delete failure |
| graph.state | none | show active graph case, phase, node, evidence, and transitions | no active case |
| graph.next | none | show legal transitions, missing guards, and preferred next action | no active case |
| graph.audit | none | audit active graph case, policy, completion, and shell admission | no active case |
| graph.recover | none | inspect recovery ladder and alternate action guidance | no active case |
| graph.plan | objective req; constraints opt; assumptions opt; risks opt; steps req; checks opt; paths opt; reason req | record structured plan | empty objective; no steps; no checks or paths |
| graph.transition | target req; reason req | request guarded transition | illegal target; missing guard |
| graph.context | packages req; reason req | select context packages | unknown or empty package list |
| graph.note | kind req; summary req; path opt | record non-evidence graph state | unknown kind; empty summary |
| graph.evidence | kind req; summary req; path opt | record explicit evidence on active case | no active case; empty summary |
| graph.compact | reason req | request graph compaction checkpoint | policy refusal |
| workspace.summary | path opt .; depth opt 3; limit opt 200 | bounded repository shape map | workspace escape; invalid limit |
| workspace.index | path opt .; depth opt 3; limit opt 200 | compact repository index with readmes and manifests | workspace escape; invalid limit |
| verify.cargo | gate req; package opt; timeout opt 120 | run direct cargo gate | unknown gate; timeout; command failure |
| verify.xtask | gate req; timeout opt 120 | run direct xtask gate | unknown gate; timeout; command failure |
| doc.scaffold | root req; kind opt documentation; count opt; mode opt approx; title req; sections opt | create compact README-indexed document tree | workspace escape; invalid root |
| doc.audit | root req; count opt; mode opt approx | audit README and document topology | missing README; count mismatch |
| agent.done | summary req | close the task or maintenance cycle | no open task or cycle |
| agent.ask | question req | ask the owner; task enters waiting | a question is already outstanding |

Detailed contracts: [fs.md](fs.md), [shell.md](shell.md),
[queue-ops.md](queue-ops.md), [memory-ops.md](memory-ops.md),
[graph-ops.md](graph-ops.md), [workspace.md](workspace.md),
[verification-tools.md](verification-tools.md), [doc-tools.md](doc-tools.md),
[control.md](control.md).

## Dispatch

The dispatcher processes each act block in a fixed order:

1. Parse: the act block is parsed per
   [../protocol/action-format.md](../protocol/action-format.md); grammar
   faults follow [../protocol/recovery.md](../protocol/recovery.md).
2. Registry validation: the tool name and every parameter are checked
   against the table above; unknown names, missing required parameters,
   and unknown or duplicate parameters produce one error notice listing
   every offender at once.
3. Execute: the validated action runs and yields exactly one observation.
   Queue mutation tools call lkjagent-store, which writes the queue row and
   the queue_mutation transcript event in one transaction per
   [../memory/store.md](../memory/store.md).

## Observation Frame

Every executed action yields exactly one observation frame appended as a
user message:

```
<observation>
<status>ok</status>
<content>
...
</content>
</observation>
```

status is ok or error. Observations are capped at 2,048 tokens each per
[../context/budgets.md](../context/budgets.md); truncation keeps head and
tail with a truncation notice naming the retrieval path (a ranged fs.read,
a narrower shell command, a memory.find query).

## Single Source Rule

The table above is the only definition of the toolset. Validation in the
dispatcher and the registry section of the system prompt are both generated
from it; there is no second copy to fall out of step.

## Status

implemented.
