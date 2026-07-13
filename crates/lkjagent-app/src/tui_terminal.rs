use std::io::{self, IsTerminal};
use std::path::Path;

use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

pub trait TerminalOps {
    fn enable_raw(&mut self) -> Result<(), String>;
    fn enter_screen(&mut self) -> Result<(), String>;
    fn hide_cursor(&mut self) -> Result<(), String>;
    fn show_cursor(&mut self) -> Result<(), String>;
    fn leave_screen(&mut self) -> Result<(), String>;
    fn disable_raw(&mut self) -> Result<(), String>;
}

pub struct TerminalGuard<O: TerminalOps> {
    ops: O,
    raw: bool,
    screen: bool,
    hidden: bool,
}

impl<O: TerminalOps> TerminalGuard<O> {
    pub fn enter(ops: O) -> Result<Self, String> {
        let mut guard = Self {
            ops,
            raw: false,
            screen: false,
            hidden: false,
        };
        guard.raw = true;
        guard.ops.enable_raw()?;
        guard.screen = true;
        guard.ops.enter_screen()?;
        guard.hidden = true;
        guard.ops.hide_cursor()?;
        Ok(guard)
    }
}

impl<O: TerminalOps> Drop for TerminalGuard<O> {
    fn drop(&mut self) {
        if self.hidden {
            let _ = self.ops.show_cursor();
            self.hidden = false;
        }
        if self.screen {
            let _ = self.ops.leave_screen();
            self.screen = false;
        }
        if self.raw {
            let _ = self.ops.disable_raw();
            self.raw = false;
        }
    }
}

pub struct NativeTerminal;

impl TerminalOps for NativeTerminal {
    fn enable_raw(&mut self) -> Result<(), String> {
        enable_raw_mode().map_err(|error| error.to_string())
    }

    fn enter_screen(&mut self) -> Result<(), String> {
        execute!(io::stdout(), EnterAlternateScreen).map_err(|error| error.to_string())
    }

    fn hide_cursor(&mut self) -> Result<(), String> {
        execute!(io::stdout(), Hide).map_err(|error| error.to_string())
    }

    fn show_cursor(&mut self) -> Result<(), String> {
        execute!(io::stdout(), Show).map_err(|error| error.to_string())
    }

    fn leave_screen(&mut self) -> Result<(), String> {
        execute!(io::stdout(), LeaveAlternateScreen).map_err(|error| error.to_string())
    }

    fn disable_raw(&mut self) -> Result<(), String> {
        disable_raw_mode().map_err(|error| error.to_string())
    }
}

pub fn run(data_dir: &Path) -> Result<(), String> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("tui requires a terminal on stdin and stdout".to_string());
    }
    let _guard = TerminalGuard::enter(NativeTerminal)?;
    crate::tui_runtime::run(data_dir)
}
