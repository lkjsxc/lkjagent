use crate::kernel::obligation_facts::{
    DocumentAuditFacts, WriteContractFacts, WriteContractStatus,
};
use crate::kernel::obligation_parse::{inferred_kind, root_identity_status};
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
    if audit.is_some_and(|facts| root_identity_status(facts.status)) {
        return Some(root_identity(root, &contract_kind(snapshot, audit, root)));
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
