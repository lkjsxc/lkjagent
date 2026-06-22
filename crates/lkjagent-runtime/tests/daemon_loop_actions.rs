pub const WRITE_ACTION: &str = "<act>
<tool>fs.write</tool>
<path>out.txt</path>
<content>hello</content>
</act>";

pub const PLAN_WRITE_ACTION: &str = "<act>
<tool>graph.plan</tool>
<objective>write the file</objective>
<steps>write out.txt; read out.txt; record verification</steps>
<checks>fs.read out.txt confirms hello</checks>
<paths>out.txt</paths>
<reason>mutation requires a graph plan</reason>
</act>";

pub const PLAN_RESUME_ACTION: &str = "<act>
<tool>graph.plan</tool>
<objective>resume owner task</objective>
<steps>inspect workspace; verify safe completion</steps>
<checks>workspace summary and verification observation</checks>
<paths>.</paths>
<reason>resume after owner guidance</reason>
</act>";

pub const ASK_ACTION: &str = "<act>
<tool>agent.ask</tool>
<question>Should the output target Rust or SQLite?</question>
</act>";

pub const OWNER_QUESTION_NOTE_ACTION: &str = "<act>
<tool>graph.note</tool>
<kind>question</kind>
<summary>Should the output target Rust or SQLite?</summary>
</act>";

pub const DONE_WRITE_ACTION: &str = "<act>
<tool>agent.done</tool>
<summary>wrote file</summary>
</act>";

pub const DONE_GENERIC_ACTION: &str = "<act>
<tool>agent.done</tool>
<summary>owner task complete</summary>
</act>";

pub const READ_OUT_ACTION: &str = "<act>
<tool>fs.read</tool>
<path>out.txt</path>
</act>";

pub const EVIDENCE_WRITE_ACTION: &str = "<act>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed hello in out.txt</summary>
<path>out.txt</path>
</act>";

pub const WORKSPACE_ACTION: &str = "<act>
<tool>workspace.summary</tool>
<path>.</path>
</act>";

pub const EVIDENCE_GENERIC_ACTION: &str = "<act>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>workspace summary provided enough verification for this resumed task</summary>
<path>.</path>
</act>";
