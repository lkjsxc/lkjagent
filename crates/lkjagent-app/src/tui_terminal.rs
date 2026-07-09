use std::io::{IsTerminal, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rusqlite::Connection;

use crate::tui_snapshot::TuiSnapshot;
use crate::tui_state::{reduce, TuiEffect, TuiEvent, TuiModel};

pub fn can_run_tty() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

pub fn run(conn: &Connection, data_dir: &Path) -> Result<(), String> {
    let mut stdout = std::io::stdout();
    enable_raw_mode().map_err(|error| error.to_string())?;
    execute!(stdout, EnterAlternateScreen).map_err(|error| error.to_string())?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|error| error.to_string())?;
    let result = run_loop(conn, data_dir, &mut terminal);
    cleanup(&mut terminal).and(result)
}

fn run_loop<W: Write>(
    conn: &Connection,
    data_dir: &Path,
    terminal: &mut Terminal<CrosstermBackend<W>>,
) -> Result<(), String> {
    let mut model = TuiModel::new();
    let mut snapshot = load_snapshot(conn, data_dir, &mut model)?;
    let mut last_refresh = Instant::now();
    loop {
        terminal
            .draw(|frame| crate::tui_view::draw(frame, &model, &snapshot))
            .map_err(|error| error.to_string())?;
        if event::poll(Duration::from_millis(100)).map_err(|error| error.to_string())? {
            let event = event::read().map_err(|error| error.to_string())?;
            let effects = event_to_effects(event, &mut model);
            if handle_effects(conn, data_dir, &mut model, &snapshot, effects)? {
                break;
            }
        }
        if last_refresh.elapsed() >= Duration::from_millis(500) {
            snapshot = load_snapshot(conn, data_dir, &mut model)?;
            let (next, effects) = reduce(model, TuiEvent::TimerTick);
            model = next;
            if handle_effects(conn, data_dir, &mut model, &snapshot, effects)? {
                break;
            }
            last_refresh = Instant::now();
        }
    }
    Ok(())
}
fn event_to_effects(event: Event, model: &mut TuiModel) -> Vec<TuiEffect> {
    match event {
        Event::Key(key) => crate::tui_keys::apply_key(model, key),
        Event::Resize(width, height) => {
            apply_event(model, TuiEvent::TerminalResize { width, height })
        }
        _ => vec![TuiEffect::Redraw],
    }
}

fn apply_event(model: &mut TuiModel, event: TuiEvent) -> Vec<TuiEffect> {
    let (next, effects) = reduce(model.clone(), event);
    *model = next;
    effects
}

fn handle_effects(
    conn: &Connection,
    data_dir: &Path,
    model: &mut TuiModel,
    snapshot: &TuiSnapshot,
    effects: Vec<TuiEffect>,
) -> Result<bool, String> {
    for effect in effects {
        match effect {
            TuiEffect::SubmitOwnerMessage(text) => submit(conn, model, &text)?,
            TuiEffect::InterruptRun => note(model, "interrupt recorded"),
            TuiEffect::ApproveTool(card) => note(model, &format!("approved {}", card.name)),
            TuiEffect::RejectTool { card, reason } => {
                note(model, &format!("rejected {} {reason}", card.name))
            }
            TuiEffect::SaveTranscript => save(data_dir, model, snapshot)?,
            TuiEffect::Quit => return Ok(true),
            TuiEffect::Redraw => {}
        }
    }
    Ok(false)
}

fn submit(conn: &Connection, model: &mut TuiModel, text: &str) -> Result<(), String> {
    let reply = crate::console::handle_line(conn, text, &crate::clock::utc_now())?;
    if let Some(queue_id) = queued_id(&reply.output) {
        crate::tui_transcript::attach_owner_queue_id(model, text, queue_id);
    }
    if !reply.output.is_empty() {
        note(model, &reply.output);
    }
    if reply.quit {
        let (next, _) = reduce(model.clone(), TuiEvent::QuitRequested);
        *model = next;
    }
    Ok(())
}

fn queued_id(output: &str) -> Option<i64> {
    output
        .strip_prefix("queue: ")?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

fn save(data_dir: &Path, model: &mut TuiModel, snapshot: &TuiSnapshot) -> Result<(), String> {
    let path = crate::tui_transcript::save(data_dir, model, snapshot)?;
    note(model, &format!("transcript saved {}", path.display()));
    Ok(())
}

fn note(model: &mut TuiModel, text: &str) {
    let (next, _) = reduce(
        model.clone(),
        TuiEvent::StateTransitionObserved(text.to_string()),
    );
    *model = next;
}

fn load_snapshot(
    conn: &Connection,
    data_dir: &Path,
    model: &mut TuiModel,
) -> Result<TuiSnapshot, String> {
    match crate::tui_snapshot::load(conn, data_dir) {
        Ok(snapshot) => Ok(snapshot),
        Err(error) => {
            let (next, _) = reduce(model.clone(), TuiEvent::ErrorObserved(error));
            *model = next;
            Ok(TuiSnapshot::empty())
        }
    }
}

fn cleanup<W: Write>(terminal: &mut Terminal<CrosstermBackend<W>>) -> Result<(), String> {
    disable_raw_mode().map_err(|error| error.to_string())?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(|error| error.to_string())?;
    terminal.show_cursor().map_err(|error| error.to_string())
}
