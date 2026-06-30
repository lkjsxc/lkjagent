use super::TotalResolverPlan;

pub fn resolver_label(plan: &TotalResolverPlan) -> String {
    format!("rule={} plan={}", resolver_rule_id(plan), plan_label(plan))
}

pub fn resolver_rule_id(plan: &TotalResolverPlan) -> String {
    match plan {
        TotalResolverPlan::RuntimeEffect => "runtime-effect".to_string(),
        TotalResolverPlan::ExactInspection { tool } => format!("inspect-{}", slug(tool)),
        TotalResolverPlan::SemanticWriteContract { .. } => "semantic-write-contract".to_string(),
        TotalResolverPlan::Audit { tool } => format!("audit-{}", slug(tool)),
        TotalResolverPlan::EvidenceRecording { tool } => format!("record-{}", slug(tool)),
        TotalResolverPlan::OwnerWait => "owner-wait".to_string(),
        TotalResolverPlan::BlockedHandoff { .. } => "blocked-handoff".to_string(),
        TotalResolverPlan::CloseCase => "close-case".to_string(),
    }
}

fn plan_label(plan: &TotalResolverPlan) -> String {
    match plan {
        TotalResolverPlan::RuntimeEffect => "runtime-effect".to_string(),
        TotalResolverPlan::ExactInspection { tool } => format!("exact-inspection:{tool}"),
        TotalResolverPlan::SemanticWriteContract { contract } => {
            format!(
                "semantic-write:{}:{}",
                contract.root,
                contract.exact_paths.join("|")
            )
        }
        TotalResolverPlan::Audit { tool } => format!("audit:{tool}"),
        TotalResolverPlan::EvidenceRecording { tool } => format!("evidence:{tool}"),
        TotalResolverPlan::OwnerWait => "owner-wait".to_string(),
        TotalResolverPlan::BlockedHandoff { reason } => format!("blocked-handoff:{reason}"),
        TotalResolverPlan::CloseCase => "close-case".to_string(),
    }
}

fn slug(value: &str) -> String {
    value.replace('.', "-")
}
