# Registry

## Purpose

The canonical table of every tool: parameters, contracts, and primary error
cases. The dispatcher validates incoming actions against this table, and
the prompt builder renders the registry section of the system prompt from
the same table, so prompt and behavior cannot drift. Other tool documents
link here; this file is the single source.

## The Table

Parameters are marked req or opt; a default follows opt where one exists.

| Tool | Parameters | Contract | Primary errors |
| --- | --- | --- | --- |
| fs.read | path req; start opt 1; count opt 200 | ranged raw read, one header line | missing file; duplicate read |
| fs.write | path req; content req | write file, create parent directories | write failure |
| fs.edit | path req; find req; replace req | replace exactly one match of find | zero or many matches, count reported |
| shell.run | command req; timeout opt 60, max 600 | run /bin/sh -lc in the workspace | timeout; spawn failure |
| queue.list | status opt all; limit opt 20 | list queue rows by id, status, source, and preview | unknown status |
| queue.enqueue | content req; reason req | append a pending queue row | empty content |
| queue.edit | id req; content req; reason req | replace content of a pending queue row | missing row; non-pending |
| queue.delete | id req; reason req | tombstone a pending queue row | missing row; non-pending |
| queue.redeliver | id req; reason req; content opt | create a pending row linked to a source row | missing row |
| memory.save | kind req; title req; tags opt; content req | insert one memory row, return its id | unknown kind |
| memory.find | query req; limit opt 5 | ranked full-text search over memory | none; empty results are ok |
| skill.use | name req | append skill body as immutable frame | unknown name; already loaded; skill budget |
| skill.save | name req; content req | validate and write skill into library | format validation failure |
| agent.done | summary req | close the task or maintenance cycle | no open task or cycle |
| agent.ask | question req | ask the owner; task enters waiting | a question is already outstanding |

Detailed contracts: [fs.md](fs.md), [shell.md](shell.md),
[queue-ops.md](queue-ops.md), [memory-ops.md](memory-ops.md),
[skill-ops.md](skill-ops.md), [control.md](control.md).

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
