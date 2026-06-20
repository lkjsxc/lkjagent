use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};

pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub authorization: Option<String>,
    pub body: String,
}

pub struct StubServer {
    pub base_url: String,
    handle: JoinHandle<std::io::Result<RecordedRequest>>,
}

pub fn serve_once(status: u16, body: &'static str) -> TestResult<StubServer> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept()?;
        let request = read_request(&mut stream)?;
        let response = format!(
            "HTTP/1.1 {status} OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes())?;
        Ok(request)
    });
    Ok(StubServer {
        base_url: format!("http://{address}"),
        handle,
    })
}

impl StubServer {
    pub fn recorded(self) -> TestResult<RecordedRequest> {
        self.handle
            .join()
            .map_err(|_| Error::other("stub server thread failed"))?
            .map_err(Box::<dyn std::error::Error>::from)
    }
}

fn read_request(stream: &mut std::net::TcpStream) -> std::io::Result<RecordedRequest> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..read]);
        if request_complete(&buffer)? {
            break;
        }
    }
    parse_request(&buffer)
}

fn request_complete(buffer: &[u8]) -> std::io::Result<bool> {
    let text = String::from_utf8_lossy(buffer);
    let Some(header_end) = text.find("\r\n\r\n") else {
        return Ok(false);
    };
    let length = content_length(&text[..header_end])?;
    Ok(buffer.len() >= header_end + 4 + length)
}

fn content_length(headers: &str) -> std::io::Result<usize> {
    for line in headers.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.eq_ignore_ascii_case("content-length") {
            return value
                .trim()
                .parse::<usize>()
                .map_err(|error| Error::new(ErrorKind::InvalidData, error));
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "missing content-length"))
}

fn parse_request(buffer: &[u8]) -> std::io::Result<RecordedRequest> {
    let text = String::from_utf8(buffer.to_vec())
        .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;
    let Some((headers, body)) = text.split_once("\r\n\r\n") else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "missing header terminator",
        ));
    };
    let (method, path) = request_target(headers)?;
    Ok(RecordedRequest {
        method,
        path,
        authorization: header_value(headers, "authorization"),
        body: body.to_string(),
    })
}

fn request_target(headers: &str) -> std::io::Result<(String, String)> {
    let line = headers
        .lines()
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing request line"))?;
    let mut parts = line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing method"))?;
    let path = parts
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing path"))?;
    Ok((method.to_string(), path.to_string()))
}

fn header_value(headers: &str, wanted: &str) -> Option<String> {
    for line in headers.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.eq_ignore_ascii_case(wanted) {
            return Some(value.trim().to_string());
        }
    }
    None
}
