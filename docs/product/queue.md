# Queue

## Purpose

Define how owner turns enter the system and how semantic routing preserves
conversation continuity.

## Owner Turn Intake

`lkjagent send TEXT` inserts one pending owner-turn row. It does not require a
running daemon. The daemon delivers pending rows only at cycle start, so owner
input never interrupts an endpoint call or state transaction.

A turn stores the raw owner text, received time, routing flags, optional
preferred matter ref, route lane, desired durability, transformation permission,
and delivery state. The row is evidence, not the visible unit of work. The
visible unit is the transcript, inbox trace, record, artifact, matter, or proof
path linked from the row.

## Semantic Routing

The deterministic router chooses one outcome before asking the model whenever
rules are sufficient:

- answer a waiting matter;
- append to an existing matter;
- create or update a workspace record;
- create an artifact request;
- inspect or retrieve workspace evidence;
- request a system operation;
- create a new matter.

Record-like phrases such as "record this", Japanese diary requests, todo-like
text, calendar-like text, finance notes, and project notes prefer workspace
write-through over matter creation. Ambiguous save-like text writes an inbox
trace with the route reason or asks one clarification instead of disappearing.

## Answer Routing

If a matter is waiting and the owner turn is not marked as separate, the row is
linked as the answer. The daemon records an answer event, a relation edge from
the question to the answer, and a state patch that makes the matter runnable.

`lkjagent send --new TEXT` marks the turn as separate. It bypasses answer
routing and is selected by the semantic router in FIFO order with other
matter-opening turns.

## Ordering

The daemon selects one persisted decision at a time. It does not interleave
endpoint calls from two active matters. Owner turns retain FIFO order inside
routing groups, but direct record writes may complete without waiting for a
model-dependent matter.

## Visibility

`lkjagent queue list` shows pending, delivered, answered, recorded, separate,
and failed-routing rows with route lane, desired durability, and transformation
permission. Matter, transcript, inbox, artifact, and record views link back to the owner
turn and show the workspace paths, fingerprints, decision refs, or blocker
reasons produced by routing.
