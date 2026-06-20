#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_protocol::{Action, Param};
use lkjagent_store::schema::setup;
use lkjagent_tools::dispatch::{DispatchState, ToolRuntime};
use rusqlite::Connection;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn design_path(index: usize) -> String {
    format!("docs/designs/{}/design-{index:03}.md", design_group(index))
}

pub fn main_path(index: usize) -> String {
    format!("main/arcs/{}/part-{index:03}.md", main_group(index))
}

pub fn temp_workspace(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-tools-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn design_group(index: usize) -> String {
    group_name("set", index)
}

fn main_group(index: usize) -> String {
    group_name("arc", index)
}

fn group_name(prefix: &str, index: usize) -> String {
    let group = index.saturating_sub(1) / 10 + 1;
    format!("{prefix}-{group:03}")
}

pub fn runtime(workspace: PathBuf) -> TestResult<ToolRuntime> {
    Ok(ToolRuntime::new(workspace, "2026-01-01T00:00:00Z"))
}

pub fn store() -> TestResult<Connection> {
    let conn = Connection::open_in_memory()?;
    setup(&conn)?;
    Ok(conn)
}

pub fn state() -> DispatchState {
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
