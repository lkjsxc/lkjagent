use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use lkjagent_app::daemon::run_until_idle;
use lkjagent_app::endpoint::LlmEndpoint;
use rusqlite::Connection;

pub struct LiveOptions {
    pub out_dir: PathBuf,
    pub data_dir: PathBuf,
    pub duration_seconds: u64,
    pub force_skip: bool,
}

pub fn run(options: LiveOptions) -> Result<PathBuf, String> {
    fs::create_dir_all(&options.out_dir).map_err(|error| error.to_string())?;
    let missing = if options.force_skip {
        crate::experiment_live_config::force_missing()
    } else {
        crate::experiment_live_config::missing_endpoint(&options.data_dir)
    };
    for profile in profiles() {
        let dir = options.out_dir.join(profile.name);
        fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
        if missing.is_empty() {
            run_profile(&options, &profile, &dir)?;
        } else {
            write_skip(&dir, &profile, &missing)?;
        }
    }
    let adoption_path = options.out_dir.join("adoption.md");
    fs::write(&adoption_path, adoption(&missing)).map_err(|error| error.to_string())?;
    Ok(adoption_path)
}

struct Profile {
    name: &'static str,
    objective: &'static str,
    idea: &'static str,
}

fn profiles() -> Vec<Profile> {
    vec![
        Profile {
            name: "personal-workspace",
            objective: "Review journal, todo, calendar, finance, and notes records.",
            idea: "one workspace personal lanes",
        },
        Profile {
            name: "software-project",
            objective: "Summarize project records, repository evidence, and report next actions.",
            idea: "project evidence lane",
        },
        Profile {
            name: "structured-artifact",
            objective: "Draft a nested report using an artifact manifest and checked units.",
            idea: "artifact manifest units",
        },
        Profile {
            name: "protocol-stress",
            objective: "Exercise tool-call format, parse faults, admission, and recovery.",
            idea: "strict protocol recovery",
        },
    ]
}

fn run_profile(options: &LiveOptions, profile: &Profile, out: &Path) -> Result<(), String> {
    let data = options.data_dir.join(profile.name);
    fs::create_dir_all(&data).map_err(|error| error.to_string())?;
    crate::experiment_live_config::install_profile_config(&options.data_dir, &data)?;
    enqueue(&data, profile.objective)?;
    let target = Duration::from_secs(options.duration_seconds.max(1));
    let started = Instant::now();
    let mut endpoint = LlmEndpoint::new(&data);
    let mut final_state = "open".to_string();
    let mut status = "ran".to_string();
    let mut note = String::new();
    let mut turns = 0_u64;
    while started.elapsed() < target {
        match run_until_idle(&data, &mut endpoint, 1) {
            Ok(snapshot) => {
                turns += 1;
                final_state = format!("{:?}", snapshot.task.state).to_ascii_lowercase();
            }
            Err(error) => {
                status = "blocked".to_string();
                note = error;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    let elapsed = started.elapsed().as_secs();
    let metrics = metrics_line(&data)?;
    fs::write(
        out.join("summary.md"),
        format!(
            "# Live Profile Summary\n\nprofile={}\nstatus={}\nstate={}\ntarget_seconds={}\nelapsed_seconds={}\nturns={}\nobjective={}\nmetrics={}\nnote={}\n",
            profile.name,
            status,
            final_state,
            target.as_secs(),
            elapsed,
            turns,
            profile.objective,
            metrics,
            note
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        out.join("raw-evidence.md"),
        format!(
            "# Raw Evidence\n\ndata={}\nmetrics={}\nnote={}\n",
            data.display(),
            metrics,
            note
        ),
    )
    .map_err(|error| error.to_string())
}

fn metrics_line(data: &Path) -> Result<String, String> {
    let conn = Connection::open(data.join("lkjagent.sqlite3")).map_err(|e| e.to_string())?;
    let names = [
        "prompt_frames",
        "provider_exchanges",
        "observations",
        "artifacts",
        "check_results",
        "context_items",
    ];
    names
        .iter()
        .map(|name| table_count(&conn, name).map(|count| format!("{name}={count}")))
        .collect::<Result<Vec<_>, _>>()
        .map(|items| items.join(","))
}

fn table_count(conn: &Connection, table: &str) -> Result<i64, String> {
    let sql = format!("SELECT COUNT(*) FROM {table}");
    conn.query_row(&sql, [], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn write_skip(out: &Path, profile: &Profile, missing: &[String]) -> Result<(), String> {
    fs::write(
        out.join("summary.md"),
        format!(
            "# Live Profile Summary\n\nprofile={}\nstatus=skipped\nmissing_endpoint={}\nelapsed_seconds=0\nobjective={}\n",
            profile.name,
            missing.join(","),
            profile.objective
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        out.join("raw-evidence.md"),
        format!(
            "# Raw Evidence\n\nNo endpoint call was made. Missing endpoint input: {}\n",
            missing.join(",")
        ),
    )
    .map_err(|error| error.to_string())
}

fn enqueue(data: &Path, objective: &str) -> Result<(), String> {
    let conn =
        Connection::open(data.join("lkjagent.sqlite3")).map_err(|error| error.to_string())?;
    lkjagent_store::plan_schema::setup(&conn).map_err(|error| error.to_string())?;
    lkjagent_store::plan_access::enqueue(&conn, objective, "live-profile")
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn adoption(missing: &[String]) -> String {
    let status = if missing.is_empty() {
        "deferred"
    } else {
        "skipped"
    };
    let reason = if missing.is_empty() {
        "compare live metrics before default adoption".to_string()
    } else {
        format!("missing endpoint input {}", missing.join(","))
    };
    let mut lines = vec!["# Live Profile Adoption Ledger".to_string(), String::new()];
    for profile in profiles() {
        lines.push(format!(
            "- idea={} profile={} status={} reason={}",
            profile.idea, profile.name, status, reason
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}
