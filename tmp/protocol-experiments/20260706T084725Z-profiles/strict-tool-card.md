# Protocol Experiment Results

profile=strict-tool-card decision=experiment-decision envelope=Action tool_fp=fnv1a64:f0ecfa6e1e0f0717 stop=</tool_call>
- valid-tool-call parse=accept admission=admitted result=pass
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

## Rejected Ideas
- Old action envelopes stay rejected after tool-call adoption.