use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

#[rustfmt::skip]
const SOURCE_PATHS:&[&str]=&["Cargo.toml","Cargo.lock",".cargo","rust-toolchain","rust-toolchain.toml",
    "crates","docs","evaluation","Dockerfile","docker-compose.yml","data/lkjagent.json"];

use serde_json::Value;

pub(super) fn table(path: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let source = fs::read_to_string(path).map_err(err)?;
    let mut lines = source.lines();
    let headers = lines
        .next()
        .ok_or("TSV header missing")?
        .split('\t')
        .collect::<Vec<_>>();
    if headers.iter().any(|item| item.is_empty())
        || headers.iter().collect::<BTreeSet<_>>().len() != headers.len()
    {
        return Err(format!("{} malformed header", path.display()));
    }
    lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let values = line.split('\t').collect::<Vec<_>>();
            if values.len() != headers.len() {
                return Err(format!("{} malformed", path.display()));
            }
            Ok(headers
                .iter()
                .zip(values)
                .map(|(key, value)| ((*key).to_string(), value.to_string()))
                .collect())
        })
        .collect()
}

pub(super) fn pairs(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let source = fs::read_to_string(path).map_err(err)?;
    let mut output = BTreeMap::new();
    for line in source.lines() {
        let (key, value) = line
            .split_once('\t')
            .ok_or_else(|| format!("{} malformed", path.display()))?;
        if key.is_empty() || output.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("{} duplicate key", path.display()));
        }
    }
    Ok(output)
}

pub(super) fn field<'a>(row: &'a BTreeMap<String, String>, key: &str) -> &'a str {
    row.get(key).map_or("", String::as_str)
}

pub(super) fn text(path: &Path) -> Result<String, String> {
    Ok(fs::read_to_string(path).map_err(err)?.trim().to_string())
}

pub(super) fn json(path: &Path) -> Result<Value, String> {
    serde_json::from_str(&fs::read_to_string(path).map_err(err)?).map_err(err)
}

pub(super) fn file_hash(path: &Path) -> Result<String, String> {
    Ok(hash(&fs::read(path).map_err(err)?))
}

pub(super) fn hash(bytes: &[u8]) -> String {
    crate::evaluation_harness::sha256(bytes)
}

pub(super) fn reported(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_u64)
        .map_or_else(|| "not-reported".into(), |item| item.to_string())
}

#[rustfmt::skip]
pub(super) fn overlay(mut base: Value, factors: &BTreeMap<String, Value>) -> Result<Value, String> {
    let map=base.as_object_mut().ok_or("baseline config is not an object")?; for (key,value) in factors{map.insert(key.clone(),value.clone());} Ok(base)
}

pub(super) fn valid_hash(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..].chars().all(|item| item.is_ascii_hexdigit())
}

pub(super) fn scenario_hash(root: &Path) -> Result<String, String> {
    let mut bytes = Vec::new();
    let mut paths = walk(root)?;
    paths.sort();
    for path in paths {
        bytes.extend_from_slice(
            path.strip_prefix(root)
                .map_err(err)?
                .to_string_lossy()
                .as_bytes(),
        );
        bytes.push(0);
        bytes.extend(fs::read(path).map_err(err)?);
        bytes.push(0);
    }
    Ok(hash(&bytes))
}

fn walk(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut output = Vec::new();
    for item in fs::read_dir(root).map_err(err)? {
        let path = item.map_err(err)?.path();
        if path.is_symlink() {
            return Err(format!("symlink in evidence: {}", path.display()));
        }
        if path.is_dir() {
            output.extend(walk(&path)?);
        } else {
            output.push(path);
        }
    }
    Ok(output)
}

#[rustfmt::skip]
fn manifest(root: &Path, name: &str) -> Result<(), String> {
    let rows = table(&root.join(name))?;
    let declared = rows.iter().map(|row| (field(row, "path"), field(row, "sha256"))).collect::<BTreeMap<_, _>>();
    let excluded = root.join(name);
    let files = walk(root)?.into_iter().filter(|path| path != &excluded).collect::<Vec<_>>();
    if rows.len() != declared.len() || files.len() != declared.len() { return Err("raw manifest coverage mismatch".into()); }
    for path in files {
        let relative = path.strip_prefix(root).map_err(err)?.to_string_lossy(); let actual = file_hash(&path)?;
        if declared.get(relative.as_ref()).copied() != Some(actual.as_str()) { return Err(format!("raw manifest mismatch: {relative}")); }
    }
    Ok(())
}

pub(super) fn raw_manifest(root: &Path) -> Result<(), String> {
    manifest(root, "raw-manifest.tsv")
}
pub(super) fn campaign_manifest(root: &Path) -> Result<(), String> {
    manifest(root, "campaign-manifest.tsv")
}

#[rustfmt::skip]
pub(super) fn file_map(root: &Path) -> Result<BTreeMap<String, String>, String> {
    walk(root)?.into_iter().map(|path| {
        let relative = path.strip_prefix(root).map_err(err)?.to_string_lossy().replace('\\', "/");
        Ok((relative, file_hash(&path)?))
    }).collect()
}

#[rustfmt::skip]
pub(super) fn source_clean(root: &Path) -> bool {
    !root.join(".git").exists() || Command::new("git").args(["status", "--porcelain", "--"]).args(SOURCE_PATHS)
        .current_dir(root).output().is_ok_and(|output| output.status.success() && output.stdout.is_empty())
}

#[rustfmt::skip]
pub(super) fn source_hash(root: &Path) -> Result<String, String> {
    let mut paths=if root.join(".git").exists() { let output=Command::new("git").args(["ls-files","-z","--"]).args(SOURCE_PATHS)
            .current_dir(root).output().map_err(err)?; if !output.status.success() { return Err("git ls-files failed".into()); }
        output.stdout.split(|byte|*byte==0).filter(|name|!name.is_empty()).map(|name|root.join(String::from_utf8_lossy(name).as_ref())).collect() }
        else { let mut found=Vec::new(); for name in SOURCE_PATHS { let path=root.join(name); if !path.exists(){continue;}
            if path.is_symlink(){return Err("source input is a symlink".into());} else if path.is_dir(){found.extend(walk(&path)?);}else{found.push(path);} } found };
    paths.sort_by_key(|path|path.strip_prefix(root).unwrap_or(path).to_string_lossy().into_owned());
    let mut bytes=Vec::new(); for path in paths { if path.is_symlink(){return Err("source input is a symlink".into());}
        let name=path.strip_prefix(root).map_err(err)?.to_string_lossy().replace('\\', "/"); bytes.extend_from_slice(name.as_bytes()); bytes.push(0);
        bytes.extend(fs::read(path).map_err(err)?); bytes.push(0); } Ok(hash(&bytes))
}

#[rustfmt::skip]
pub(super) fn inside(root: &Path, value: &str) -> Result<PathBuf, String> {
    let relative = Path::new(value);
    if relative.is_absolute() || relative.components().any(|item| !matches!(item, Component::Normal(_))) { return Err("run ref escaped".into()); }
    let mut joined = root.to_path_buf();
    for item in relative.components() { if let Component::Normal(item) = item { joined.push(item);
        if joined.is_symlink() { return Err("run ref is a symlink".into()); } } }
    let base = root.canonicalize().map_err(err)?; let path = joined.canonicalize().map_err(err)?;
    if !path.starts_with(base) || !path.is_dir() { Err("run ref escaped".into()) } else { Ok(path) }
}

#[rustfmt::skip]
pub(super) fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git").args(args).current_dir(root).output().map_err(err)?;
    if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).into()); }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[rustfmt::skip]
pub(super) fn git_ok(root: &Path, args: &[&str]) -> bool {
    Command::new("git").args(args).current_dir(root).status().is_ok_and(|status| status.success())
}

pub(super) fn err(error: impl std::fmt::Display) -> String {
    error.to_string()
}
