use lkjagent_tools::dispatch::{DispatchOutput, DispatchState};
use lkjagent_tools::observe;

pub(super) fn blocked_compaction_output(
    state: &mut DispatchState,
    action_text: &str,
) -> DispatchOutput {
    let frame = observe::notice("error", "compaction only allows memory.save actions");
    let frame_ref = state.next_frame_ref;
    state.next_frame_ref = state.next_frame_ref.saturating_add(1);
    state.last_action_text = Some(action_text.to_string());
    state.last_frame_ref = Some(frame_ref);
    DispatchOutput {
        frame_ref,
        kind: frame.kind,
        content: frame.content,
        rendered: frame.rendered,
    }
}
