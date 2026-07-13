use std::sync::{Arc, Mutex};

use lkjagent_app::tui_terminal::{TerminalGuard, TerminalOps};

#[derive(Clone)]
struct RecordingOps {
    log: Arc<Mutex<Vec<&'static str>>>,
    fail_hide: bool,
}

impl RecordingOps {
    fn record(&self, value: &'static str) {
        if let Ok(mut log) = self.log.lock() {
            log.push(value);
        }
    }
}

impl TerminalOps for RecordingOps {
    fn enable_raw(&mut self) -> Result<(), String> {
        self.record("raw-on");
        Ok(())
    }

    fn enter_screen(&mut self) -> Result<(), String> {
        self.record("screen-on");
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), String> {
        self.record("cursor-hide");
        if self.fail_hide {
            Err("hide failed".into())
        } else {
            Ok(())
        }
    }

    fn show_cursor(&mut self) -> Result<(), String> {
        self.record("cursor-show");
        Ok(())
    }

    fn leave_screen(&mut self) -> Result<(), String> {
        self.record("screen-off");
        Ok(())
    }

    fn disable_raw(&mut self) -> Result<(), String> {
        self.record("raw-off");
        Ok(())
    }
}

#[test]
fn guard_restores_terminal_during_unwind() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let ops = RecordingOps {
        log: Arc::clone(&log),
        fail_hide: false,
    };
    let result = std::panic::catch_unwind(|| {
        let Ok(_guard) = TerminalGuard::enter(ops) else {
            return;
        };
        std::panic::resume_unwind(Box::new("test unwind"));
    });
    assert!(result.is_err());
    assert_eq!(
        log.lock().map(|values| values.clone()).unwrap_or_default(),
        [
            "raw-on",
            "screen-on",
            "cursor-hide",
            "cursor-show",
            "screen-off",
            "raw-off"
        ]
    );
}

#[test]
fn partial_entry_failure_restores_completed_steps() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let result = TerminalGuard::enter(RecordingOps {
        log: Arc::clone(&log),
        fail_hide: true,
    });
    assert!(result.is_err());
    assert_eq!(
        log.lock().map(|values| values.clone()).unwrap_or_default(),
        [
            "raw-on",
            "screen-on",
            "cursor-hide",
            "cursor-show",
            "screen-off",
            "raw-off"
        ]
    );
}
