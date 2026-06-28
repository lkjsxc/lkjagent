use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn close_allowed(snapshot: &RuntimeSnapshot) -> bool {
    snapshot.evidence.missing.is_empty()
        && snapshot.latest_fault.is_none()
        && snapshot.artifact.weak_paths.is_empty()
        && artifact_ready_if_present(snapshot)
}

fn artifact_ready_if_present(snapshot: &RuntimeSnapshot) -> bool {
    !snapshot
        .evidence
        .required
        .iter()
        .any(|evidence| evidence == "artifact-readiness")
        || snapshot
            .evidence
            .existing
            .iter()
            .any(|evidence| evidence == "artifact-readiness")
}
