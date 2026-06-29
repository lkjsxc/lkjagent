use crate::kernel::adapter_fingerprint::fingerprints;
use crate::kernel::event::RuntimeEvent;
use crate::kernel::facts::{
    ArtifactFacts, CaseFacts, ContextFacts, EvidenceFacts, GraphFacts, MaintenanceFacts,
    ProviderFacts, QueueFacts,
};
use crate::kernel::fault::RuntimeFault;
use crate::kernel::mission_select::select_mission;
use crate::kernel::snapshot::{RuntimeSnapshot, RuntimeSnapshotId, RuntimeSnapshotInput};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotAdapterInput {
    pub snapshot_id: u64,
    pub case_id: Option<String>,
    pub graph_node: Option<String>,
    pub graph_phase: Option<String>,
    pub active_mode_hint: Option<String>,
    pub task_family: Option<String>,
    pub owner_objective: Option<String>,
    pub queue_head: Option<String>,
    pub pending_owner_count: usize,
    pub required_evidence: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub existing_evidence: Vec<String>,
    pub artifact_id: Option<String>,
    pub artifact_root: Option<String>,
    pub artifact_kind: Option<String>,
    pub artifact_cursor: Option<String>,
    pub artifact_batch_cursor: Option<String>,
    pub artifact_weak_paths: Vec<String>,
    pub artifact_audit_status: Option<String>,
    pub latest_fault: Option<RuntimeFault>,
    pub retry_count: u32,
    pub prior_action_fingerprint: Option<String>,
    pub parameter_shape_fingerprint: Option<String>,
    pub recovery_route: Option<String>,
    pub context_hard_pressure: bool,
    pub compaction_head: Option<String>,
    pub maintenance_due: bool,
    pub maintenance_active: bool,
    pub maintenance_cooldown: bool,
    pub provider_exchange_id: Option<String>,
    pub provider_anomaly_class: Option<String>,
    pub provider_retry_count: u32,
    pub provider_pause_deadline: Option<String>,
    pub latest_observation: Option<String>,
    pub latest_successful_observation: Option<String>,
    pub latest_decision_id: Option<String>,
    pub prompt_frame_fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotAdapterError {
    MissingCaseIdForOwnerWork,
    SyntheticCaseId(String),
    EmptyFingerprint,
}

pub fn build_snapshot(
    input: SnapshotAdapterInput,
) -> Result<RuntimeSnapshot, SnapshotAdapterError> {
    validate_case_id(&input)?;
    let owner_work_exists = owner_work_exists(&input);
    let maintenance = maintenance_facts(&input, owner_work_exists);
    let (authority_fingerprint, staleness_fingerprint) = fingerprints(&input, owner_work_exists)?;
    let observation = observation_facts(&input);
    let mut snapshot = RuntimeSnapshot::new(RuntimeSnapshotInput {
        snapshot_id: RuntimeSnapshotId(input.snapshot_id),
        case: case_facts(&input),
        graph: graph_facts(&input),
        queue: queue_facts(&input),
        evidence: evidence_facts(&input),
        artifact: artifact_facts(&input),
        context: context_facts(&input),
        maintenance,
        provider: provider_facts(&input),
        authority_fingerprint,
        staleness_fingerprint,
    });
    snapshot.latest_fault = input.latest_fault;
    snapshot.retry_count = input.retry_count;
    snapshot.prior_action_fingerprint = input.prior_action_fingerprint;
    snapshot.parameter_shape_fingerprint = input.parameter_shape_fingerprint;
    snapshot.recovery_route = input.recovery_route;
    snapshot.observation = observation;
    snapshot.latest_decision_id = input.latest_decision_id;
    snapshot.prompt_frame_fingerprint = input.prompt_frame_fingerprint;
    snapshot.active_mode = select_mission(&snapshot, &RuntimeEvent::CaseResumed).active_mode();
    Ok(snapshot)
}

fn validate_case_id(input: &SnapshotAdapterInput) -> Result<(), SnapshotAdapterError> {
    if !case_bound_work_exists(input) {
        return Ok(());
    }
    match input.case_id.as_deref() {
        Some("case:unknown") => Err(SnapshotAdapterError::SyntheticCaseId(
            "case:unknown".to_string(),
        )),
        Some(value) if !value.trim().is_empty() => Ok(()),
        Some(value) => Err(SnapshotAdapterError::SyntheticCaseId(value.to_string())),
        None => Ok(()),
    }
}

fn owner_work_exists(input: &SnapshotAdapterInput) -> bool {
    input.pending_owner_count > 0 || input.queue_head.is_some() || input.case_id.is_some()
}

fn case_bound_work_exists(input: &SnapshotAdapterInput) -> bool {
    input.case_id.is_some() || input.graph_node.is_some() || input.graph_phase.is_some()
}

fn maintenance_facts(input: &SnapshotAdapterInput, owner_work_exists: bool) -> MaintenanceFacts {
    if owner_work_exists {
        return MaintenanceFacts {
            due: false,
            active: false,
            cooldown_active: input.maintenance_cooldown,
        };
    }
    MaintenanceFacts {
        due: input.maintenance_due,
        active: input.maintenance_active,
        cooldown_active: input.maintenance_cooldown,
    }
}

fn case_facts(input: &SnapshotAdapterInput) -> CaseFacts {
    CaseFacts {
        case_id: input.case_id.clone(),
        owner_objective: input.owner_objective.clone(),
        task_family: input.task_family.clone(),
        ..CaseFacts::default()
    }
}

fn graph_facts(input: &SnapshotAdapterInput) -> GraphFacts {
    GraphFacts {
        node: input.graph_node.clone(),
        phase: input.graph_phase.clone(),
        ..GraphFacts::default()
    }
}

fn queue_facts(input: &SnapshotAdapterInput) -> QueueFacts {
    QueueFacts {
        head_id: input.queue_head.clone(),
        pending_owner_count: input.pending_owner_count,
    }
}

fn evidence_facts(input: &SnapshotAdapterInput) -> EvidenceFacts {
    EvidenceFacts {
        required: input.required_evidence.clone(),
        missing: input.missing_evidence.clone(),
        existing: input.existing_evidence.clone(),
        owners: Vec::new(),
    }
}

fn artifact_facts(input: &SnapshotAdapterInput) -> ArtifactFacts {
    ArtifactFacts {
        artifact_id: input.artifact_id.clone(),
        root: input.artifact_root.clone(),
        kind: input.artifact_kind.clone(),
        weak_paths: input.artifact_weak_paths.clone(),
        audit_status: input.artifact_audit_status.clone(),
        cursor: input.artifact_cursor.clone(),
        batch_cursor: input.artifact_batch_cursor.clone(),
        ..ArtifactFacts::default()
    }
}

fn observation_facts(input: &SnapshotAdapterInput) -> crate::kernel::facts::ObservationFacts {
    crate::kernel::facts::ObservationFacts {
        latest: input.latest_observation.clone(),
        latest_successful: input.latest_successful_observation.clone(),
    }
}

fn provider_facts(input: &SnapshotAdapterInput) -> ProviderFacts {
    ProviderFacts {
        latest_exchange_id: input.provider_exchange_id.clone(),
        anomaly_class: input.provider_anomaly_class.clone(),
        retry_count: input.provider_retry_count,
        pause_deadline: input.provider_pause_deadline.clone(),
    }
}

fn context_facts(input: &SnapshotAdapterInput) -> ContextFacts {
    ContextFacts {
        hard_pressure: input.context_hard_pressure,
        compaction_head: input.compaction_head.clone(),
    }
}
