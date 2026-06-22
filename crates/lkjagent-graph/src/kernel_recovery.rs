pub fn select_recovery(fault: &crate::kernel_events::Fault) -> Vec<String> {
    match fault {
        crate::kernel_events::Fault::ParserFault => {
            names(&["show minimal grammar", "one small action"])
        }
        crate::kernel_events::Fault::ToolParameterFault => {
            names(&["show expected schema", "repair params"])
        }
        crate::kernel_events::Fault::ArtifactDrift => {
            names(&["audit objective", "repair drifted paths"])
        }
        crate::kernel_events::Fault::RepeatedActionRefusal => {
            names(&["choose different tool", "shrink scope"])
        }
        crate::kernel_events::Fault::QueueInterruption => {
            names(&["classify owner task", "preserve active case"])
        }
        _ => names(&["inspect state", "choose smallest safe action"]),
    }
}

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
