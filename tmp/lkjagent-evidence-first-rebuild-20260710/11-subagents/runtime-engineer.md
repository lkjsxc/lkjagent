# Runtime Engineer

## Objective

Replace task-and-step authority with native event-reduced state.

## Work

- add matter, obligation, operation, failure, wake, progress, and completion
  domain types;
- implement reducer, transition guards, candidates, selector, and decision;
- move prompt preparation after decision persistence;
- remove special finish and synthetic idle matter;
- implement explicit waiting and quiescence;
- switch the app to native state with no production bridge reads.

## Tests

Event-only progression, transition properties, edge-blocked waiting, fake-clock
wake, readiness rejection, evidence completion, crash resume, and task-table
absence.

## Output

Small commits with docs, focused tests, source, and exact gates. Name remaining
production references to old authority after every slice.
