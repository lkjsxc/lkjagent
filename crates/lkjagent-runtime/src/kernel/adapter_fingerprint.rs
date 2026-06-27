use crate::kernel::adapter::{SnapshotAdapterError, SnapshotAdapterInput};
use crate::kernel::snapshot::{AuthorityFingerprint, StalenessFingerprint};

pub(crate) fn fingerprints(
    input: &SnapshotAdapterInput,
    owner_work_exists: bool,
) -> Result<(AuthorityFingerprint, StalenessFingerprint), SnapshotAdapterError> {
    Ok((
        authority_fingerprint("authority", &authority_parts(input, owner_work_exists))?,
        staleness_fingerprint("stale", &staleness_parts(input, owner_work_exists))?,
    ))
}

fn authority_parts(input: &SnapshotAdapterInput, owner_work_exists: bool) -> Vec<String> {
    let mut parts = staleness_parts(input, owner_work_exists);
    parts.push(format!("required={}", input.required_evidence.join("|")));
    parts.push(format!("existing={}", input.existing_evidence.join("|")));
    parts
}

fn staleness_parts(input: &SnapshotAdapterInput, owner_work_exists: bool) -> Vec<String> {
    let maintenance_due = if owner_work_exists {
        false
    } else {
        input.maintenance_due
    };
    let maintenance_active = if owner_work_exists {
        false
    } else {
        input.maintenance_active
    };
    vec![
        format!("queue={:?}:{}", input.queue_head, input.pending_owner_count),
        format!("case={:?}", input.case_id),
        format!("active_mode={:?}", input.active_mode_hint),
        format!("graph={:?}:{:?}", input.graph_node, input.graph_phase),
        format!(
            "artifact={:?}:{:?}",
            input.artifact_root, input.artifact_cursor
        ),
        format!(
            "fault={:?}:{}:{:?}:{:?}",
            input.latest_fault,
            input.retry_count,
            input.prior_action_fingerprint,
            input.parameter_shape_fingerprint
        ),
        format!("missing={}", input.missing_evidence.join("|")),
        format!(
            "compaction={}:{}",
            input.context_hard_pressure,
            input.compaction_head.as_deref().unwrap_or("")
        ),
        format!(
            "maintenance={maintenance_due}:{maintenance_active}:{}",
            input.maintenance_cooldown
        ),
        format!(
            "provider={:?}:{}:{}",
            input.provider_anomaly_class,
            input.provider_retry_count,
            input.provider_pause_deadline.as_deref().unwrap_or("")
        ),
        format!(
            "observation={}:{}",
            input.latest_observation.as_deref().unwrap_or(""),
            input.latest_successful_observation.as_deref().unwrap_or("")
        ),
        format!(
            "prompt={}",
            input.prompt_frame_fingerprint.as_deref().unwrap_or("")
        ),
    ]
}

fn authority_fingerprint(
    prefix: &str,
    parts: &[String],
) -> Result<AuthorityFingerprint, SnapshotAdapterError> {
    AuthorityFingerprint::new(format!("{prefix}:{}", parts.join(";")))
        .map_err(|_| SnapshotAdapterError::EmptyFingerprint)
}

fn staleness_fingerprint(
    prefix: &str,
    parts: &[String],
) -> Result<StalenessFingerprint, SnapshotAdapterError> {
    StalenessFingerprint::new(format!("{prefix}:{}", parts.join(";")))
        .map_err(|_| SnapshotAdapterError::EmptyFingerprint)
}
