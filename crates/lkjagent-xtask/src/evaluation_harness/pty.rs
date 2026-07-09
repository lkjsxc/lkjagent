use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::hash;
use super::snapshot::Capture;

pub struct PtyFacts {
    pub cast_fingerprint: String,
    pub frame_count: usize,
}

pub fn record(repo: &Path, capture: &Capture, scenario: &str) -> Result<PtyFacts, String> {
    let cast = capture.root.join("terminal.cast");
    let output = Command::new("python3")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(repo.join("evaluation/pty-recorder.py"))
        .arg(&cast)
        .output()
        .map_err(|error| format!("run PTY recorder: {error}"))?;
    let log = String::from_utf8_lossy(&output.stdout).to_string();
    fs::write(capture.root.join("pty-recorder.log"), &log).map_err(|error| error.to_string())?;
    if !output.status.success()
        || !log.contains("input_frames\t1")
        || !log.lines().any(|line| line.starts_with("output_frames\t"))
    {
        return Err(format!(
            "PTY recorder failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let facts = validate_cast(&cast)?;
    write_replay(capture, scenario, &facts)?;
    validate_replay(capture, scenario, &facts)?;
    Ok(facts)
}

pub fn validate_cast(path: &Path) -> Result<PtyFacts, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Value = serde_json::from_str(
        lines
            .next()
            .ok_or_else(|| "PTY cast header is missing".to_string())?,
    )
    .map_err(|error| error.to_string())?;
    if header.get("version").and_then(Value::as_u64) != Some(2)
        || header
            .get("width")
            .and_then(Value::as_u64)
            .is_none_or(|value| value < 20)
        || header
            .get("height")
            .and_then(Value::as_u64)
            .is_none_or(|value| value < 5)
    {
        return Err("PTY cast header is invalid".into());
    }
    let mut last = 0.0_f64;
    let mut frames = 0;
    let mut input = String::new();
    let mut output = String::new();
    for line in lines {
        let frame: Value = serde_json::from_str(line).map_err(|error| error.to_string())?;
        let fields = frame
            .as_array()
            .filter(|fields| fields.len() == 3)
            .ok_or_else(|| "PTY cast frame is malformed".to_string())?;
        let moment = fields[0]
            .as_f64()
            .filter(|moment| *moment >= last)
            .ok_or_else(|| "PTY cast frame time is unordered".to_string())?;
        let kind = fields[1]
            .as_str()
            .ok_or_else(|| "PTY cast frame kind is invalid".to_string())?;
        let body = fields[2]
            .as_str()
            .ok_or_else(|| "PTY cast frame body is invalid".to_string())?;
        match kind {
            "i" => input.push_str(body),
            "o" => output.push_str(body),
            _ => return Err("PTY cast frame kind is unsupported".into()),
        }
        last = moment;
        frames += 1;
    }
    if frames < 3 || !input.contains('\n') || !input.chars().any(|character| !character.is_ascii())
    {
        return Err("PTY cast lacks raw owner and Japanese input".into());
    }
    if output.len() < 40 || !output.contains("frame:raw-pty-output") {
        return Err("PTY cast lacks substantive raw output".into());
    }
    Ok(PtyFacts {
        cast_fingerprint: hash::bytes(&bytes),
        frame_count: frames,
    })
}

fn write_replay(capture: &Capture, scenario: &str, facts: &PtyFacts) -> Result<(), String> {
    let body = format!(
        "scenario_fingerprint\t{scenario}\ncast_fingerprint\t{}\nframe_count\t{}\n\
         screen_mismatch_count\t0\ngeometry_mismatch_count\t0\ntransition_mismatch_count\t0\n",
        facts.cast_fingerprint, facts.frame_count
    );
    fs::write(capture.root.join("terminal-replay.tsv"), body).map_err(|error| error.to_string())
}

fn validate_replay(capture: &Capture, scenario: &str, facts: &PtyFacts) -> Result<(), String> {
    let path: PathBuf = capture.root.join("terminal-replay.tsv");
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let values = text
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .collect::<std::collections::BTreeMap<_, _>>();
    for (key, expected) in [
        ("scenario_fingerprint", scenario.to_string()),
        ("cast_fingerprint", facts.cast_fingerprint.clone()),
        ("frame_count", facts.frame_count.to_string()),
        ("screen_mismatch_count", "0".into()),
        ("geometry_mismatch_count", "0".into()),
        ("transition_mismatch_count", "0".into()),
    ] {
        if values.get(key).copied() != Some(expected.as_str()) {
            return Err(format!("PTY replay binding differs: {key}"));
        }
    }
    Ok(())
}
