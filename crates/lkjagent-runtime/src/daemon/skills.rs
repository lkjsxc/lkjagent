use lkjagent_context::assemble::append_frame;
use lkjagent_context::model::{Frame, FrameKind, NoticeKind};
use lkjagent_store::events::{append_event, EventKind};
use lkjagent_tools::dispatch::load_skill_frame;
use lkjagent_tools::dispatch::DispatchOutput;
use lkjagent_tools::observe::{self, OutputKind};
use lkjagent_tools::structure_seed::scaffold_recursive_docs;
use rusqlite::Connection;

use super::runner::ResidentDaemon;
use crate::error::RuntimeResult;
use crate::prompt::token_estimate;

impl ResidentDaemon {
    pub(super) fn auto_load_recursive_structure(
        &mut self,
        conn: &Connection,
        now: &str,
    ) -> RuntimeResult<()> {
        let output = load_skill_frame(
            "recursive-structure",
            "auto-load recursive-structure",
            &self.runtime.tools,
            &mut self.dispatch_state,
        );
        self.append_dispatch_output(conn, now, &output)
    }

    pub(super) fn auto_scaffold_recursive_docs(
        &mut self,
        conn: &Connection,
        now: &str,
    ) -> RuntimeResult<()> {
        let output = match scaffold_recursive_docs(&self.runtime.tools.workspace) {
            Ok(content) => observe::ok(
                content,
                self.runtime.tools.observation_tokens,
                "inspect docs with shell.run",
            ),
            Err(error) => observe::error(error.to_string(), self.runtime.tools.observation_tokens),
        };
        self.append_output_frame(conn, now, &output.kind, output.rendered)
    }

    fn append_dispatch_output(
        &mut self,
        conn: &Connection,
        now: &str,
        output: &DispatchOutput,
    ) -> RuntimeResult<()> {
        self.append_output_frame(conn, now, &output.kind, output.rendered.clone())
    }

    fn append_output_frame(
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
        OutputKind::Skill { .. } => FrameKind::SkillBody,
    }
}

fn event_kind(kind: &OutputKind) -> EventKind {
    match kind {
        OutputKind::Notice { .. } => EventKind::Notice,
        OutputKind::Observation { .. } | OutputKind::Skill { .. } => EventKind::Observation,
    }
}
