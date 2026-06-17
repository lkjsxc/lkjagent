#![allow(dead_code)]

use lkjagent_context::model::{ContextState, Frame, FrameKind, PrefixSection};
use lkjagent_protocol::render_observation;
use lkjagent_runtime::prompt::{build_prefix, PromptInputs};
use lkjagent_runtime::task::RuntimeState;
use lkjagent_store::schema::setup;
use lkjagent_tools::dispatch::DispatchOutput;
use lkjagent_tools::observe::OutputKind;
use rusqlite::Connection;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn prefix() -> TestResult<Vec<Frame>> {
    Ok(build_prefix(&PromptInputs {
        skill_index: "demo-skill: test trigger.".to_string(),
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
