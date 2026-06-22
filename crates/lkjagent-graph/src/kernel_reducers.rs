use crate::kernel_events::{AuditResult, CaseEvent, Fault, ToolObservation};
use crate::kernel_types::StateVector;
use crate::kernel_vector::update_state_vector;

pub fn apply_audit_result(vector: &StateVector, audit: &AuditResult) -> StateVector {
    match audit.kind.as_str() {
        "doc.audit" => update_state_vector(
            vector,
            &CaseEvent::DocAudit {
                passed: audit.passed,
            },
        ),
        "artifact.audit" => update_state_vector(
            vector,
            &CaseEvent::ArtifactAudit {
                passed: audit.passed,
            },
        ),
        "relation.audit" => update_state_vector(
            vector,
            &CaseEvent::RelationAudit {
                passed: audit.passed,
            },
        ),
        "mock-content.audit" => update_state_vector(
            vector,
            &CaseEvent::MockContentAudit {
                passed: audit.passed,
            },
        ),
        "model-name.audit" => update_state_vector(
            vector,
            &CaseEvent::ModelNameAudit {
                passed: audit.passed,
            },
        ),
        _ if audit.passed => vector.clone(),
        _ => update_state_vector(
            vector,
            &CaseEvent::ArtifactObjectiveMismatch {
                reason: audit.kind.clone(),
            },
        ),
    }
}

pub fn apply_tool_observation(vector: &StateVector, observation: &ToolObservation) -> StateVector {
    if observation.succeeded {
        update_state_vector(vector, &CaseEvent::ParsedAction)
    } else {
        update_state_vector(
            vector,
            &CaseEvent::ToolParameterFault {
                expected: observation.tool.clone(),
                received: "failed observation".to_string(),
            },
        )
    }
}

pub fn classify_fault(text: &str) -> Fault {
    let lower_text = text.to_ascii_lowercase();
    if lower_text.contains("parse") {
        Fault::ParserFault
    } else if lower_text.contains("parameter") || lower_text.contains("schema") {
        Fault::ToolParameterFault
    } else if lower_text.contains("drift") {
        Fault::ArtifactDrift
    } else if lower_text.contains("repeat") {
        Fault::RepeatedActionRefusal
    } else if lower_text.contains("queue") {
        Fault::QueueInterruption
    } else {
        Fault::ToolExecutionFault
    }
}
