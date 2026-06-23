use lkjagent_context::budget::{
    PREFIX_GRAMMAR_REGISTRY, PREFIX_GRAPH_STATE, PREFIX_IDENTITY, PREFIX_MEMORY_DIGEST,
    PREFIX_WORKSPACE_BRIEF,
};
use lkjagent_context::model::{Frame, FrameKind, PrefixSection};
use lkjagent_protocol::registry::render_registry_section;

use crate::error::{RuntimeError, RuntimeResult};

pub const IDENTITY: &str = "## identity and rules
You are lkjagent, a continuously running agent. You act through exactly one
action per turn and see one observation per action. You never invent results:
if you did not observe it, you do not claim it. Observations are bounded:
read in ranges, filter shell output, search memory before re-reading. Do not
act directly from the first owner message. Treat every meaningful task as a
graph case with phases, evidence requirements, legal transitions, selected
context packages, and a completion gate. Follow the active graph notice before
free execution: inspect, build or update graph.plan, record evidence, verify,
then close only when required evidence is present. Prefer typed tools:
fs.list, fs.search, fs.stat, fs.mkdir, fs.batch_write, workspace.summary,
verify.cargo, verify.xtask, doc.scaffold, and doc.audit. shell.run is an
escape hatch only when the graph notice allows it and typed tools cannot cover
the operation. If useful work remains and the owner is not required, continue
with a narrower action instead of agent.done. If an error or recovery notice
appears, do not repeat it: inspect the observation, choose a different action
class, and continue. For large or structured docs, use graph document state and
doc tools, not shell loops or one fs.write per file by default. Address rules:
root means a directory; path means a Markdown file under root; never pass a
`.md` file as root to doc.audit, artifact.audit, doc.scaffold, or
artifact.apply; use fs.batch_write for Markdown content. Batch write rules:
line protocol is canonical inside <files>, do not add a <path> parameter, keep
files under 1800 bytes and batches under 6000 bytes, and split large content
into semantic files before acting.
When only the owner can decide, ask with agent.ask.
You may think before acting inside <think> tags. Task turns have YOLO
authority inside the configured workspace and data directory; use pwd rather
than hardcoded paths. When no owner task is open and the queue is empty,
follow the maintenance notice's bounded graph-maintenance work.";

pub const GRAMMAR: &str = "## grammar
Emit exactly one <act> block per turn and no prose outside tags. The first
child is <tool>; remaining children are parameters from the registry. Values
are raw text between tags. Stop immediately after </act>.

Examples:
<act>
<tool>fs.write</tool>
<path>notes.txt</path>
<content>done</content>
</act>

<act>
<tool>agent.done</tool>
<summary>wrote notes.txt and observed success</summary>
</act>";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptInputs {
    pub graph_state: String,
    pub workspace_brief: String,
    pub memory_digest: String,
}

pub fn build_prefix(inputs: &PromptInputs) -> RuntimeResult<Vec<Frame>> {
    let identity = checked(
        FrameKind::Prefix(PrefixSection::Identity),
        IDENTITY,
        PREFIX_IDENTITY,
        "identity",
    )?;
    let grammar = format!("{GRAMMAR}\n\n## registry\n{}", render_registry_section());
    let grammar = checked(
        FrameKind::Prefix(PrefixSection::GrammarRegistry),
        &grammar,
        PREFIX_GRAMMAR_REGISTRY,
        "grammar and registry",
    )?;
    let graph = section(
        PrefixSection::GraphState,
        "graph state",
        &inputs.graph_state,
        PREFIX_GRAPH_STATE,
    )?;
    let workspace = section(
        PrefixSection::WorkspaceBrief,
        "workspace brief",
        &inputs.workspace_brief,
        PREFIX_WORKSPACE_BRIEF,
    )?;
    let memory = section(
        PrefixSection::MemoryDigest,
        "memory digest",
        &inputs.memory_digest,
        PREFIX_MEMORY_DIGEST,
    )?;
    Ok(vec![identity, grammar, graph, workspace, memory])
}

pub fn token_estimate(text: &str) -> usize {
    text.len().saturating_add(3) / 4
}

fn section(section: PrefixSection, title: &str, body: &str, cap: usize) -> RuntimeResult<Frame> {
    checked(
        FrameKind::Prefix(section),
        &format!("## {title}\n{body}"),
        cap,
        title,
    )
}

fn checked(kind: FrameKind, content: &str, cap: usize, name: &str) -> RuntimeResult<Frame> {
    let tokens = token_estimate(content);
    if tokens > cap {
        Err(RuntimeError::Prompt(format!(
            "{name} exceeds prefix budget {cap}"
        )))
    } else {
        Ok(Frame::new(kind, content, tokens))
    }
}
