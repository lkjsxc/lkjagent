use std::fs::File;
use std::io::{self, Read};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use super::input::InputEvent;

pub struct TerminalMode {
    saved: String,
}

impl TerminalMode {
    pub fn activate() -> io::Result<Self> {
        let output = Command::new("stty")
            .arg("-g")
            .stdin(Stdio::from(terminal()?))
            .output()?;
        if !output.status.success() {
            return Err(io::Error::other("failed to read terminal mode"));
        }
        let saved = String::from_utf8_lossy(&output.stdout).trim().to_string();
        run_stty(&["-echo", "-icanon", "-isig", "min", "1", "time", "0"])?;
        Ok(Self { saved })
    }
}

impl Drop for TerminalMode {
    fn drop(&mut self) {
        if let Ok(terminal) = terminal() {
            let _ = Command::new("stty")
                .arg(&self.saved)
                .stdin(Stdio::from(terminal))
                .status();
        }
    }
}

pub fn spawn_terminal_input() -> Receiver<InputEvent> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || read_terminal_loop(io::stdin(), sender));
    receiver
}

fn run_stty(args: &[&str]) -> io::Result<()> {
    let status = Command::new("stty")
        .args(args)
        .stdin(Stdio::from(terminal()?))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other("failed to update terminal mode"))
    }
}

fn terminal() -> io::Result<File> {
    File::open("/dev/tty")
}

fn read_terminal_loop<R>(mut reader: R, sender: Sender<InputEvent>)
where
    R: Read,
{
    let mut state = TerminalInput::default();
    loop {
        let mut byte = [0];
        let event = match reader.read(&mut byte) {
            Ok(0) => vec![InputEvent::Eof],
            Ok(_) => state.accept(byte[0]),
            Err(error) => vec![InputEvent::Error(error.to_string())],
        };
        for item in event {
            let done = matches!(item, InputEvent::Eof | InputEvent::Error(_));
            if sender.send(item).is_err() || done {
                return;
            }
        }
    }
}

#[derive(Default)]
struct TerminalInput {
    line: String,
    utf8: Vec<u8>,
    escape_bytes: usize,
}

impl TerminalInput {
    fn accept(&mut self, byte: u8) -> Vec<InputEvent> {
        if self.skip_escape() {
            return Vec::new();
        }
        match byte {
            b'\r' | b'\n' => self.submit(),
            3 => vec![InputEvent::Line("/quit".to_string())],
            4 => vec![InputEvent::Eof],
            8 | 127 => self.backspace(),
            27 => {
                self.escape_bytes = 2;
                Vec::new()
            }
            byte if byte < 0x20 => Vec::new(),
            byte => self.text(byte),
        }
    }

    fn skip_escape(&mut self) -> bool {
        if self.escape_bytes == 0 {
            return false;
        }
        self.escape_bytes -= 1;
        true
    }

    fn submit(&mut self) -> Vec<InputEvent> {
        self.utf8.clear();
        let submitted = self.line.trim().to_string();
        self.line.clear();
        vec![
            InputEvent::Line(submitted),
            InputEvent::Buffer(String::new()),
        ]
    }

    fn backspace(&mut self) -> Vec<InputEvent> {
        self.utf8.clear();
        self.line.pop();
        vec![InputEvent::Buffer(self.line.clone())]
    }

    fn text(&mut self, byte: u8) -> Vec<InputEvent> {
        self.utf8.push(byte);
        match std::str::from_utf8(&self.utf8) {
            Ok(text) => {
                self.line.push_str(text);
                self.utf8.clear();
                vec![InputEvent::Buffer(self.line.clone())]
            }
            Err(error) if error.error_len().is_some() || self.utf8.len() >= 4 => {
                self.line.push(char::REPLACEMENT_CHARACTER);
                self.utf8.clear();
                vec![InputEvent::Buffer(self.line.clone())]
            }
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InputEvent, TerminalInput};

    #[test]
    fn terminal_input_reports_visible_buffer_before_submit() {
        let mut input = TerminalInput::default();
        assert_eq!(input.accept(b'a'), vec![InputEvent::Buffer("a".into())]);
        assert_eq!(input.accept(b'b'), vec![InputEvent::Buffer("ab".into())]);
        assert_eq!(input.accept(127), vec![InputEvent::Buffer("a".to_string())]);
        assert_eq!(
            input.accept(b'\r'),
            vec![
                InputEvent::Line("a".to_string()),
                InputEvent::Buffer(String::new())
            ]
        );
    }

    #[test]
    fn terminal_input_keeps_utf8_characters() {
        let mut input = TerminalInput::default();
        let mut events = Vec::new();
        for byte in "あ".as_bytes() {
            events.extend(input.accept(*byte));
        }
        assert_eq!(events, vec![InputEvent::Buffer("あ".to_string())]);
    }
}
