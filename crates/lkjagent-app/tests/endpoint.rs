use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

use lkjagent_app::daemon::Endpoint;
use lkjagent_app::endpoint::LlmEndpoint;
use lkjagent_core::render::Prompt;

type TestResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[test]
fn llm_endpoint_uses_configured_chat_endpoint() -> TestResult<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let url = format!("http://{}", listener.local_addr()?);
    let data = fixture_root("endpoint")?;
    fs::write(
        data.join("lkjagent.json"),
        format!("{{\"endpoint\":{{\"url\":\"{url}\",\"model\":\"local\",\"timeout-seconds\":5}}}}"),
    )?;
    std::env::set_var("LKJAGENT_ENDPOINT_URL", &url);
    std::env::set_var("LKJAGENT_MODEL", "local");
    let handle = thread::spawn(move || serve_once(listener));
    let mut endpoint = LlmEndpoint::new(&data);
    let text = endpoint.complete(&prompt(), 0)?;
    handle.join().map_err(|_| "server thread failed")??;
    assert_eq!(text, "<message>hello</message>");
    Ok(())
}

fn serve_once(listener: TcpListener) -> TestResult<()> {
    let (mut stream, _) = listener.accept()?;
    let mut request = [0_u8; 4096];
    let count = stream.read(&mut request)?;
    let body = String::from_utf8_lossy(&request[..count]);
    assert!(body.contains("/v1/chat/completions"));
    assert!(body.contains("</message>"));
    let response = "{\"choices\":[{\"message\":{\"content\":\"<message>hello</message>\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":1,\"completion_tokens\":1,\"total_tokens\":2}}";
    stream.write_all(
        format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{}",
            response.len(),
            response
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn prompt() -> Prompt {
    Prompt {
        system: "system".to_string(),
        user: "user".to_string(),
        fingerprint: "abc".to_string(),
        max_tokens: 700,
        stop: "</message>".to_string(),
    }
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-app-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
