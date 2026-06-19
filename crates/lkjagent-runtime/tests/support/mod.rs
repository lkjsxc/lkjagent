#![allow(dead_code)]

use lkjagent_context::model::{ContextState, Frame, FrameKind, PrefixSection};
use lkjagent_protocol::{render_observation, Action, Param};
use lkjagent_runtime::prompt::{build_prefix, PromptInputs};
use lkjagent_runtime::task::RuntimeState;
use lkjagent_store::schema::setup;
use lkjagent_tools::dispatch::{DispatchOutput, DispatchState, ToolRuntime};
use lkjagent_tools::observe::OutputKind;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod http;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn prefix() -> TestResult<Vec<Frame>> {
    Ok(build_prefix(&PromptInputs {
        graph_state: "case=none\nphase=waiting\nnode=classify".to_string(),
        workspace_brief: "workspace brief".to_string(),
        memory_digest: "memory digest".to_string(),
    })?)
}

pub fn runtime_state() -> TestResult<RuntimeState> {
    Ok(RuntimeState::new(ContextState::new(prefix()?, Vec::new())))
}

pub fn ok_output(content: &str) -> DispatchOutput {
    DispatchOutput {
        frame_ref: 1,
        kind: OutputKind::Observation {
            status: "ok".to_string(),
        },
        content: content.to_string(),
        rendered: render_observation("ok", content),
    }
}

pub fn error_output(content: &str) -> DispatchOutput {
    DispatchOutput {
        frame_ref: 1,
        kind: OutputKind::Observation {
            status: "error".to_string(),
        },
        content: content.to_string(),
        rendered: render_observation("error", content),
    }
}

pub fn repeat_notice() -> DispatchOutput {
    DispatchOutput {
        frame_ref: 1,
        kind: OutputKind::Notice {
            kind: "error".to_string(),
        },
        content: "repeat action refused; see frame 1".to_string(),
        rendered:
            "<notice>\n<kind>error</kind>\n<content>repeat action refused</content>\n</notice>"
                .to_string(),
    }
}

pub fn summary_frame() -> Frame {
    Frame::new(
        FrameKind::Notice(lkjagent_context::model::NoticeKind::Compaction),
        "task summary: continue",
        10,
    )
}

pub fn oversized_state() -> RuntimeState {
    RuntimeState::new(ContextState::new(
        vec![Frame::new(
            FrameKind::Prefix(PrefixSection::Identity),
            "old prefix",
            10,
        )],
        vec![Frame::new(FrameKind::Observation, "old log", 29_000)],
    ))
}

pub fn store() -> TestResult<Connection> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    Ok(conn)
}

pub fn temp_workspace(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-runtime-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn tool_runtime(workspace: PathBuf) -> TestResult<ToolRuntime> {
    Ok(ToolRuntime::new(workspace, "2026-01-01T00:00:00Z"))
}

pub fn dispatch_state() -> DispatchState {
    DispatchState::default()
}

pub fn action(tool: &str, params: &[(&str, &str)]) -> Action {
    Action::new(
        tool,
        params
            .iter()
            .map(|(name, value)| Param::new(*name, *value))
            .collect(),
    )
}
