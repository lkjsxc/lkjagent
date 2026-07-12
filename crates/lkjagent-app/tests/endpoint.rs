use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

use lkjagent_app::cli;
use lkjagent_app::daemon::Endpoint;
use lkjagent_app::endpoint::LlmEndpoint;
use lkjagent_core::render::Prompt;

type TestResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[test]
fn llm_endpoint_uses_configured_chat_endpoint() -> TestResult<()> {
    let _env = EndpointEnvGuard::unset();
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let url = format!("http://{}", listener.local_addr()?);
    let data = fixture_root("endpoint")?;
    fs::write(
        data.join("lkjagent.json"),
        format!(
            "{{\"endpoint_url\":\"{url}\",\"endpoint_model\":\"local\",\"endpoint_timeout_seconds\":5}}"
        ),
    )?;
    let handle = thread::spawn(move || serve_once(listener));
    let mut endpoint = LlmEndpoint::new(&data);
    let text = endpoint.complete(&prompt(), 0)?.content;
    handle.join().map_err(|_| "server thread failed")??;
    assert_eq!(text, "<final><message>hello</message></final>");
    Ok(())
}

#[test]
fn flat_config_exposes_workspace_and_budget_keys() -> TestResult<()> {
    let data = fixture_root("flat-config")?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"workspace_root\":\"visible-workspace\",\"prompt_context_tokens\":32768,\"live_campaign_seconds\":900}",
    )?;

    cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "send",
        "todo check flat config workspace",
    ])?;
    let status = cli::run(["--data", data.to_string_lossy().as_ref(), "doctor"])?;
    let json = cli::run([
        "--data",
        data.to_string_lossy().as_ref(),
        "doctor",
        "--json",
    ])?;

    assert!(data
        .join("visible-workspace/artifacts/transcripts/queue-000001.md")
        .exists());
    assert!(status.contains("prompt_cap=32768"));
    assert!(status.contains("live_seconds=900"));
    assert!(json.contains("visible-workspace"));
    Ok(())
}

#[test]
fn nested_config_is_rejected() -> TestResult<()> {
    let data = fixture_root("nested-config")?;
    fs::write(
        data.join("lkjagent.json"),
        "{\"endpoint\":{\"url\":\"http://127.0.0.1\",\"model\":\"local\"}}",
    )?;

    let error = match cli::run(["--data", data.to_string_lossy().as_ref(), "doctor"]) {
        Ok(output) => return Err(format!("nested config was accepted: {output}").into()),
        Err(error) => error,
    };

    assert!(error.contains("must not be nested"));
    Ok(())
}

fn serve_once(listener: TcpListener) -> TestResult<()> {
    let (mut stream, _) = listener.accept()?;
    let mut request = [0_u8; 4096];
    let count = stream.read(&mut request)?;
    let body = String::from_utf8_lossy(&request[..count]);
    assert!(body.contains("/v1/chat/completions"));
    assert!(body.contains("</message></final>"));
    let response = "{\"choices\":[{\"message\":{\"content\":\"<final><message>hello</message></final>\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":1,\"completion_tokens\":1,\"total_tokens\":2}}";
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
        stop: "</message></final>".to_string(),
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

struct EndpointEnvGuard {
    url: Option<String>,
    model: Option<String>,
    timeout: Option<String>,
}

impl EndpointEnvGuard {
    fn unset() -> Self {
        let guard = Self {
            url: std::env::var("LKJAGENT_ENDPOINT_URL").ok(),
            model: std::env::var("LKJAGENT_MODEL").ok(),
            timeout: std::env::var("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS").ok(),
        };
        std::env::remove_var("LKJAGENT_ENDPOINT_URL");
        std::env::remove_var("LKJAGENT_MODEL");
        std::env::remove_var("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS");
        guard
    }
}

impl Drop for EndpointEnvGuard {
    fn drop(&mut self) {
        restore_env("LKJAGENT_ENDPOINT_URL", &self.url);
        restore_env("LKJAGENT_MODEL", &self.model);
        restore_env("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS", &self.timeout);
    }
}

fn restore_env(name: &str, value: &Option<String>) {
    if let Some(value) = value {
        std::env::set_var(name, value);
    } else {
        std::env::remove_var(name);
    }
}
