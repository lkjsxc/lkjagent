use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::{Component, Path};
use std::sync::atomic::{AtomicU64, Ordering};

use rustix::fd::OwnedFd;
use rustix::fs::{AtFlags, Mode, OFlags, RenameFlags};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(1);

pub fn read_text(workspace: &Path, path: &str) -> Result<String, String> {
    String::from_utf8(read_bytes(workspace, path)?).map_err(|error| error.to_string())
}

#[rustfmt::skip]
pub fn read_bytes(workspace: &Path, path: &str) -> Result<Vec<u8>, String> {
    let (parent, name) = open_parent(workspace, path, false)?;
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    let fd = rustix::fs::openat(&parent, &name, flags, Mode::empty()).map_err(io_error)?;
    let mut file = std::fs::File::from(fd);
    if !file.metadata().map_err(|error| error.to_string())?.is_file() { return Err("effect target is not a regular file".to_string()); }
    let mut bytes = Vec::new(); file.read_to_end(&mut bytes).map_err(|error| error.to_string())?; Ok(bytes)
}

#[rustfmt::skip]
pub fn write_bytes(workspace: &Path, path: &str, bytes: &[u8]) -> Result<(), String> {
    let (parent, name) = open_parent(workspace, path, true)?; match read_at(&parent, &name)? { Some(prior) => replace_existing(&parent, &name, &prior, bytes), None => create_new(&parent, &name, bytes) }
}

#[rustfmt::skip]
pub fn apply_revision(workspace: &Path, path: &str, expected: &Option<Vec<u8>>, intended: &Option<Vec<u8>>) -> Result<(), String> {
    let (parent, name) = open_parent(workspace, path, intended.is_some())?;
    match (expected, intended) {
        (None, None) => ensure_absent(&parent, &name),
        (None, Some(bytes)) => create_new(&parent, &name, bytes),
        (Some(prior), Some(bytes)) => replace_existing(&parent, &name, prior, bytes),
        (Some(prior), None) => remove_existing(&parent, &name, prior),
    }
}

#[rustfmt::skip]
pub(crate) fn open_parent(workspace: &Path, path: &str, create: bool) -> Result<(OwnedFd, OsString), String> {
    let mut parts = Vec::new();
    for part in Path::new(path).components() { match part { Component::Normal(value) => parts.push(value.to_os_string()), _ => return Err("effect path is not normalized".to_string()) } }
    let name = parts.pop().ok_or_else(|| "effect path has no file name".to_string())?;
    let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW;
    let mut parent = rustix::fs::open(workspace, flags, Mode::empty()).map_err(io_error)?;
    for part in parts { parent = match rustix::fs::openat(&parent, &part, flags, Mode::empty()) {
        Ok(fd) => fd,
        Err(rustix::io::Errno::NOENT) if create => {
            let mode = Mode::RWXU | Mode::RGRP | Mode::XGRP | Mode::ROTH | Mode::XOTH;
            match rustix::fs::mkdirat(&parent, &part, mode) { Ok(()) => rustix::fs::fsync(&parent).map_err(io_error)?, Err(rustix::io::Errno::EXIST) => {}, Err(error) => return Err(error.to_string()) }
            rustix::fs::openat(&parent, &part, flags, Mode::empty()).map_err(io_error)?
        }
        Err(error) => return Err(error.to_string()),
    }; }
    Ok((parent, name))
}

#[rustfmt::skip]
pub fn path_occupied(workspace: &Path, path: &str) -> Result<bool, String> {
    let mut parts = Vec::new();
    for part in Path::new(path).components() { match part { Component::Normal(value) => parts.push(value.to_os_string()), _ => return Err("effect path is not normalized".to_string()) } }
    let name = parts.pop().ok_or_else(|| "effect path has no file name".to_string())?;
    let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW;
    let mut parent = rustix::fs::open(workspace, flags, Mode::empty()).map_err(io_error)?;
    for part in parts { parent = match rustix::fs::openat(&parent, &part, flags, Mode::empty()) { Ok(fd) => fd, Err(rustix::io::Errno::NOENT) => return Ok(false), Err(error) => return Err(error.to_string()) }; }
    match rustix::fs::statat(&parent, &name, AtFlags::SYMLINK_NOFOLLOW) { Ok(_) => Ok(true), Err(rustix::io::Errno::NOENT) => Ok(false), Err(error) => Err(error.to_string()) }
}

fn ensure_absent(parent: &OwnedFd, name: &OsString) -> Result<(), String> {
    match read_at(parent, name)? {
        None => Ok(()),
        Some(_) => Err("effect target appeared before deletion".to_string()),
    }
}

fn create_new(parent: &OwnedFd, name: &OsString, bytes: &[u8]) -> Result<(), String> {
    let temp = stage(parent, bytes)?;
    let result = rustix::fs::renameat_with(parent, &temp, parent, name, RenameFlags::NOREPLACE)
        .map_err(io_error);
    if result.is_err() {
        let _cleanup = rustix::fs::unlinkat(parent, &temp, AtFlags::empty());
    } else {
        rustix::fs::fsync(parent).map_err(io_error)?;
    }
    result
}

fn replace_existing(
    parent: &OwnedFd,
    name: &OsString,
    expected: &[u8],
    intended: &[u8],
) -> Result<(), String> {
    let temp = stage(parent, intended)?;
    if let Err(error) =
        rustix::fs::renameat_with(parent, &temp, parent, name, RenameFlags::EXCHANGE)
    {
        let _cleanup = rustix::fs::unlinkat(parent, &temp, AtFlags::empty());
        return Err(error.to_string());
    }
    match read_at(parent, &temp) {
        Ok(Some(actual)) if actual == expected => {
            rustix::fs::unlinkat(parent, &temp, AtFlags::empty()).map_err(io_error)?;
            rustix::fs::fsync(parent).map_err(io_error)
        }
        result => Err(preserve_replacement_conflict(parent, name, &temp, result)),
    }
}

#[rustfmt::skip]
fn preserve_replacement_conflict(parent: &OwnedFd, name: &OsString, captured: &OsString,
    result: Result<Option<Vec<u8>>, String>) -> String {
    let quarantine = temp_name();
    if rustix::fs::renameat_with(parent, name, parent, &quarantine, RenameFlags::NOREPLACE).is_err() {
        let sync = rustix::fs::fsync(parent).err().map_or(String::new(), |error| format!("; directory sync failed: {error}"));
        return format!("effect replacement conflict preserved at {}{sync}", captured.to_string_lossy());
    }
    if rustix::fs::renameat_with(parent, captured, parent, name, RenameFlags::NOREPLACE).is_err() {
        let restored = rustix::fs::renameat_with(parent, &quarantine, parent, name, RenameFlags::NOREPLACE);
        let sync = rustix::fs::fsync(parent).err().map_or(String::new(), |error| format!("; directory sync failed: {error}"));
        return format!("effect replacement conflict preserved at {}; restore={restored:?}{sync}", captured.to_string_lossy());
    }
    let sync = rustix::fs::fsync(parent).err().map_or(String::new(), |error| format!("; directory sync failed: {error}"));
    let reason = match result { Ok(_) => "effect target changed before replacement".to_string(), Err(error) => error };
    format!("{reason}; replacement quarantined at {}{sync}", quarantine.to_string_lossy())
}

fn remove_existing(parent: &OwnedFd, name: &OsString, expected: &[u8]) -> Result<(), String> {
    let temp = temp_name();
    rustix::fs::renameat_with(parent, name, parent, &temp, RenameFlags::NOREPLACE)
        .map_err(io_error)?;
    match read_at(parent, &temp) {
        Ok(Some(actual)) if actual == expected => {
            rustix::fs::unlinkat(parent, &temp, AtFlags::empty()).map_err(io_error)?;
            rustix::fs::fsync(parent).map_err(io_error)
        }
        result => {
            let restored =
                rustix::fs::renameat_with(parent, &temp, parent, name, RenameFlags::NOREPLACE);
            if let Err(error) = restored {
                let sync = rustix::fs::fsync(parent).err();
                return Err(format!(
                    "effect deletion conflict could not restore target: {error}; sync={sync:?}"
                ));
            }
            rustix::fs::fsync(parent).map_err(io_error)?;
            Err(match result {
                Ok(_) => "effect target changed before deletion".to_string(),
                Err(error) => error,
            })
        }
    }
}

fn stage(parent: &OwnedFd, bytes: &[u8]) -> Result<OsString, String> {
    for _attempt in 0..16 {
        let name = temp_name();
        let flags =
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::CLOEXEC | OFlags::NOFOLLOW;
        match rustix::fs::openat(parent, &name, flags, Mode::RUSR | Mode::WUSR) {
            Ok(fd) => {
                let mut file = std::fs::File::from(fd);
                if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
                    let _cleanup = rustix::fs::unlinkat(parent, &name, AtFlags::empty());
                    return Err(error.to_string());
                }
                return Ok(name);
            }
            Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("effect temporary file names are exhausted".to_string())
}

#[rustfmt::skip]
fn read_at(parent: &OwnedFd, name: &OsString) -> Result<Option<Vec<u8>>, String> {
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    match rustix::fs::openat(parent, name, flags, Mode::empty()) {
        Ok(fd) => {
            let mut file = std::fs::File::from(fd);
            if !file.metadata().map_err(|error| error.to_string())?.is_file() { return Err("effect target is not a regular file".to_string()); }
            let mut bytes = Vec::new(); file.read_to_end(&mut bytes).map_err(|error| error.to_string())?; Ok(Some(bytes))
        }
        Err(rustix::io::Errno::NOENT) => Ok(None), Err(error) => Err(error.to_string()),
    }
}

fn temp_name() -> OsString {
    let ordinal = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    format!(".lkjagent-effect-{}-{ordinal}.tmp", std::process::id()).into()
}

fn io_error(error: rustix::io::Errno) -> String {
    error.to_string()
}
