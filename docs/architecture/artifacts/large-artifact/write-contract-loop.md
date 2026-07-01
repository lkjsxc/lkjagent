# Write Contract Loop

## Purpose

Define how the daemon chooses the next atom and admits file mutation.

## Contract Fields

| Field | Meaning |
| --- | --- |
| `contract_id` | Durable active contract identifier. |
| `artifact_id` | Artifact semantic identifier. |
| `atom_ids` | Atom rows this contract can satisfy. |
| `exact_paths` | The only paths admitted for mutation. |
| `max_files` | Maximum file count for the batch. |
| `max_file_bytes` | Per-file byte budget. |
| `max_batch_bytes` | Total batch byte budget. |
| `target_count` | Desired count for the selected atom. |
| `count_floor` | Minimum accepted count for the selected atom. |
| `required_sections` | Required headings or semantic elements. |
| `continuity_digest` | Bounded continuity facts for this atom only. |
| `forbidden_weak_classes` | Weak classes that make the atom fail audit. |

## Selection

`artifact.next` loads the active plan, returns the existing active contract when
one exists, otherwise selects the first eligible non-ready atom by dependency
order. If no writable atom remains, it returns assembly or readiness candidates.

## Admission

`fs.batch_write` validates every path against the active contract before any
mutation. A path under a known artifact root is refused when no active contract
matches it. A path outside the contract, a completed contract path, a file count
above `max_files`, a file above `max_file_bytes`, or a batch above
`max_batch_bytes` is refused. The prompt example names exact paths and the line
protocol; it never prefills body prose.

## Status

implemented.
