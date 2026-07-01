use crate::kernel::adapter::SnapshotAdapterInput;
use crate::kernel::facts::{
    ArtifactFacts, ArtifactProgressFacts, CaseFacts, ContextFacts, EvidenceFacts, GraphFacts,
    ObservationFacts, ProviderFacts, QueueFacts,
};

pub(crate) fn case_facts(input: &SnapshotAdapterInput) -> CaseFacts {
    CaseFacts {
        case_id: input.case_id.clone(),
        owner_objective: input.owner_objective.clone(),
        task_family: input.task_family.clone(),
        ..CaseFacts::default()
    }
}

pub(crate) fn graph_facts(input: &SnapshotAdapterInput) -> GraphFacts {
    GraphFacts {
        node: input.graph_node.clone(),
        phase: input.graph_phase.clone(),
        ..GraphFacts::default()
    }
}

pub(crate) fn queue_facts(input: &SnapshotAdapterInput) -> QueueFacts {
    QueueFacts {
        head_id: input.queue_head.clone(),
        pending_owner_count: input.pending_owner_count,
    }
}

pub(crate) fn evidence_facts(input: &SnapshotAdapterInput) -> EvidenceFacts {
    EvidenceFacts {
        required: input.required_evidence.clone(),
        missing: input.missing_evidence.clone(),
        existing: input.existing_evidence.clone(),
        owners: Vec::new(),
    }
}

pub(crate) fn artifact_facts(input: &SnapshotAdapterInput) -> ArtifactFacts {
    ArtifactFacts {
        artifact_id: input.artifact_id.clone(),
        root: input.artifact_root.clone(),
        kind: input.artifact_kind.clone(),
        weak_paths: input.artifact_weak_paths.clone(),
        audit_status: input.artifact_audit_status.clone(),
        cursor: input.artifact_cursor.clone(),
        batch_cursor: input.artifact_batch_cursor.clone(),
        progress: artifact_progress(input),
        ..ArtifactFacts::default()
    }
}

pub(crate) fn artifact_progress(input: &SnapshotAdapterInput) -> ArtifactProgressFacts {
    ArtifactProgressFacts {
        plan_status: input.artifact_plan_status.clone(),
        atom_total: input.artifact_atom_total,
        atom_ready: input.artifact_atom_ready,
        atom_missing: input.artifact_atom_missing,
        next_atom: input.artifact_next_atom.clone(),
        next_path: input.artifact_next_path.clone(),
        active_contract: input.artifact_active_contract.clone(),
        measured_total: input.artifact_measured_total,
        accepted_floor: input.artifact_accepted_floor,
        assembly_pending: input.artifact_assembly_pending,
        readiness: input.artifact_readiness.clone(),
        completion_blockers: input.artifact_completion_blockers.clone(),
    }
}

pub(crate) fn observation_facts(input: &SnapshotAdapterInput) -> ObservationFacts {
    ObservationFacts {
        latest: input.latest_observation.clone(),
        latest_successful: input.latest_successful_observation.clone(),
    }
}

pub(crate) fn provider_facts(input: &SnapshotAdapterInput) -> ProviderFacts {
    ProviderFacts {
        latest_exchange_id: input.provider_exchange_id.clone(),
        anomaly_class: input.provider_anomaly_class.clone(),
        retry_count: input.provider_retry_count,
        pause_deadline: input.provider_pause_deadline.clone(),
    }
}

pub(crate) fn context_facts(input: &SnapshotAdapterInput) -> ContextFacts {
    ContextFacts {
        hard_pressure: input.context_hard_pressure,
        compaction_head: input.compaction_head.clone(),
    }
}
