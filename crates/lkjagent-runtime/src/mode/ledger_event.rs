use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::model::RuntimeEvent;

pub fn event_kind(event: &RuntimeEvent) -> &'static str {
    match event {
        RuntimeEvent::OwnerMessageQueued => "owner_message_received",
        RuntimeEvent::EndpointActionParsed { .. } => "model_action_parsed",
        RuntimeEvent::EndpointActionParseFailed => "parse_fault",
        RuntimeEvent::ToolSucceeded => "tool_observation",
        RuntimeEvent::ToolFailed { .. } => "tool_error",
        RuntimeEvent::VerificationSucceeded => "verification_passed",
        RuntimeEvent::VerificationFailed => "verification_failed",
        RuntimeEvent::ContextPressureRaised => "context_pressure_detected",
        RuntimeEvent::MaintenanceTick => "maintenance_tick",
        RuntimeEvent::CompletionRequested => "completion_requested",
        RuntimeEvent::QueueBecameNonEmpty => "owner_message_received",
        RuntimeEvent::TurnBudgetCheckpoint => "turn_budget_checkpoint",
        RuntimeEvent::TurnBudgetExhausted => "turn_budget_exhausted",
    }
}

pub fn fingerprint(parts: &[&str]) -> String {
    let mut hasher = DefaultHasher::new();
    parts.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
