use crate::kernel::admission::ToolAdmissionView;
use crate::kernel::progress::progress_key_for_snapshot;
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn repeat_guard(
    view: ToolAdmissionView,
    snapshot: &RuntimeSnapshot,
) -> ToolAdmissionView {
    let Some(fault) = snapshot.latest_fault else {
        return view;
    };
    if snapshot.retry_count < 2 {
        return view;
    }
    let progress = progress_key_for_snapshot(snapshot).fingerprint();
    let mut guarded = view.with_exhausted_fault_guard(fault.class(), progress.clone());
    if let Some(raw) = snapshot.prior_action_fingerprint.clone() {
        if raw != progress {
            guarded.refused_action_fingerprints.push(raw);
        }
    }
    guarded
}
