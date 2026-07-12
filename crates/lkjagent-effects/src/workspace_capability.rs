use crate::error::{EffectError, EffectResult};
use rustix::fd::OwnedFd;
use rustix::fs::{FileType, Mode, OFlags};
use sha2::{Digest, Sha256};
use std::ffi::{CStr, CString};
use std::io::Read;
use std::path::Path;
pub const READ_LINES: usize = 200;
pub const LIST_ENTRIES: usize = 200;
pub const SEARCH_HITS: usize = 50;
pub const WORKSPACE_FILES: usize = 500;
pub const WORKSPACE_BYTES: usize = 1_048_576;
pub const LINE_BYTES: usize = 16_384;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryListing {
    pub entries: Vec<DirectoryEntry>,
    pub next_offset: Option<usize>,
    pub truncated: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub kind: EntryKind,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub hits: Vec<SearchHit>,
    pub files_read: usize,
    pub bytes_read: usize,
    pub truncated: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    pub path: String,
    pub line: usize,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePage {
    pub path: String,
    pub revision: String,
    pub lines: Vec<NumberedLine>,
    pub total_lines: usize,
    pub next_line: Option<usize>,
    pub truncated: bool,
    pub final_newline: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberedLine {
    pub number: usize,
    pub text: String,
}
#[derive(Debug)]
pub struct OpenedWorkspace {
    root: OwnedFd,
}
impl OpenedWorkspace {
    pub fn open(root: &Path) -> EffectResult<Self> {
        let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW;
        let root = rustix::fs::open(root, flags, Mode::empty()).map_err(io_error)?;
        Ok(Self { root })
    }
    pub fn read_file(
        &self,
        path: &str,
        first_line: usize,
        line_count: usize,
    ) -> EffectResult<FilePage> {
        if first_line == 0 || line_count == 0 || line_count > READ_LINES {
            return Err(EffectError::Bound("invalid line page".into()));
        }
        let bytes = read_regular(self.open_path(path)?, WORKSPACE_BYTES)?;
        let revision = format!("{:x}", Sha256::digest(&bytes));
        let text =
            String::from_utf8(bytes).map_err(|error| EffectError::Utf8(error.to_string()))?;
        let final_newline = text.ends_with('\n');
        if text.split('\n').any(|line| line.len() > LINE_BYTES) {
            return Err(EffectError::Bound("line exceeds byte bound".into()));
        }
        let mut split = if text.is_empty() {
            Vec::new()
        } else {
            text.split('\n').collect::<Vec<_>>()
        };
        if final_newline {
            split.pop();
        }
        let total_lines = split.len();
        let lines = split
            .iter()
            .enumerate()
            .skip(first_line - 1)
            .take(line_count)
            .map(|(index, text)| NumberedLine {
                number: index + 1,
                text: (*text).into(),
            })
            .collect::<Vec<_>>();
        let consumed = first_line.saturating_sub(1).saturating_add(lines.len());
        let next_line = (consumed < total_lines).then_some(consumed + 1);
        Ok(FilePage {
            path: path.into(),
            revision,
            lines,
            total_lines,
            next_line,
            truncated: next_line.is_some(),
            final_newline,
        })
    }
    pub(crate) fn open_path(&self, path: &str) -> EffectResult<OwnedFd> {
        let mut current = rustix::io::dup(&self.root).map_err(io_error)?;
        for part in valid_parts(path)? {
            current = open_child(&current, &part)?;
        }
        Ok(current)
    }
}
fn valid_parts(path: &str) -> EffectResult<Vec<CString>> {
    if path == "." {
        return Ok(Vec::new());
    }
    if path.is_empty()
        || path.len() > 4096
        || path.starts_with('/')
        || path.contains("//")
        || path.ends_with('/')
    {
        return Err(EffectError::Path(
            "path must be a normalized relative name".into(),
        ));
    }
    path.split('/')
        .map(|part| {
            if part == "." || part == ".." || part.to_ascii_lowercase().starts_with(".lkjagent") {
                return Err(EffectError::Path(
                    "path contains a forbidden component".into(),
                ));
            }
            CString::new(part).map_err(|_| EffectError::Path("invalid path component".into()))
        })
        .collect()
}
pub(crate) fn open_child(parent: &OwnedFd, name: &CStr) -> EffectResult<OwnedFd> {
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    rustix::fs::openat(parent, name, flags, Mode::empty()).map_err(|error| {
        if error == rustix::io::Errno::LOOP {
            EffectError::Unsafe("symlink rejected".into())
        } else {
            io_error(error)
        }
    })
}
pub(crate) fn file_type(fd: &OwnedFd) -> EffectResult<FileType> {
    Ok(FileType::from_raw_mode(
        rustix::fs::fstat(fd).map_err(io_error)?.st_mode,
    ))
}
pub(crate) fn require_type(fd: &OwnedFd, expected: FileType) -> EffectResult<()> {
    if file_type(fd)? == expected {
        Ok(())
    } else {
        Err(EffectError::Unsafe("target has an unsafe file type".into()))
    }
}
pub(crate) fn read_regular(fd: OwnedFd, max: usize) -> EffectResult<Vec<u8>> {
    require_type(&fd, FileType::RegularFile)?;
    let size = usize::try_from(rustix::fs::fstat(&fd).map_err(io_error)?.st_size)
        .map_err(|_| EffectError::Bound("file size is not representable".into()))?;
    if size > max {
        return Err(EffectError::Bound("file exceeds byte bound".into()));
    }
    let mut bytes = Vec::with_capacity(size);
    std::fs::File::from(fd)
        .take((max + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > max {
        return Err(EffectError::Bound("file grew beyond byte bound".into()));
    }
    Ok(bytes)
}
fn io_error(error: rustix::io::Errno) -> EffectError {
    EffectError::Io(error.to_string())
}
