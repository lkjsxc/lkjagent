pub const WRITE_ACTION: &str = "<action>
<tool>fs.write</tool>
<path>out.txt</path>
<content>hello</content>
</action>";

pub const PLAN_WRITE_ACTION: &str = "<action>
<tool>graph.plan</tool>
<objective>write the file</objective>
<steps>write out.txt; read out.txt; record verification</steps>
<checks>fs.read out.txt confirms hello</checks>
<paths>out.txt</paths>
<reason>mutation requires a graph plan</reason>
</action>";

pub const PLAN_RESUME_ACTION: &str = "<action>
<tool>graph.plan</tool>
<objective>resume owner task</objective>
<steps>inspect workspace; verify safe completion</steps>
<checks>workspace summary and verification observation</checks>
<paths>.</paths>
<reason>resume after owner guidance</reason>
</action>";

pub const ASK_ACTION: &str = "<action>
<tool>agent.ask</tool>
<question>Should the output target Rust or SQLite?</question>
</action>";

pub const OWNER_QUESTION_NOTE_ACTION: &str = "<action>
<tool>graph.note</tool>
<kind>question</kind>
<summary>Should the output target Rust or SQLite?</summary>
</action>";

pub const DONE_WRITE_ACTION: &str = "<action>
<tool>agent.done</tool>
<summary>wrote file</summary>
</action>";

pub const DONE_GENERIC_ACTION: &str = "<action>
<tool>agent.done</tool>
<summary>owner task complete</summary>
</action>";

pub const READ_OUT_ACTION: &str = "<action>
<tool>fs.read</tool>
<path>out.txt</path>
</action>";

pub const EVIDENCE_WRITE_ACTION: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed hello in out.txt</summary>
<path>out.txt</path>
</action>";

pub const WORKSPACE_ACTION: &str = "<action>
<tool>workspace.summary</tool>
<path>.</path>
</action>";

pub const EVIDENCE_GENERIC_ACTION: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>workspace summary provided enough verification for this resumed task</summary>
<path>.</path>
</action>";
