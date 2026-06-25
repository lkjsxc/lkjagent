use crate::kernel::admission::ToolAdmissionView;
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn repeat_guard(
    view: ToolAdmissionView,
    snapshot: &RuntimeSnapshot,
) -> ToolAdmissionView {
    let Some(fingerprint) = snapshot.prior_action_fingerprint.clone() else {
        return view;
    };
    let Some(fault) = snapshot.latest_fault else {
        return view;
    };
    if snapshot.retry_count < 2 {
        return view;
    }
    view.with_exhausted_fault_guard(fault.class(), fingerprint)
}
