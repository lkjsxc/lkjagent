use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use lkjagent_store::tui_snapshot::{self, SnapshotPage, TuiSnapshot};
use rusqlite::Connection;

use crate::tui_model::{TuiEffect, TuiEvent, TuiModel};

pub const CONVERSATION_PAGE: usize = 100;
const ACTIVITY_PAGE: usize = 200;

pub fn newest(connection: &mut Connection) -> Result<TuiSnapshot, String> {
    page(connection, None)
}

pub fn page(connection: &mut Connection, before: Option<i64>) -> Result<TuiSnapshot, String> {
    tui_snapshot::snapshot(
        connection,
        &SnapshotPage {
            conversation_before: before,
            conversation_limit: CONVERSATION_PAGE,
            activity_before: None,
            activity_limit: ACTIVITY_PAGE,
        },
    )
    .map_err(error)
}

pub fn apply(model: &mut TuiModel, event: TuiEvent) -> Vec<TuiEffect> {
    let (next, effects) = crate::tui_reducer::reduce(model.clone(), event);
    *model = next;
    effects
}

pub fn draw(stdout: &mut impl Write, model: &TuiModel) -> Result<(), String> {
    queue!(
        stdout,
        MoveTo(0, 0),
        Clear(ClearType::All),
        Print(crate::tui_render::render(model))
    )
    .map_err(error)?;
    stdout.flush().map_err(error)
}

pub fn error(value: impl std::fmt::Display) -> String {
    value.to_string()
}
