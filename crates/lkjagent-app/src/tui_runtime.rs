use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::terminal;
use rusqlite::Connection;

use crate::tui_input::InputAction;
use crate::tui_io::{apply, draw, error, newest, page, CONVERSATION_PAGE};
use crate::tui_model::{ComposerEvent, TuiEffect, TuiEvent, TuiModel};
use crate::tui_viewport;
use crate::tui_worker::Worker;

const FRAME_INTERVAL: Duration = Duration::from_millis(250);
const WAKE_INTERVAL: Duration = Duration::from_millis(500);
pub fn run(data_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(data_dir).map_err(|error| error.to_string())?;
    let database = data_dir.join("lkjagent.sqlite3");
    lkjagent_store::transactions::NativeStore::open(&database).map_err(error)?;
    let mut connection = Connection::open(&database).map_err(error)?;
    connection
        .busy_timeout(Duration::from_millis(100))
        .map_err(error)?;
    let (width, height) = terminal::size().map_err(error)?;
    let mut model = TuiModel::new(usize::from(width), usize::from(height));
    let first = newest(&mut connection)?;
    apply(&mut model, TuiEvent::Snapshot(first));
    let worker = Worker::start(data_dir)?;
    drive(data_dir, &mut connection, &worker, &mut model)
}

fn drive(
    data_dir: &Path,
    connection: &mut Connection,
    worker: &Worker,
    model: &mut TuiModel,
) -> Result<(), String> {
    let mut stdout = io::stdout();
    let mut last_frame = Instant::now();
    let mut last_wake = Instant::now();
    let mut oldest_complete = false;
    let mut request = 0_u64;
    loop {
        draw(&mut stdout, model)?;
        if event::poll(Duration::from_millis(50)).map_err(error)? {
            let native = event::read().map_err(error)?;
            for action in crate::tui_input::map(native, model) {
                if handle(
                    action,
                    data_dir,
                    connection,
                    worker,
                    model,
                    &mut request,
                    &mut oldest_complete,
                )? {
                    return Ok(());
                }
            }
        }
        if last_frame.elapsed() >= FRAME_INTERVAL || worker.drain() > 0 {
            if let Ok(snapshot) = newest(connection) {
                apply(model, TuiEvent::Snapshot(snapshot));
            }
            last_frame = Instant::now();
        }
        if last_wake.elapsed() >= WAKE_INTERVAL {
            worker.wake();
            last_wake = Instant::now();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle(
    action: InputAction,
    data_dir: &Path,
    connection: &mut Connection,
    worker: &Worker,
    model: &mut TuiModel,
    request: &mut u64,
    oldest_complete: &mut bool,
) -> Result<bool, String> {
    match action {
        InputAction::Quit => return Ok(true),
        InputAction::Reduce(event) => {
            apply(model, event);
        }
        InputAction::Submit => submit(data_dir, connection, worker, model, request),
        InputAction::LoadOlderAndScroll(delta) => {
            load_older(connection, model, oldest_complete)?;
            apply(model, TuiEvent::Scroll(delta));
        }
    }
    Ok(false)
}

fn submit(
    data_dir: &Path,
    connection: &mut Connection,
    worker: &Worker,
    model: &mut TuiModel,
    request: &mut u64,
) {
    *request = request.saturating_add(1);
    let request_id = format!("submission-{request}");
    let effects = apply(
        model,
        TuiEvent::Composer(ComposerEvent::Submit {
            message_id: request_id.clone(),
        }),
    );
    for effect in effects {
        let TuiEffect::CommitOwnerMessage { body, .. } = effect;
        match crate::public_loop::send_message(data_dir, &body, false) {
            Ok(receipt) => {
                apply(
                    model,
                    TuiEvent::Composer(ComposerEvent::SubmitCommitted {
                        request_id: request_id.clone(),
                        message_id: receipt.message_id,
                    }),
                );
                if let Ok(snapshot) = newest(connection) {
                    apply(model, TuiEvent::Snapshot(snapshot));
                }
                worker.wake();
            }
            Err(message) => {
                apply(
                    model,
                    TuiEvent::Composer(ComposerEvent::SubmitFailed {
                        message_id: request_id.clone(),
                        error: crate::tui_render::clip_display(&message, 160),
                    }),
                );
            }
        }
    }
}

fn load_older(
    connection: &mut Connection,
    model: &mut TuiModel,
    oldest_complete: &mut bool,
) -> Result<(), String> {
    let rows = model.screen.rows(model.width.max(1), &model.search);
    let height = model.conversation_height();
    if *oldest_complete || !tui_viewport::at_loaded_top(&model.viewport, &rows, height) {
        return Ok(());
    }
    let before = model
        .screen
        .conversation
        .iter()
        .filter_map(|item| item.sequence)
        .min();
    let Some(before) = before else { return Ok(()) };
    let page = page(connection, Some(before))?;
    *oldest_complete = page.conversation.len() < CONVERSATION_PAGE;
    apply(model, TuiEvent::Snapshot(page));
    Ok(())
}
