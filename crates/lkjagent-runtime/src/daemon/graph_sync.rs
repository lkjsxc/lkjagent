use rusqlite::Connection;

use super::graph_policy::{completion_decision, effective_policy, policy_for};
use super::runner::ResidentDaemon;
use crate::graph_state::render_state;
use crate::mode::ActiveModePolicy;

impl ResidentDaemon {
    pub(super) fn clear_graph_dispatch_state(&mut self) {
        self.dispatch_state.graph_state = None;
        self.dispatch_state.graph_completion_ready = true;
        self.dispatch_state.graph_missing.clear();
        self.dispatch_state.graph_policy = None;
        self.dispatch_state.effective_policy = None;
        self.dispatch_state.authority_view = None;
    }

    pub(super) fn sync_graph_dispatch_state(&mut self, conn: &Connection) {
        let Some(graph) = self.state.graph.as_ref() else {
            self.clear_graph_dispatch_state();
            return;
        };
        self.dispatch_state.graph_state = Some(render_state(graph));
        self.dispatch_state.graph_policy = Some(policy_for(graph));
        match completion_decision(conn, graph) {
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

    pub(super) fn sync_effective_dispatch_policy(
        &mut self,
        conn: &Connection,
        mode_policy: &ActiveModePolicy,
    ) {
        self.dispatch_state.authority_view = None;
        if mode_policy.graph_policy_applies {
            self.sync_graph_dispatch_state(conn);
        } else {
            self.clear_graph_dispatch_state();
        }
        self.dispatch_state.effective_policy = Some(effective_policy(
            mode_policy,
            self.dispatch_state.graph_policy.as_ref(),
        ));
    }
}
