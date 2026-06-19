use super::runner::ResidentDaemon;
use crate::graph_state::render_state;

impl ResidentDaemon {
    pub(super) fn sync_graph_dispatch_state(&mut self) {
        let Some(graph) = self.state.graph.as_ref() else {
            self.dispatch_state.graph_state = None;
            self.dispatch_state.graph_completion_ready = true;
            self.dispatch_state.graph_missing.clear();
            return;
        };
        self.dispatch_state.graph_state = Some(render_state(graph));
        match lkjagent_graph::completion_decision(graph) {
            lkjagent_graph::TransitionDecision::Admit { .. } => {
                self.dispatch_state.graph_completion_ready = true;
                self.dispatch_state.graph_missing.clear();
            }
            lkjagent_graph::TransitionDecision::Defer { missing } => {
                self.dispatch_state.graph_completion_ready = false;
                self.dispatch_state.graph_missing = missing;
            }
            lkjagent_graph::TransitionDecision::Recover { reason, .. }
            | lkjagent_graph::TransitionDecision::Refuse { reason } => {
                self.dispatch_state.graph_completion_ready = false;
                self.dispatch_state.graph_missing = vec![reason];
            }
        }
    }
}
