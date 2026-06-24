# lkjagent Model Run Log

## Snapshot

- created_at: 1782286341
- daemon_state: working
- queue_depth: 0
- active_case: 1
- active_node: recover-by-alternate-tool
- active_phase: recovery
- context: 7.96K/24.58K 32.39% prefix=5.38K log=17.15K reserve=2.05K headroom=16.61K
- token_usage: in=6.73K out=117 cache=unknown total=6.85K

## Owner Objective

Raw:

```text
Create docs of minecraft beginner's guide.
```

Normalized:

```text
Deliver the owner task by improving semantic recursive documentation.
```

## Constraints And Preferences

* evidence: plan
* evidence: observation
* evidence: document-structure
* checks: document audit
* packages: planning-checklist
* packages: context-slice
* packages: doc-construction

## Active State Tracks

| rank | posture | label | intensity | confidence | phase | evidence gap |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Recovering | parse-recovery | 90 | 70 | recovery | recovery evidence |
| 2 | Structuring | document-structure | 88 | 70 | planning | document audit |
| 3 | Recovering | action-param-reliability | 61 | 60 | recovery | normalizer tests |
| 4 | Exploring | observability-ledger | 48 | 55 | planning | status evidence |

## Plan

| step | status | target paths | evidence | checks |
| --- | --- | --- | --- | --- |
| active | active | recover-by-alternate-tool | pending | document audit |

## Touched Paths

* none

## Evidence Ledger

| kind | requirement | summary | path | confidence |
| --- | --- | --- | --- | --- |
| none | none | none | none | low |

## Fault Ledger

| turn | kind | message | recovery |
| --- | --- | --- | --- |
| 9 | error | parse fault: missing act block | recover-by-alternate-tool |
| 9 | notice | recovery: parse faults are consecutive count=4; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked | recover-by-alternate-tool |
| 9 | notice | Consecutive parse faults reached count=4; graph recovery is active. Use graph.recover, reduce scope, choose an alternate native tool, or replan around the blocked step. | recover-by-alternate-tool |
| 10 | error | parse fault: missing act block | recover-by-alternate-tool |
| 10 | notice | recovery: parse faults are consecutive count=5; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked | recover-by-alternate-tool |
| 10 | notice | Consecutive parse faults reached count=5; graph recovery is active. Use graph.recover, reduce scope, choose an alternate native tool, or replan around the blocked step. | recover-by-alternate-tool |
| 12 | error | parse fault: missing act block | recover-by-alternate-tool |
| 13 | error | parse fault: missing act block | recover-by-alternate-tool |

## Recent Transcript

| id | turn | kind | preview |
| --- | --- | --- | --- |
| 1 | none | notice | maintenance cycle opened<br>directive=distill<br>turn_budget=8<br>work=read recent transcript spans and save durable lessons |
| 2 | 1 | action | <act><br><tool>memory.find</tool><br><query>maintenance</query><br><limit>10</limit><br></act> |
| 3 | 1 | observation | <observation><br><status>ok</status><br><content>no memory results</content><br></observation> |
| 4 | 2 | error | parse fault: missing act block |
| 5 | 2 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |
| 6 | none | queue_mutation | operation=enqueue<br>reason=console-send<br>target_id=1<br>source_queue_id=null<br>before=<br>after=Create docs of minecraft beginner's guide. |
| 7 | 3 | error | parse fault: missing act block |
| 8 | 3 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |
| 9 | 4 | owner | Create docs of minecraft beginner's guide. |
| 10 | 4 | error | parse fault: missing act block |
| 11 | 4 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |
| 12 | 5 | action | <act><br><tool>graph.recover</tool><br></act> |
| 13 | 5 | observation | <observation><br><status>ok</status><br><content><br>node=plan<br>phase=planning<br>recovery_ladder=inspect-state,smaller-scope,alternate-native-tool,replan,admitted-shell-escape,block-step-and-continue<br>repeat_count=0<br>next=use graph.plan, fs.list, fs.tree, fs.search, graph.state<br></content><br></observation> |
| 14 | 6 | error | parse fault: missing act block |
| 15 | 6 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |
| 16 | 7 | error | parse fault: missing act block |
| 17 | 7 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |
| 18 | 8 | error | parse fault: missing act block |
| 19 | 8 | notice | recovery: parse faults are consecutive count=3; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked |
| 20 | 8 | notice | Consecutive parse faults reached count=3; graph recovery is active. Use graph.recover, reduce scope, choose an alternate native tool, or replan around the blocked step. |
| 21 | 9 | error | parse fault: missing act block |
| 22 | 9 | notice | recovery: parse faults are consecutive count=4; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked |
| 23 | 9 | notice | Consecutive parse faults reached count=4; graph recovery is active. Use graph.recover, reduce scope, choose an alternate native tool, or replan around the blocked step. |
| 24 | 10 | error | parse fault: missing act block |
| 25 | 10 | notice | recovery: parse faults are consecutive count=5; simplify to one valid act block; prefer typed file/doc tools for large payloads; ask only if blocked |
| 26 | 10 | notice | Consecutive parse faults reached count=5; graph recovery is active. Use graph.recover, reduce scope, choose an alternate native tool, or replan around the blocked step. |
| 27 | 11 | action | <act><br><tool>graph.recover</tool><br></act> |
| 28 | 11 | notice | <notice><br><kind>error</kind><br><content><br>repeat action refused; see frame 2<br>active_mode=Recovery<br>next_action_must_change_shape=true<br>forbidden_tool=graph.recover<br>preferred_next_action=graph.state<br>valid_example:<br><act><br><tool>graph.state</tool><br></act><br></content><br></notice> |
| 29 | 11 | notice | recovery: repeated action was refused; change the next action instead of resending it |
| 30 | 12 | error | parse fault: missing act block |
| 31 | 12 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |
| 32 | 13 | error | parse fault: missing act block |
| 33 | 13 | notice | recovery: the previous completion was not executed; emit exactly one valid act block next |

## Verification

| command | result | notes |
| --- | --- | --- |
| document audit | pending | graph case check |

