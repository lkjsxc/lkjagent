use std::collections::{BTreeMap, BTreeSet};

const SUITES: &[&str] = &[
    "core-contracts",
    "effects-safety",
    "store-boundaries",
    "app-flows",
    "tui-contract",
    "tui-native",
    "acceptance-negative",
];
const STATIC: &[&str] = &[
    "check-docs",
    "check-lines",
    "check-files",
    "check-style",
    "docs-authority",
    "evaluation-harness",
    "git-diff-check",
];
const DOCKER: &[&str] = &[
    "docker-build",
    "docker-test",
    "docker-lint",
    "docker-verify",
];

pub fn derivations(bytes: &[u8], source: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Ok(text) = std::str::from_utf8(bytes) else {
        return out;
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return out;
    };
    let columns = header.split('\t').collect::<Vec<_>>();
    let expected = [
        "command_id",
        "command",
        "source_commit",
        "exit_code",
        "capture_sha256",
        "measured_test_count",
        "image_sha256",
        "export_sha256",
    ];
    if columns != expected {
        return out;
    }
    let mut rows = BTreeMap::new();
    for line in lines {
        let values = line.split('\t').collect::<Vec<_>>();
        if values.len() != expected.len()
            || values[2] != source
            || values[3] != "0"
            || !hash(values[4])
            || command(values[0]) != Some(values[1])
            || rows.insert(values[0], values).is_some()
        {
            return out;
        }
    }
    if SUITES.iter().all(|id| {
        rows.get(id)
            .is_some_and(|row| row[5].parse::<u64>().is_ok_and(|count| count > 0))
    }) {
        out.insert("E01".into());
    }
    if SUITES.iter().chain(STATIC).all(|id| rows.contains_key(id)) {
        out.insert("D03".into());
    }
    if DOCKER.iter().all(|id| {
        rows.get(id).is_some_and(|row| {
            hash(row[6])
                && hash(row[7])
                && row[6] == rows["docker-build"][6]
                && row[7] == rows["docker-build"][7]
        })
    }) && rows
        .get("docker-build")
        .is_some_and(|row| row[1].contains("--no-cache"))
    {
        out.insert("E02".into());
    }
    out
}

fn command(id: &str) -> Option<&'static str> {
    Some(match id {
        "core-contracts" => "cargo test --locked -p lkjagent-core",
        "effects-safety" => "cargo test --locked -p lkjagent-effects",
        "store-boundaries" => "cargo test --locked -p lkjagent-store",
        "app-flows" => "cargo test --locked -p lkjagent-app",
        "tui-contract" => "cargo test --locked -p lkjagent-app --test tui_contract",
        "tui-native" => "cargo test --locked -p lkjagent-app --test tui_native --test tui_responsive --test tui_terminal_guard --test tui_pty",
        "acceptance-negative" => "cargo test --locked -p lkjagent-xtask --test acceptance_negative",
        "check-docs" => "cargo run --locked -p lkjagent-xtask -- check-docs",
        "check-lines" => "cargo run --locked -p lkjagent-xtask -- check-lines",
        "check-files" => "cargo run --locked -p lkjagent-xtask -- check-files",
        "check-style" => "cargo run --locked -p lkjagent-xtask -- check-style",
        "docs-authority" => "cargo run --locked -p lkjagent-xtask -- gate docs-authority",
        "evaluation-harness" => "cargo run --locked -p lkjagent-xtask -- gate evaluation-harness",
        "git-diff-check" => "git diff --check SOURCE..HEAD",
        "native-doctor" => "cargo run --locked -p lkjagent-app -- --data DATA doctor --json",
        "docker-build" => "docker compose build --no-cache verify test lint agent",
        "docker-test" => "docker compose run --rm test",
        "docker-lint" => "docker compose run --rm lint",
        "docker-verify" => "docker compose run --rm verify",
        _ => return None,
    })
}

fn hash(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
