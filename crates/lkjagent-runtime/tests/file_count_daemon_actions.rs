pub const PLAN_BATCH: &str = "<action>
<tool>graph.plan</tool>
<objective>create a five-file package</objective>
<steps>write batch; list files; record verification</steps>
<checks>fs.list confirms deliverable files</checks>
<paths>deliverable</paths>
<reason>exact file-count task needs planned batch creation</reason>
</action>";

pub const BATCH_WRITE: &str = "<action>
<tool>fs.batch_write</tool>
<files>
path: deliverable/README.md
content:
# Deliverable
-- lkjagent-next-file --
path: deliverable/docs/plan.md
content:
# Doc
-- lkjagent-next-file --
path: deliverable/main/part-001.md
content:
# Main 1
-- lkjagent-next-file --
path: deliverable/main/part-002.md
content:
# Main 2
-- lkjagent-next-file --
path: deliverable/main/part-003.md
content:
# Main 3
</files>
</action>";

pub const LIST_DELIVERABLE: &str = "<action>
<tool>fs.list</tool>
<path>deliverable</path>
<kind>file</kind>
</action>";

pub const VERIFY_DELIVERABLE: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.list observed five deliverable files</summary>
<path>deliverable</path>
</action>";

pub const DONE: &str = "<action>
<tool>agent.done</tool>
<summary>created a README-indexed five-file deliverable</summary>
</action>";
