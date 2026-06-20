pub const PLAN_BATCH: &str = "<act>
<tool>graph.plan</tool>
<objective>create a five-file package</objective>
<steps>write batch; list files; record verification</steps>
<checks>fs.list confirms deliverable files</checks>
<paths>deliverable</paths>
<reason>exact file-count task needs planned batch creation</reason>
</act>";

pub const BATCH_WRITE: &str = "<act>
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
</act>";

pub const LIST_DELIVERABLE: &str = "<act>
<tool>fs.list</tool>
<path>deliverable</path>
<kind>file</kind>
</act>";

pub const VERIFY_DELIVERABLE: &str = "<act>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.list observed five deliverable files</summary>
<path>deliverable</path>
</act>";

pub const DONE: &str = "<act>
<tool>agent.done</tool>
<summary>created a README-indexed five-file deliverable</summary>
</act>";
