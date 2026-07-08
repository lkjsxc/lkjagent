# Protocol Experiment Results

profile=artifact-manifest features=manifest+nested-units decision=experiment-decision envelope=Action tool_fp=fnv1a64:633ffc357a0e01a9 stop=</lkjagent_action>
- valid-finish parse=accept admission=admitted result=pass
- safe-fs-read parse=accept admission=admitted result=pass
- invalid-count parse=reject:Action(ArgsSchemaViolation("wrong primitive for count")) admission=n/a result=pass
- old-tool-call parse=reject:Action(NoActionFound) admission=n/a result=pass
- old-action-envelope parse=reject:Action(NoActionFound) admission=n/a result=pass
- missing-required parse=reject:Action(ArgsSchemaViolation("missing arg summary")) admission=n/a result=pass
- unknown-tool parse=reject:UnknownTool admission=n/a result=pass
- duplicate-field parse=reject:Action(DuplicateTag("argument/path")) admission=n/a result=pass
- unknown-field parse=reject:Action(ArgsSchemaViolation("unknown arg extra")) admission=n/a result=pass
- placeholder-path parse=accept admission=rejected result=pass
- prose-outside parse=reject:Action(EnvelopeMalformed) admission=n/a result=pass
- unclosed parse=reject:Action(EnvelopeMalformed) admission=n/a result=pass
- empty parse=reject:Action(ArgsSchemaViolation("missing decision_id")) admission=n/a result=pass
- workspace-escape parse=accept admission=rejected result=pass

## Decision
result=deferred next=live

## Rejected Ideas
- Old action envelopes stay rejected after tool-call adoption.