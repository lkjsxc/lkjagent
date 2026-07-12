use std::ffi::CString;

use rustix::fd::OwnedFd;
use rustix::fs::{Dir, FileType};

use crate::error::{EffectError, EffectResult};
use crate::workspace_capability::{
    file_type, open_child, read_regular, require_type, DirectoryEntry, DirectoryListing, EntryKind,
    OpenedWorkspace, SearchHit, SearchResult, LINE_BYTES, LIST_ENTRIES, SEARCH_HITS,
    WORKSPACE_BYTES, WORKSPACE_FILES,
};

impl OpenedWorkspace {
    pub fn list_directory(
        &self,
        path: &str,
        offset: usize,
        count: usize,
    ) -> EffectResult<DirectoryListing> {
        if offset > WORKSPACE_FILES || count == 0 || count > LIST_ENTRIES {
            return Err(EffectError::Bound("invalid directory page".into()));
        }
        let dir = self.open_path(path)?;
        require_type(&dir, FileType::Directory)?;
        let names = directory_entries(&dir)?;
        let entries = names
            .iter()
            .skip(offset)
            .take(count)
            .map(|name| directory_entry(&dir, name))
            .collect::<EffectResult<Vec<_>>>()?;
        let consumed = offset.saturating_add(entries.len());
        let next_offset = (consumed < names.len()).then_some(consumed);
        Ok(DirectoryListing {
            entries,
            next_offset,
            truncated: next_offset.is_some(),
        })
    }

    pub fn search_text(&self, path: &str, query: &str) -> EffectResult<SearchResult> {
        if query.is_empty() || query.len() > LINE_BYTES {
            return Err(EffectError::Bound("invalid search query size".into()));
        }
        let mut result = SearchResult {
            hits: Vec::new(),
            files_read: 0,
            bytes_read: 0,
            truncated: false,
        };
        let display_path = if path == "." {
            String::new()
        } else {
            path.into()
        };
        search_fd(self.open_path(path)?, display_path, query, &mut result)?;
        Ok(result)
    }
}

fn directory_entry(dir: &OwnedFd, name: &CString) -> EffectResult<DirectoryEntry> {
    let target = open_child(dir, name)?;
    let kind = match file_type(&target)? {
        FileType::RegularFile => EntryKind::File,
        FileType::Directory => EntryKind::Directory,
        _ => {
            return Err(EffectError::Unsafe("non-regular directory entry".into()));
        }
    };
    Ok(DirectoryEntry {
        name: name.to_string_lossy().into_owned(),
        kind,
    })
}

fn directory_entries(fd: &OwnedFd) -> EffectResult<Vec<CString>> {
    let mut dir = Dir::read_from(fd).map_err(io_error)?;
    let mut names = Vec::new();
    while let Some(entry) = dir.read() {
        let entry = entry.map_err(io_error)?;
        let raw = entry.file_name();
        if matches!(raw.to_bytes(), b"." | b"..") {
            continue;
        }
        if names.len() == WORKSPACE_FILES {
            return Err(EffectError::Bound("directory exceeds entry bound".into()));
        }
        let text = raw
            .to_str()
            .map_err(|error| EffectError::Utf8(error.to_string()))?;
        if text.to_ascii_lowercase().starts_with(".lkjagent") {
            return Err(EffectError::Path("reserved internal name".into()));
        }
        let name = CString::new(text).map_err(|_| EffectError::Path("invalid name".into()))?;
        let kind = file_type(&open_child(fd, &name)?)?;
        if !matches!(kind, FileType::RegularFile | FileType::Directory) {
            return Err(EffectError::Unsafe("non-regular directory entry".into()));
        }
        names.push(name);
    }
    names.sort();
    Ok(names)
}

fn search_fd(
    fd: OwnedFd,
    path: String,
    query: &str,
    result: &mut SearchResult,
) -> EffectResult<()> {
    if result.files_read >= WORKSPACE_FILES || result.bytes_read >= WORKSPACE_BYTES {
        result.truncated = true;
        return Ok(());
    }
    match file_type(&fd)? {
        FileType::RegularFile => search_file(fd, path, query, result),
        FileType::Directory => {
            for name in directory_entries(&fd)? {
                let name_text = name.to_string_lossy();
                let child_path = if path.is_empty() {
                    name_text.into_owned()
                } else {
                    format!("{path}/{name_text}")
                };
                search_fd(open_child(&fd, &name)?, child_path, query, result)?;
            }
            Ok(())
        }
        _ => Err(EffectError::Unsafe("target has an unsafe file type".into())),
    }
}

fn search_file(
    fd: OwnedFd,
    path: String,
    query: &str,
    result: &mut SearchResult,
) -> EffectResult<()> {
    let remaining = WORKSPACE_BYTES - result.bytes_read;
    let size = usize::try_from(rustix::fs::fstat(&fd).map_err(io_error)?.st_size)
        .map_err(|_| EffectError::Bound("file size is not representable".into()))?;
    if size > remaining {
        result.truncated = true;
        return Ok(());
    }
    let bytes = read_regular(fd, remaining)?;
    result.files_read += 1;
    result.bytes_read += bytes.len();
    let text = String::from_utf8(bytes).map_err(|error| EffectError::Utf8(error.to_string()))?;
    for (line, text) in text.split('\n').enumerate() {
        if text.len() > LINE_BYTES {
            return Err(EffectError::Bound("line exceeds byte bound".into()));
        }
        if text.contains(query) {
            if result.hits.len() == SEARCH_HITS {
                result.truncated = true;
            } else {
                result.hits.push(SearchHit {
                    path: path.clone(),
                    line: line + 1,
                    text: text.into(),
                });
            }
        }
    }
    Ok(())
}

fn io_error(error: rustix::io::Errno) -> EffectError {
    EffectError::Io(error.to_string())
}
