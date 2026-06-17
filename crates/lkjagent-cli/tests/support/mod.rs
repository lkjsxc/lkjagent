use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn temp_data(name: &str) -> TestResult<PathBuf> {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    path.push(format!(
        "lkjagent-cli-{name}-{}-{stamp}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn write_config(data: &PathBuf) -> TestResult<()> {
    fs::write(
        data.join("lkjagent.toml"),
        "[endpoint]\nurl = \"http://endpoint:8080\"\nmodel = \"local-test\"\n",
    )?;
    Ok(())
}

pub fn open_store(data: &PathBuf) -> TestResult<Connection> {
    let conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    lkjagent_store::schema::setup(&conn)?;
    Ok(conn)
}

pub fn valid_skill() -> &'static str {
    "# Skill: Demo Skill

## Purpose

Exercise CLI skill listing.

## Trigger

A CLI test needs a listed skill.

## Context

- No additional context.

## Procedure

1. Run `pwd`.

## Checks

- `pwd` prints the workspace.

## Must Not

- Do not mutate unrelated files.
"
}
