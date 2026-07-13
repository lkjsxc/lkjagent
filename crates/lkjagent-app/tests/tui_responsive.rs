use std::error::Error;
use std::sync::mpsc;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use lkjagent_app::endpoint::{CompletionRecord, Endpoint};
use lkjagent_app::tui_input::{self, InputAction};
use lkjagent_app::tui_model::TuiModel;
use lkjagent_app::tui_reducer::reduce;
use lkjagent_app::tui_worker::Worker;
use lkjagent_core::prompt::Prompt;

struct BlockingEndpoint {
    started: mpsc::Sender<()>,
    release: mpsc::Receiver<()>,
}

impl Endpoint for BlockingEndpoint {
    fn complete(&mut self, _: &Prompt, _: u32) -> Result<CompletionRecord, String> {
        self.started.send(()).map_err(|error| error.to_string())?;
        self.release.recv().map_err(|error| error.to_string())?;
        Err("test release".into())
    }
}

#[test]
fn input_remains_reducible_while_worker_endpoint_is_blocked() -> Result<(), Box<dyn Error>> {
    let data = std::env::temp_dir().join(format!("lkjagent-tui-blocked-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&data);
    lkjagent_app::public_loop::send_message(&data, "inspect notes", false)?;
    let (started_tx, started_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let worker = Worker::spawn_with(&data, move || {
        Box::new(BlockingEndpoint {
            started: started_tx,
            release: release_rx,
        })
    })?;
    started_rx.recv_timeout(Duration::from_secs(5))?;
    let mut model = TuiModel::new(80, 24);
    apply_native(&mut model, Event::Paste("日本語".into()));
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Char('本'), KeyModifiers::NONE)),
    );
    apply_native(&mut model, Event::Resize(41, 9));
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL)),
    );
    apply_native(&mut model, Event::Paste("検索".into()));
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
    );
    assert_eq!(model.composer.text, "日本語");
    assert_eq!((model.width, model.height), (41, 9));
    assert!(model.screen.activity.expanded);
    assert!(!model.search_active);
    release_tx.send(())?;
    drop(worker);
    Ok(())
}

#[test]
fn native_key_mapping_covers_edit_submit_scroll_and_quit() {
    let mut model = TuiModel::new(80, 24);
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Char('日'), KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Char('本'), KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
    );
    assert_eq!(model.composer.text, "本");
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)),
    );
    apply_native(&mut model, Event::Paste("かな".into()));
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)),
    );
    apply_native(
        &mut model,
        Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE)),
    );
    assert_eq!(model.composer.text, "かな");
    let enter = tui_input::map(
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        &model,
    );
    assert!(matches!(enter.as_slice(), [InputAction::Submit]));
    let up = tui_input::map(
        Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)),
        &model,
    );
    assert!(matches!(
        up.as_slice(),
        [InputAction::LoadOlderAndScroll(_)]
    ));
    let quit = tui_input::map(
        Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        &model,
    );
    assert!(matches!(quit.as_slice(), [InputAction::Quit]));
}

fn apply_native(model: &mut TuiModel, event: Event) {
    for action in tui_input::map(event, model) {
        if let InputAction::Reduce(event) = action {
            let (next, _) = reduce(model.clone(), event);
            *model = next;
        }
    }
}
