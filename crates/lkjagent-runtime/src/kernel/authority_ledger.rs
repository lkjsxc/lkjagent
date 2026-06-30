use crate::kernel::completion::completion_gate;
use crate::kernel::decision::RuntimeDecision;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::obligation::{obligations_for, Obligation};
use crate::kernel::obligation_facts::{runtime_facts, ArtifactRootStatus, RuntimeFacts};
use crate::kernel::progress::progress_key_for_snapshot;
use crate::kernel::resolver::{resolve_obligations, resolver_label, ResolverPlan};
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn authority_ledger_entries(
    snapshot: &RuntimeSnapshot,
    event: &RuntimeEvent,
    decision: &RuntimeDecision,
) -> Vec<String> {
    let facts = runtime_facts(snapshot, event);
    let obligations = obligations_for(&facts);
    let completion = completion_gate(snapshot);
    let resolver = resolve_obligations(
        decision.mission,
        snapshot,
        &facts,
        &obligations,
        completion.allowed,
    );
    vec![
        format!("root_status={}", root_status(facts.root_status)),
        format!("obligation_set={}", obligation_set(&obligations)),
        format!("resolver_plan={}", resolver_plan(Some(&resolver))),
        format!(
            "write_contract={}",
            write_contract(facts.write_contract.as_ref())
        ),
        format!(
            "progress_key={}",
            progress_key_for_snapshot(snapshot).fingerprint()
        ),
        format!(
            "completion_blockers={}",
            none_or_join(&decision.completion_blockers)
        ),
    ]
}

fn root_status(status: ArtifactRootStatus) -> &'static str {
    match status {
        ArtifactRootStatus::Unknown => "unknown",
        ArtifactRootStatus::Missing => "missing",
        ArtifactRootStatus::RootIsFile => "root-is-file",
        ArtifactRootStatus::EmptyDirectory => "empty-directory",
        ArtifactRootStatus::IdentityIncomplete => "identity-incomplete",
        ArtifactRootStatus::StructureFailed => "structure-failed",
        ArtifactRootStatus::StructurePassed => "structure-passed",
        ArtifactRootStatus::ContentWeak => "content-weak",
        ArtifactRootStatus::Ready => "ready",
    }
}

fn obligation_set(obligations: &[Obligation]) -> String {
    if obligations.is_empty() {
        return "none".to_string();
    }
    obligations
        .iter()
        .map(|obligation| format!("{obligation:?}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn resolver_plan(plan: Option<&ResolverPlan>) -> String {
    plan.map_or_else(|| "none".to_string(), resolver_label)
}

fn write_contract(contract: Option<&crate::kernel::WriteContractFacts>) -> String {
    contract.map_or_else(
        || "none".to_string(),
        |contract| format!("{}:{}", contract.root, contract.exact_paths.join("|")),
    )
}

fn none_or_join(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

#[allow(dead_code)]
fn _facts_type_anchor(_: &RuntimeFacts) {}
