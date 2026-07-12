use crate::model::StepKind;
use crate::runtime_artifact::DEFAULT_UNIT_TARGET_TOKENS;

pub const WRITE_PROMPT_MAX_TOKENS: u32 = DEFAULT_UNIT_TARGET_TOKENS + 256;
use crate::runtime_decision::OutputEnvelope;

pub fn max_tokens(kind: StepKind) -> u32 {
    match kind {
        StepKind::Write | StepKind::Revise => WRITE_PROMPT_MAX_TOKENS,
        StepKind::Plan => 900,
        StepKind::Explore => 500,
        StepKind::Respond | StepKind::Ask => 700,
        StepKind::Verify => 300,
    }
}

pub fn protocol(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Plan => "Return exactly one <plan> block with one action per physical line. The only action prefixes are write, explore, and respond; never emit plan, verify, or check actions. Harness step labels are context, not actions to copy. Use concrete values, not placeholders. Write paths are relative to the workspace root: never start them with /, ./, or ../ and never use . or .. path components. Copy the syntax of the filled example.",
        StepKind::Write | StepKind::Revise => "Return exactly <content> prose </content>. Write the requested file body only. No analysis outside the block.",
        StepKind::Explore => "Return exactly one compact <tool_call><tool>allowed tool</tool><input>ordered fields</input></tool_call>. No prose or JSON.",
        StepKind::Respond | StepKind::Ask => "Return exactly <final><message>owner-facing answer</message></final>. Use gathered facts only.",
        StepKind::Verify => "Return exactly <verdict>pass or fail plus measured evidence</verdict>.",
    }
}

pub fn protocol_for_envelope(envelope: OutputEnvelope) -> &'static str {
    match envelope {
        OutputEnvelope::Content => protocol(StepKind::Write),
        OutputEnvelope::Plan => protocol(StepKind::Plan),
        OutputEnvelope::Action => protocol(StepKind::Explore),
        OutputEnvelope::Message => protocol(StepKind::Respond),
        OutputEnvelope::Verdict => protocol(StepKind::Verify),
        OutputEnvelope::None => "No model output expected.",
    }
}

pub fn expected_block(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Write | StepKind::Revise => "content",
        StepKind::Plan => "plan",
        StepKind::Explore => "tool_call",
        StepKind::Respond | StepKind::Ask => "final",
        StepKind::Verify => "verdict",
    }
}

pub fn envelope_tag(envelope: OutputEnvelope) -> Option<&'static str> {
    match envelope {
        OutputEnvelope::Content => Some("content"),
        OutputEnvelope::Plan => Some("plan"),
        OutputEnvelope::Action => Some("tool_call"),
        OutputEnvelope::Message => Some("final"),
        OutputEnvelope::Verdict => Some("verdict"),
        OutputEnvelope::None => None,
    }
}
