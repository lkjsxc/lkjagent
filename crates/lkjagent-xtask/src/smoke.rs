use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: &[String], root: &Path) -> i32 {
    match args {
        [] => replay(root),
        [cmd] if cmd == "replay" => replay(root),
        [cmd] if cmd == "live" => live(root),
        _ => {
            print_failure(&[
                "smoke failed".to_string(),
                "exit status: 2".to_string(),
                "use: smoke replay | smoke live".to_string(),
            ]);
            2
        }
    }
}

pub fn replay_summary(root: &Path) -> Result<String, String> {
    let log = fs::read_to_string(root.join("data/logs/current-model-run.md"))
        .map_err(|error| format!("read current-model-run: {error}"))?;
    let lines = [
        header("deterministic-replay", "no-endpoint"),
        case_line("missing-root-loop", "decision-missing-root", "stories/novel-named", "stories/iwanna", &log, "missing_root", "blocked"),
        case_line("generic-root", "decision-generic-root", "structured-output", "stories/the-bell-rings-twice/manuscript/chapter-01.md", &log, "structured-output", "blocked"),
        case_line("false-close", "decision-false-close", "none", "none", &log, "agent.done", "refused"),
        case_line("provider-anomaly", "decision-provider-anomaly", "stories/novel-named", "none", &log, "provider anomaly", "blocked"),
        case_line("manuscript-incomplete", "decision-manuscript-incomplete", "stories/bell-rings-twice", "stories/bell-rings-twice/manuscript/chapter-01.md", &log, "missing_manuscript_paths", "refused"),
        "token_aggregate=prompt_known=0 completion_known=0 total_known=0 unknown=0 source=deterministic-replay".to_string(),
    ];
    Ok(lines.join("\n"))
}

fn replay(root: &Path) -> i32 {
    match replay_summary(root)
        .and_then(|summary| write_summary(root, "runtime-smoke-replay", &summary))
    {
        Ok(path) => {
            println!("ok smoke-replay artifact={}", path.display());
            0
        }
        Err(error) => {
            print_failure(&[
                "smoke replay failed".to_string(),
                "exit status: 1".to_string(),
                error,
            ]);
            1
        }
    }
}

fn live(root: &Path) -> i32 {
    let configured = env_present("LKJAGENT_ENDPOINT_URL") && env_present("LKJAGENT_MODEL");
    let status = if configured {
        "live-smoke=skipped reason=explicit-operator-command-required"
    } else {
        "live-smoke=skipped reason=endpoint-config-absent"
    };
    match write_summary(root, "runtime-smoke-live", status) {
        Ok(path) => {
            println!("ok smoke-live artifact={}", path.display());
            0
        }
        Err(error) => {
            print_failure(&[
                "smoke live failed".to_string(),
                "exit status: 1".to_string(),
                error,
            ]);
            1
        }
    }
}

fn header(name: &str, mode: &str) -> String {
    format!("smoke={name} mode={mode} summary_version=1")
}

fn case_line(
    case: &str,
    decision: &str,
    root: &str,
    paths: &str,
    log: &str,
    needle: &str,
    completion_gate: &str,
) -> String {
    format!(
        "case={case} decision_ids={decision} root={root} paths={paths} word_count=0 completion_gate={completion_gate} observed_count={}",
        log.matches(needle).count()
    )
}

fn write_summary(root: &Path, dir: &str, summary: &str) -> Result<PathBuf, String> {
    let out_dir = root.join("tmp").join(dir);
    fs::create_dir_all(&out_dir).map_err(|error| format!("create smoke dir: {error}"))?;
    let path = out_dir.join("summary.txt");
    fs::write(&path, format!("{summary}\n")).map_err(|error| format!("write summary: {error}"))?;
    Ok(path)
}

fn env_present(name: &str) -> bool {
    env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

fn print_failure(lines: &[String]) {
    for line in lines {
        eprintln!("{line}");
    }
}
