# Protocol Experiment Results

profile=context-kernel features=prompt-cards+context-exclusion-audit decision=experiment-decision envelope=Action tool_fp=fnv1a64:633ffc357a0e01a9 stop=</tool_call>
- valid-tool-call parse=accept admission=admitted result=pass
- safe-fs-read-example parse=accept admission=admitted result=pass
- invalid-count parse=accept admission=rejected result=pass
- old-action-envelope parse=reject:WrongBlock admission=n/a result=pass
- missing-tool-name parse=reject:BadParams admission=n/a result=pass
- unknown-tool parse=reject:UnknownTool admission=n/a result=pass
- duplicate-field parse=reject:BadParams admission=n/a result=pass
- tool-name-second parse=reject:BadParams admission=n/a result=pass
- missing-required parse=reject:BadParams admission=n/a result=pass
- unknown-field parse=reject:BadParams admission=n/a result=pass
- placeholder-path parse=accept admission=rejected result=pass
- prose-outside parse=reject:WrongBlock admission=n/a result=pass
- unclosed parse=reject:Unclosed admission=n/a result=pass
- empty parse=reject:Empty admission=n/a result=pass
- workspace-escape parse=accept admission=rejected result=pass

## Decision
result=candidate next=live-trial

## Rejected Ideas
- Old action envelopes stay rejected after tool-call adoption.