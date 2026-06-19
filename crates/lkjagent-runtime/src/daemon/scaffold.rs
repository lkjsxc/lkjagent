use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_tools::benchmark_seed::scaffold_markdown_corpus;
use lkjagent_tools::observe::{self, OutputKind};
use lkjagent_tools::structure_seed::{scaffold_profile, ScaffoldProfile};
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::prompt::token_estimate;

impl ResidentDaemon {
    pub(super) fn auto_scaffold_recursive_docs(
        &mut self,
        conn: &Connection,
        now: &str,
        profile: ScaffoldProfile,
    ) -> RuntimeResult<()> {
        let output = match scaffold_profile(&self.runtime.tools.workspace, profile) {
            Ok(content) => observe::ok(
                content,
                self.runtime.tools.observation_tokens,
                "inspect docs with shell.run",
            ),
            Err(error) => observe::error(error.to_string(), self.runtime.tools.observation_tokens),
        };
        self.append_output_frame(conn, now, &output.kind, output.rendered)?;
        if matches!(output.kind, OutputKind::Observation { .. }) {
            self.record_scaffold_graph_evidence(
                conn,
                now,
                "recursive docs scaffold",
                Some("docs"),
            )?;
        }
        Ok(())
    }

    pub(super) fn auto_scaffold_markdown_corpus(
        &mut self,
        conn: &Connection,
        now: &str,
        target: usize,
    ) -> RuntimeResult<()> {
        let output = match scaffold_markdown_corpus(&self.runtime.tools.workspace, target) {
            Ok(content) => observe::ok(
                content,
                self.runtime.tools.observation_tokens,
                "finish with agent.done",
            ),
            Err(error) => observe::error(error.to_string(), self.runtime.tools.observation_tokens),
        };
        self.append_output_frame(conn, now, &output.kind, output.rendered)?;
        if matches!(output.kind, OutputKind::Observation { .. }) {
            self.record_scaffold_graph_evidence(
                conn,
                now,
                "markdown corpus scaffold",
                Some("docs/benchmark-corpus"),
            )?;
        }
        Ok(())
    }

    pub(super) fn recursive_docs_requested(content: &str) -> bool {
        let lower = content.to_ascii_lowercase();
        lower.contains("docs")
            || lower.contains("documentation")
            || lower.contains("encyclopedia")
            || lower.contains("knowledge base")
            || lower.contains("wiki")
            || content.contains("ドキュメント")
            || content.contains("百科事典")
    }

    pub(super) fn benchmark_docs_requested(content: &str) -> bool {
        let lower = content.to_ascii_lowercase();
        lower.contains("docs/benchmark-corpus")
            || (lower.contains("benchmark")
                && (lower.contains("documentation") || lower.contains("corpus")))
    }

    pub(super) fn scaffold_profile(&self) -> ScaffoldProfile {
        if self.dispatch_state.control.guard.is_knowledge() {
            ScaffoldProfile::Knowledge
        } else {
            ScaffoldProfile::Generic
        }
    }

    pub(super) fn append_output_frame(
        &mut self,
        conn: &Connection,
        now: &str,
        kind: &OutputKind,
        rendered: String,
    ) -> RuntimeResult<()> {
        let tokens = token_estimate(&rendered);
        self.state.context = append_frame(
            &self.state.context,
            Frame::new(frame_kind(kind), rendered.clone(), tokens),
        );
        append_event(
            conn,
            self.event_turn(),
            event_kind(kind),
            &rendered,
            tokens as i64,
            now,
        )?;
        Ok(())
    }
}

fn frame_kind(kind: &OutputKind) -> FrameKind {
    match kind {
        OutputKind::Observation { .. } => FrameKind::Observation,
        OutputKind::Notice { .. } => FrameKind::Notice(NoticeKind::Error),
    }
}

fn event_kind(kind: &OutputKind) -> EventKind {
    match kind {
        OutputKind::Notice { .. } => EventKind::Notice,
        OutputKind::Observation { .. } => EventKind::Observation,
    }
}
