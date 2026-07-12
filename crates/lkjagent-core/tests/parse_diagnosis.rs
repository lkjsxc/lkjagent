use lkjagent_core::parse::{parse_fault_diagnosis, ParseFault};
use lkjagent_core::runtime_tool_call::ToolCallError;

#[test]
fn action_parse_faults_have_repair_guidance() {
    let diagnosis = parse_fault_diagnosis(&ParseFault::Action(ToolCallError::Attribute));
    assert!(diagnosis.contains("Repair:"));
    assert!(diagnosis.contains("Remove attributes"));
    assert!(diagnosis.contains("child tags only"));
}

#[test]
fn envelope_faults_have_next_action_guidance() {
    let diagnosis = parse_fault_diagnosis(&ParseFault::WrongBlock);
    assert!(diagnosis.contains("Use only the expected envelope"));
}
