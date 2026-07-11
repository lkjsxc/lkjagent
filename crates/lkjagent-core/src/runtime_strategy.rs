pub fn instruction(policy: &str) -> Option<&'static str> {
    Some(match policy {
        "grammar-repair" => "Repair the exact envelope grammar; preserve the intended operation.",
        "concrete-example" => {
            "Copy one valid shape from the decision tool card, then replace its values."
        }
        "constrained-grammar" => {
            "Emit only fields admitted by the persisted grammar; add no prose."
        }
        "narrow-output" => {
            "Return one smaller valid envelope containing only the next necessary unit."
        }
        "reduce-unit" => "Reduce the requested semantic unit before producing output.",
        "continue-boundary" => {
            "Continue only from the last verified semantic boundary; do not repeat prior bytes."
        }
        "split-section" => "Split the work and emit exactly one independently verifiable section.",
        "replan-artifact" => "Replace the artifact plan with smaller closed units before writing.",
        "remove-hidden-tool" => "Use only tools named in the persisted decision view.",
        "correct-primitive" => {
            "Correct the rejected field to its exact persisted primitive and bounds."
        }
        "select-target" => "Choose one deterministic admitted target and state no alternatives.",
        "reinspect" => "Inspect current durable evidence before proposing another effect.",
        "retry-backoff" => "Retry only after the persisted endpoint eligibility time.",
        "alternate-sampling" => {
            "Use the reduced deterministic output allowance for this endpoint attempt."
        }
        "smaller-prompt" => {
            "Use only the bounded objective, current operation, and newest causal evidence."
        }
        "reconnect" => {
            "Treat this as a fresh endpoint connection; do not assume prior response state."
        }
        "inspect-filesystem" => "Inspect current filesystem bytes before any write or replay.",
        "idempotent-replay" => {
            "Replay only the exact persisted idempotency intent after state comparison."
        }
        "compensate" => {
            "Apply only compensation whose prior bytes and target revision still match."
        }
        "quarantine" => {
            "Preserve conflicting bytes and move the conflict to an explicit quarantine path."
        }
        "inspect-check" => "Inspect the measured check evidence and name the failing predicate.",
        "repair-source" => {
            "Change the source that caused the measured failure, not the check result."
        }
        "rerun-check" => "Rerun only the invalidated check against the changed source fingerprint.",
        "replan" => "Replace the failed operation plan while preserving satisfied obligations.",
        "inspect-state" => {
            "Compare durable state, dependencies, and evidence before choosing the next action."
        }
        "split-work" => {
            "Split the remaining obligation and perform one smaller verifiable operation."
        }
        "clarify" => "Ask one bounded question whose answer changes the blocked operation.",
        _ => return None,
    })
}

pub fn prompt_cap(policy: &str) -> usize {
    match policy {
        "smaller-prompt" => 4_000,
        "reduce-unit" | "narrow-output" => 6_000,
        "split-section" | "split-work" => 6_500,
        _ => 8_000,
    }
}
