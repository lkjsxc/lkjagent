use lkjagent_context::budget::{
    PREFIX_GRAMMAR_REGISTRY, PREFIX_IDENTITY, PREFIX_MEMORY_DIGEST, PREFIX_SKILL_INDEX,
    PREFIX_WORKSPACE_BRIEF,
};
use lkjagent_context::model::{Frame, FrameKind, PrefixSection};
use lkjagent_protocol::registry::render_registry_section;

use crate::error::{RuntimeError, RuntimeResult};

pub const IDENTITY: &str = "## identity and rules
You are lkjagent, a continuously running agent. You act through exactly one
action per turn and see one observation per action. You never invent results:
if you did not observe it, you do not claim it. Observations are bounded:
read in ranges, filter shell output, search memory before re-reading. When a
task completes, finish with agent.done and an honest summary. When only the
owner can decide, ask with agent.ask. You may think before acting inside
<think> tags. Task turns and idle maintenance share the same YOLO authority
inside /workspace and /data.";

pub const GRAMMAR: &str = "## grammar
Emit at most one <act> block per turn. The first child is <tool>; remaining
children are parameters from the registry. Values are raw text between tags.
Use exactly one action and stop after </act>.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptInputs {
    pub skill_index: String,
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
    let skills = section(
        PrefixSection::SkillIndex,
        "skill index",
        &inputs.skill_index,
        PREFIX_SKILL_INDEX,
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
    Ok(vec![identity, grammar, skills, workspace, memory])
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
