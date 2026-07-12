use crate::runtime_decision::OutputEnvelope;

pub fn protocol_for_envelope(envelope: OutputEnvelope) -> &'static str {
    match envelope {
        OutputEnvelope::Action => "Return exactly one compact <tool_call><tool>allowed tool</tool><input>ordered fields</input></tool_call>. No prose or JSON.",
        OutputEnvelope::Message => "Return exactly <final><message>owner-facing answer</message></final>. Use gathered facts only.",
        OutputEnvelope::None => "No model output expected.",
    }
}
