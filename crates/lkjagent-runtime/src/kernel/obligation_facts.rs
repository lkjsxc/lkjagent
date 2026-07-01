use crate::kernel::content_atom::{
    facts_from_snapshot as content_atom_facts, generic_root_conflict, ContentAtomFacts,
};
use crate::kernel::decision::ContentWriteContract;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::manuscript::{facts_from_snapshot, ManuscriptFacts};
use crate::kernel::obligation_contract::{root_identity, write_contract_for};
use crate::kernel::obligation_parse::{
    audit_or_contract_text, candidate_event, failure_lines, inferred_kind, line_value,
    recovery_event, root_identity_status, status_from_snapshot, status_from_text,
};
use crate::kernel::snapshot::RuntimeSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRootStatus {
    Unknown,
    Missing,
    RootIsFile,
    EmptyDirectory,
    IdentityIncomplete,
    StructureFailed,
    StructurePassed,
    ContentWeak,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteContractStatus {
    Pending,
    Satisfied,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentAuditFacts {
    pub root: String,
    pub status: ArtifactRootStatus,
    pub topology_lane: String,
    pub content_lane: String,
    pub failures: Vec<String>,
    pub candidate_runtime_event: Option<RuntimeEvent>,
    pub candidate_contract_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteContractFacts {
    pub root: String,
    pub exact_paths: Vec<String>,
    pub max_files: usize,
    pub max_file_bytes: usize,
    pub max_batch_bytes: usize,
    pub required_sections: Vec<String>,
    pub forbidden_weak_phrase_classes: Vec<String>,
    pub status: WriteContractStatus,
    pub contract_id: Option<String>,
    pub atom_ids: Vec<String>,
    pub target_count: usize,
    pub count_floor: usize,
    pub continuity_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFacts {
    pub root: Option<String>,
    pub root_status: ArtifactRootStatus,
    pub document_audit: Option<DocumentAuditFacts>,
    pub write_contract: Option<WriteContractFacts>,
    pub missing_evidence: Vec<String>,
    pub weak_paths: Vec<String>,
    pub(crate) manuscript: Option<ManuscriptFacts>,
    pub(crate) content_atoms: ContentAtomFacts,
    pub(crate) generic_root_conflict: Option<String>,
    pub owner_work_exists: bool,
    pub recovery_required: bool,
    pub hard_compaction: bool,
}

pub fn runtime_facts(snapshot: &RuntimeSnapshot, event: &RuntimeEvent) -> RuntimeFacts {
    let audit = latest_audit(snapshot);
    let root = fact_root(snapshot, audit.as_ref());
    let root_status = audit
        .as_ref()
        .map(|facts| facts.status)
        .unwrap_or_else(|| status_from_snapshot(snapshot));
    let manuscript = facts_from_snapshot(snapshot);
    let content_atoms = content_atom_facts(snapshot);
    RuntimeFacts {
        write_contract: write_contract_for(snapshot, audit.as_ref(), root.as_deref()),
        root,
        root_status,
        document_audit: audit,
        manuscript,
        content_atoms,
        generic_root_conflict: generic_root_conflict(snapshot),
        missing_evidence: snapshot.evidence.missing.clone(),
        weak_paths: snapshot.artifact.weak_paths.clone(),
        owner_work_exists: snapshot.owner_work_exists(),
        recovery_required: recovery_event(event) || snapshot.latest_fault.is_some(),
        hard_compaction: snapshot.context.hard_pressure,
    }
}

pub fn write_contract_facts_for_snapshot(snapshot: &RuntimeSnapshot) -> Option<WriteContractFacts> {
    let audit = latest_audit(snapshot);
    let root = fact_root(snapshot, audit.as_ref());
    write_contract_for(snapshot, audit.as_ref(), root.as_deref())
}

pub fn root_identity_required(snapshot: &RuntimeSnapshot) -> bool {
    latest_audit(snapshot).is_some_and(|facts| root_identity_status(facts.status))
}

pub fn root_identity_contract(root: &str, kind: &str) -> WriteContractFacts {
    root_identity(root, kind)
}

impl WriteContractFacts {
    pub fn to_content_contract(&self) -> ContentWriteContract {
        ContentWriteContract {
            root: self.root.clone(),
            paths: self.exact_paths.clone(),
            max_files: self.max_files,
            max_file_bytes: self.max_file_bytes,
            max_batch_bytes: self.max_batch_bytes,
            required_sections: self.required_sections.clone(),
            forbidden_weak_phrase_classes: self.forbidden_weak_phrase_classes.clone(),
            contract_id: self.contract_id.clone(),
            atom_ids: self.atom_ids.clone(),
            target_count: self.target_count,
            count_floor: self.count_floor,
            continuity_digest: self.continuity_digest.clone(),
        }
    }
}

fn latest_audit(snapshot: &RuntimeSnapshot) -> Option<DocumentAuditFacts> {
    snapshot
        .observation
        .latest
        .as_deref()
        .and_then(document_audit_facts)
        .or_else(|| {
            snapshot
                .observation
                .latest_successful
                .as_deref()
                .and_then(document_audit_facts)
        })
}

fn fact_root(snapshot: &RuntimeSnapshot, audit: Option<&DocumentAuditFacts>) -> Option<String> {
    audit
        .map(|facts| facts.root.clone())
        .or_else(|| snapshot.artifact.root.clone())
}

fn document_audit_facts(text: &str) -> Option<DocumentAuditFacts> {
    if !audit_or_contract_text(text) {
        return None;
    }
    let root = line_value(text, "root")
        .or_else(|| line_value(text, "normalized_root"))
        .unwrap_or_else(|| "workspace".to_string());
    let failures = failure_lines(text);
    let topology = line_value(text, "topology").unwrap_or_else(|| "not-requested".to_string());
    let content = line_value(text, "content_readiness")
        .or_else(|| line_value(text, "readiness"))
        .unwrap_or_else(|| "not-requested".to_string());
    let status = status_from_text(text, &topology, &content, &failures);
    Some(DocumentAuditFacts {
        candidate_runtime_event: candidate_event(text, status),
        candidate_contract_kind: line_value(text, "kind").or_else(|| inferred_kind(&root)),
        root,
        status,
        topology_lane: topology,
        content_lane: content,
        failures,
    })
}
