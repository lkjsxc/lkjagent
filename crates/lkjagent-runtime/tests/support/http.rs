use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};

use super::TestResult;

pub struct StubServer {
    pub base_url: String,
    handle: JoinHandle<std::io::Result<()>>,
}

pub fn serve_responses(bodies: Vec<String>) -> TestResult<StubServer> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let handle = thread::spawn(move || {
        for body in bodies {
            let (mut stream, _) = listener.accept()?;
            drain_request(&mut stream)?;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(response.as_bytes())?;
        }
        Ok(())
    });
    Ok(StubServer {
        base_url: format!("http://{address}"),
        handle,
    })
}

impl StubServer {
    pub fn join(self) -> TestResult<()> {
        self.handle
            .join()
            .map_err(|_| "stub server thread failed")?
            .map_err(Box::<dyn std::error::Error>::from)
    }
}

pub fn completion(content: &str) -> String {
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":\"{}\"}},\"finish_reason\":\"stop\"}}],\"usage\":{{\"prompt_tokens\":5,\"completion_tokens\":3}}}}",
        json_string(content)
    )
}

pub fn length_completion(content: &str) -> String {
    format!(
        "{{\"choices\":[{{\"message\":{{\"content\":\"{}\"}},\"finish_reason\":\"length\"}}],\"usage\":{{\"prompt_tokens\":5,\"completion_tokens\":1024}}}}",
        json_string(content)
    )
}

fn drain_request(stream: &mut std::net::TcpStream) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            return Ok(());
        }
        buffer.extend_from_slice(&chunk[..read]);
        if request_complete(&buffer) {
            return Ok(());
        }
    }
}

fn request_complete(buffer: &[u8]) -> bool {
    let text = String::from_utf8_lossy(buffer);
    let Some(header_end) = text.find("\r\n\r\n") else {
        return false;
    };
    let headers = &text[..header_end];
    let length = headers.lines().find_map(content_length).unwrap_or(0);
    buffer.len() >= header_end.saturating_add(4).saturating_add(length)
}

fn content_length(line: &str) -> Option<usize> {
    let (name, value) = line.split_once(':')?;
    name.eq_ignore_ascii_case("content-length")
        .then(|| value.trim().parse::<usize>().ok())
        .flatten()
}

fn json_string(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect(),
            '\n' => "\\n".chars().collect(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            other => vec![other],
        })
        .collect()
}
