use lkjagent_tools::dispatch::{DispatchOutput, DispatchState};
use lkjagent_tools::observe;

pub(super) fn maintenance_allows(tool: &str) -> bool {
    matches!(
        tool,
        "agent.ask" | "agent.done" | "memory.find" | "memory.prune" | "memory.save" | "queue.list"
    )
}

pub(super) fn blocked_maintenance_output(
    state: &mut DispatchState,
    action_text: &str,
) -> DispatchOutput {
    let frame = observe::notice(
        "error",
        "maintenance only allows memory.find, memory.prune, memory.save, queue.list, agent.done, or agent.ask",
    );
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
