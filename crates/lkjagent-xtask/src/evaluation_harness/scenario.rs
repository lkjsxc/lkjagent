use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use super::hash;

pub const ALIASES: [&str; 4] = [
    "daily-life-recall",
    "exact-file-edit",
    "multi-project-development",
    "long-artifact-recovery",
];

pub struct Scenario {
    pub id: String,
    pub path: PathBuf,
    pub fingerprint: String,
    pub turns: Vec<(u64, String)>,
}

pub fn load(root: &Path, alias: &str) -> Result<Scenario, String> {
    if !ALIASES.contains(&alias) {
        return Err("campaign scenario must be a tracked alias".into());
    }
    let path = root.join("evaluation/scenarios").join(alias);
    let mut bytes = Vec::new();
    for name in [
        "scenario.tsv",
        "matters.tsv",
        "owner-schedule.tsv",
        "seed-manifest.tsv",
        "checks.tsv",
    ] {
        let file = path.join(name);
        if file.is_symlink() {
            return Err(format!("scenario refuses symlink: {name}"));
        }
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&fs::read(file).map_err(|error| error.to_string())?);
        bytes.push(0);
    }
    let text =
        fs::read_to_string(path.join("owner-schedule.tsv")).map_err(|error| error.to_string())?;
    let mut turns = Vec::new();
    for (index, row) in text.lines().enumerate().skip(1) {
        let fields = row.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 || hash::bytes(fields[3].as_bytes()) != fields[2] {
            return Err(format!("owner schedule row {} is malformed", index + 1));
        }
        let offset = fields[0]
            .parse::<u64>()
            .map_err(|_| format!("owner schedule row {} has invalid offset", index + 1))?;
        if turns.last().is_some_and(|(prior, _)| *prior >= offset) {
            return Err("owner schedule offsets are not increasing".into());
        }
        turns.push((offset, fields[3].to_string()));
    }
    if turns.len() != 5 || turns.last().map(|turn| turn.0) != Some(840) {
        return Err("owner schedule is not the bounded tracked schedule".into());
    }
    Ok(Scenario {
        id: alias.into(),
        path,
        fingerprint: hash::bytes(&bytes),
        turns,
    })
}

pub fn check(root: &Path, _faults: &BTreeSet<String>) -> Result<Vec<Scenario>, Vec<String>> {
    let mut found = Vec::new();
    let mut errors = Vec::new();
    for alias in ALIASES {
        match load(root, alias) {
            Ok(value) => found.push(value),
            Err(error) => errors.push(error),
        }
    }
    if errors.is_empty() {
        Ok(found)
    } else {
        Err(errors)
    }
}

pub fn endpoint_file(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| error.to_string())?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() > 16_384 {
        return Err("endpoint file must be a bounded regular non-symlink file".into());
    }
    let file = File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    file.take(16_385)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.contains(&0) || bytes.len() > 16_384 {
        return Err("endpoint file contains NUL or exceeds 16384 bytes".into());
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| "endpoint file is not UTF-8".to_string())?;
    let allowed = [
        "LKJAGENT_ENDPOINT_URL",
        "LKJAGENT_MODEL",
        "LKJAGENT_API_KEY",
        "LKJAGENT_ENDPOINT_TIMEOUT_SECONDS",
    ];
    let mut values = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("endpoint file line {} is malformed", index + 1))?;
        if !allowed.contains(&key) || values.contains_key(key) {
            return Err(format!(
                "endpoint file line {} has unknown or duplicate key",
                index + 1
            ));
        }
        if value.is_empty()
            || value.trim() != value
            || value.contains([';', '`', '\n', '\r', '"', '\'', '|', '&', '<', '>', '\\'])
            || value.contains('$')
        {
            return Err(format!(
                "endpoint file line {} has unsafe value syntax",
                index + 1
            ));
        }
        values.insert(key.to_string(), value.to_string());
    }
    if values.is_empty() {
        return Err("endpoint file has no allowed keys".into());
    }
    Ok(values)
}

pub fn validate(root: &Path, alias: &str) -> Result<(), String> {
    let scenario = load(root, alias)?;
    validate_seed(&scenario)
}

pub fn validate_seed(scenario: &Scenario) -> Result<(), String> {
    let text = fs::read_to_string(scenario.path.join("seed-manifest.tsv"))
        .map_err(|error| error.to_string())?;
    for row in text.lines().skip(1) {
        let fields = row.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 || fields[0].contains("..") || Path::new(fields[0]).is_absolute() {
            return Err("seed manifest row is malformed".into());
        }
        let path = scenario.path.join("seed").join(fields[0]);
        if path.is_symlink()
            || hash::bytes(&fs::read(path).map_err(|e| e.to_string())?) != fields[3]
        {
            return Err(format!("scenario seed differs: {}", fields[0]));
        }
    }
    Ok(())
}
