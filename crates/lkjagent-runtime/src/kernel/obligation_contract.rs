use crate::kernel::manuscript::{facts_from_snapshot, ManuscriptFacts};
use crate::kernel::obligation_facts::{
    ArtifactRootStatus, DocumentAuditFacts, WriteContractFacts, WriteContractStatus,
};
use crate::kernel::obligation_parse::{inferred_kind, root_identity_status, status_from_snapshot};
use crate::kernel::obligation_paths::{
    contract_paths, identity_paths, required_sections, weak_phrase_classes,
};
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn write_contract_for(
    snapshot: &RuntimeSnapshot,
    audit: Option<&DocumentAuditFacts>,
    root: Option<&str>,
) -> Option<WriteContractFacts> {
    let root = root?;
    if audit.is_some_and(|facts| root_identity_status(facts.status))
        || root_identity_status(status_from_snapshot(snapshot))
    {
        return Some(root_identity(root, &contract_kind(snapshot, audit, root)));
    }
    if let Some(facts) = facts_from_snapshot(snapshot) {
        if let Some(contract) = manuscript_contract(root, &facts) {
            return Some(contract);
        }
    }
    if let Some(contract) = structure_repair_contract(snapshot, audit, root) {
        return Some(contract);
    }
    let text = snapshot.observation.latest.as_deref()?;
    if !text.contains("candidate_action=fs.batch_write") {
        return None;
    }
    let paths = contract_paths(text, root);
    if paths.is_empty() {
        None
    } else {
        Some(batch_contract(snapshot, root, paths))
    }
}

pub(crate) fn root_identity(root: &str, kind: &str) -> WriteContractFacts {
    WriteContractFacts {
        root: root.to_string(),
        exact_paths: identity_paths(root, kind),
        max_files: 20,
        max_file_bytes: 1_800,
        max_batch_bytes: 6_000,
        required_sections: required_sections(kind),
        forbidden_weak_phrase_classes: weak_phrase_classes(),
        status: WriteContractStatus::Pending,
    }
}

fn contract_kind(
    snapshot: &RuntimeSnapshot,
    audit: Option<&DocumentAuditFacts>,
    root: &str,
) -> String {
    audit
        .and_then(|facts| facts.candidate_contract_kind.clone())
        .or_else(|| snapshot.artifact.kind.clone())
        .or_else(|| inferred_kind(root))
        .unwrap_or_else(|| "artifact".to_string())
}

fn structure_repair_contract(
    snapshot: &RuntimeSnapshot,
    audit: Option<&DocumentAuditFacts>,
    root: &str,
) -> Option<WriteContractFacts> {
    let audit = audit?;
    if audit.status != ArtifactRootStatus::StructureFailed {
        return None;
    }
    let paths = audit
        .failures
        .iter()
        .filter_map(|failure| failure_path(root, failure))
        .collect::<Vec<_>>();
    (!paths.is_empty()).then(|| batch_contract(snapshot, root, paths))
}

fn failure_path(root: &str, failure: &str) -> Option<String> {
    let (_, path) = failure.split_once(':')?;
    let relative = path.trim();
    if relative.is_empty() || relative.starts_with('/') {
        return None;
    }
    Some(crate::kernel::obligation_paths::full_path(root, relative))
}

fn manuscript_contract(root: &str, facts: &ManuscriptFacts) -> Option<WriteContractFacts> {
    let path = facts.next_path.clone()?;
    let max_file_bytes = match facts.anomaly_shrink_level {
        0 => 12_000,
        1 => 6_000,
        _ => 3_000,
    };
    Some(WriteContractFacts {
        root: root.to_string(),
        exact_paths: vec![path],
        max_files: 1,
        max_file_bytes,
        max_batch_bytes: max_file_bytes,
        required_sections: vec![
            "finished chapter prose".to_string(),
            "scene action and dialogue or interiority".to_string(),
            "continuity with prior facts".to_string(),
        ],
        forbidden_weak_phrase_classes: manuscript_weak_classes(),
        status: WriteContractStatus::Pending,
    })
}

fn manuscript_weak_classes() -> Vec<String> {
    [
        "scaffold-only",
        "outline-only",
        "story-bible-only",
        "placeholder",
        "owner-terms-only",
        "generic-example",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn batch_contract(
    snapshot: &RuntimeSnapshot,
    root: &str,
    paths: Vec<String>,
) -> WriteContractFacts {
    WriteContractFacts {
        root: root.to_string(),
        exact_paths: paths,
        max_files: 20,
        max_file_bytes: 1_800,
        max_batch_bytes: 6_000,
        required_sections: required_sections(
            snapshot.artifact.kind.as_deref().unwrap_or("artifact"),
        ),
        forbidden_weak_phrase_classes: weak_phrase_classes(),
        status: WriteContractStatus::Pending,
    }
}
