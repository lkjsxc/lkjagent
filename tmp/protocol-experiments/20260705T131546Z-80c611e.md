# Protocol Experiment Results

- current-tool-call expected=accept actual=accept result=pass
- old-action-envelope expected=reject actual=reject:WrongBlock result=pass
- old-tool-field expected=reject actual=reject:BadParams result=pass

## Rejected Ideas
- Old action envelopes stay rejected after tool-call adoption.