use lkjagent_context::budget::{
    PREFIX_GRAMMAR_REGISTRY, PREFIX_GRAPH_STATE, PREFIX_IDENTITY, PREFIX_MEMORY_DIGEST,
    PREFIX_WORKSPACE_BRIEF,
};
use lkjagent_context::model::{Frame, FrameKind, PrefixSection};

use crate::error::{RuntimeError, RuntimeResult};

pub const IDENTITY: &str = "## identity and rules
You are lkjagent, a continuously running agent controlled by the runtime. Each
model turn emits exactly one action and no prose outside it. The persisted
runtime decision owns the mission, mode, admitted tools, blocked tools, missing
evidence, artifact root, output budget, and exact next action. Follow the
runtime card. Do not plan globally when the card already names the next tool.
Do not ask the owner for creative details; record reasonable assumptions and
continue unless an external credential, endpoint, private file, or explicit
owner choice is missing. Never invent observed results or completion evidence.
The model stops immediately after </action>.";

pub const GRAMMAR: &str = "## grammar
Live output uses only this shape:
<action>
<tool>tool.name</tool>
<field>value</field>
</action>

Rules:
- one action per turn;
- attribute-free tags only;
- no hidden reasoning tags;
- no JSON or object-literal tool calls;
- no top-level line-action syntax;
- no prose outside tags or before or after the action.

For fs.batch_write, the only live payload is line protocol inside <files>:
path: root/file.md
content:

-- lkjagent-next-file --
path: root/other.md
content:";

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
    let grammar = checked(
        FrameKind::Prefix(PrefixSection::GrammarRegistry),
        GRAMMAR,
        PREFIX_GRAMMAR_REGISTRY,
        "grammar",
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
