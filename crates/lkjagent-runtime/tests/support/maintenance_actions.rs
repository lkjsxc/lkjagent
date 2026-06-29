pub const MAINT_DONE: &str = "<action>
<tool>agent.done</tool>
<summary>maintenance cycle checked current state</summary>
</action>";

pub const WRITE_ACTION: &str = "<action>
<tool>fs.write</tool>
<path>owner.txt</path>
<content>owner wins</content>
</action>";

pub const MAINT_ASK: &str = "<action>
<tool>agent.ask</tool>
<question>Should maintenance wait?</question>
</action>";

pub const PLAN_ACTION: &str = "<action>
<tool>graph.plan</tool>
<objective>write owner file</objective>
<steps>Write owner file; read owner file; record verification evidence</steps>
<checks>fs.read owner.txt confirms content</checks>
<paths>owner.txt</paths>
<reason>owner request needs planned mutation</reason>
</action>";

pub const DONE_ACTION: &str = "<action>
<tool>agent.done</tool>
<summary>owner task complete</summary>
</action>";

pub const READ_ACTION: &str = "<action>
<tool>fs.read</tool>
<path>owner.txt</path>
</action>";

pub const EVIDENCE_ACTION: &str = "<action>
<tool>graph.evidence</tool>
<kind>verification</kind>
<summary>fs.read observed owner wins in owner.txt</summary>
<path>owner.txt</path>
</action>";
