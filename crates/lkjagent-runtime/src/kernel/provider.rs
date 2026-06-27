use lkjagent_llm::wire::ProviderAnomaly;

use crate::kernel::event::RuntimeEvent;

pub fn provider_anomaly_event(anomaly: &ProviderAnomaly) -> RuntimeEvent {
    RuntimeEvent::ProviderAnomaly {
        class: anomaly.kind.as_str().to_string(),
    }
}
