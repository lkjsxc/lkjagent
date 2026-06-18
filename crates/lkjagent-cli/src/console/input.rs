use std::io::{self, BufRead};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    Line(String),
    Buffer(String),
    Eof,
    Error(String),
}

pub fn spawn_line_input<R>(reader: R) -> Receiver<InputEvent>
where
    R: BufRead + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || read_line_loop(reader, sender));
    receiver
}

fn read_line_loop<R>(mut reader: R, sender: Sender<InputEvent>)
where
    R: BufRead,
{
    loop {
        let event = match read_line(&mut reader) {
            Ok(Some(line)) => InputEvent::Line(line),
            Ok(None) => InputEvent::Eof,
            Err(error) => InputEvent::Error(error.to_string()),
        };
        let done = matches!(event, InputEvent::Eof | InputEvent::Error(_));
        if sender.send(event).is_err() || done {
            return;
        }
    }
}

fn read_line<R>(reader: &mut R) -> io::Result<Option<String>>
where
    R: BufRead,
{
    let mut bytes = Vec::new();
    if reader.read_until(b'\n', &mut bytes)? == 0 {
        return Ok(None);
    }
    while matches!(bytes.last(), Some(b'\n' | b'\r')) {
        bytes.pop();
    }
    Ok(Some(String::from_utf8_lossy(&bytes).trim().to_string()))
}
