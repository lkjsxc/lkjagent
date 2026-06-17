#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use lkjagent_protocol::{Action, Param};
use lkjagent_store::schema::setup;
use lkjagent_tools::dispatch::{DispatchState, ToolRuntime};
use rusqlite::Connection;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

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

pub fn runtime(workspace: PathBuf) -> TestResult<ToolRuntime> {
    let skill_library = workspace.join("skills");
    fs::create_dir_all(&skill_library)?;
    Ok(ToolRuntime::new(
        workspace,
        skill_library,
        "2026-01-01T00:00:00Z",
    ))
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

pub fn valid_skill(name: &str) -> &'static str {
    match name {
        "Demo Skill" => DEMO_SKILL,
        _ => OTHER_SKILL,
    }
}

const DEMO_SKILL: &str = "# Skill: Demo Skill

## Purpose

Exercise the skill runtime.

## Trigger

A test needs a valid demo skill.

## Context

- No additional context.

## Procedure

1. Run `pwd`.

## Checks

- `pwd` prints the workspace.

## Must Not

- Do not mutate unrelated files.
";

const OTHER_SKILL: &str = "# Skill: Other Skill

## Purpose

Exercise the skill budget path.

## Trigger

A test needs a second valid skill.

## Context

- No additional context.

## Procedure

1. Run `pwd`.

## Checks

- `pwd` prints the workspace.

## Must Not

- Do not mutate unrelated files.
";
