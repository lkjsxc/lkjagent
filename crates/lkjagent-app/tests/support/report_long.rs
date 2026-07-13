#![cfg(target_os = "linux")]
use lkjagent_app::{
    cli,
    endpoint::{CompletionRecord, Endpoint, ScriptedEndpoint},
};
use lkjagent_core::prompt::Prompt;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub type T<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub fn fixture(name: &str) -> T<PathBuf> {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("lkjagent-report-long-{name}-{n}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}

pub fn config(data: &Path) -> std::io::Result<()> {
    fs::write(
        data.join("lkjagent.json"),
        "{\"workspace_root\":\"../workspace\",\"workspace_timezone\":\"UTC\"}",
    )
}

pub fn send(data: &Path, text: &str) -> T<String> {
    let root = data.to_string_lossy();
    Ok(cli::run(["--data", root.as_ref(), "send", text])?)
}

pub fn endpoint(output: String) -> ScriptedEndpoint {
    ScriptedEndpoint {
        outputs: vec![output],
        index: 0,
    }
}

pub fn map_record(title: &str, body: &str, slug: &str, children: &str, words: u32) -> String {
    format!("<tool_call><tool>write_record</tool><input><family>report</family><title>{title}</title><body>{body}</body><slug>{slug}</slug><unit>index</unit><children>{children}</children><minimum_words>{words}</minimum_words></input></tool_call>")
}

pub fn child_record(title: &str, body: &str, slug: &str, unit: &str) -> String {
    format!("<tool_call><tool>write_record</tool><input><family>report</family><title>{title}</title><body>{body}</body><slug>{slug}</slug><unit>{unit}</unit></input></tool_call>")
}

pub fn final_message(body: &str) -> String {
    format!("<final><message>{body}</message></final>")
}

pub fn scalar(connection: &Connection, sql: &str) -> T<i64> {
    Ok(connection.query_row(sql, [], |row| row.get(0))?)
}

pub fn scalar1(connection: &Connection, sql: &str, value: &str) -> T<i64> {
    Ok(connection.query_row(sql, [value], |row| row.get(0))?)
}

pub fn inventory(root: &Path) -> T<Vec<String>> {
    fn walk(root: &Path, at: &Path, out: &mut Vec<String>) -> T {
        for row in fs::read_dir(at)? {
            let path = row?.path();
            if path.is_dir() {
                walk(root, &path, out)?
            } else {
                out.push(path.strip_prefix(root)?.to_string_lossy().into_owned())
            }
        }
        Ok(())
    }
    let mut out = vec![];
    if root.exists() {
        walk(root, root, &mut out)?;
    }
    out.sort();
    Ok(out)
}

pub struct Mutate {
    pub path: PathBuf,
    pub output: String,
}
impl Endpoint for Mutate {
    fn complete(&mut self, _: &Prompt, _: u32) -> Result<CompletionRecord, String> {
        fs::write(&self.path, "owner bytes win\n").map_err(|error| error.to_string())?;
        Ok(CompletionRecord::scripted(self.output.clone()))
    }
}
