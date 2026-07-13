use crate::error::EffectError;
use crate::workspace_capability::{
    file_type, io_error, open_child, require_type, valid_parts, OpenedWorkspace, WORKSPACE_BYTES,
};
use crate::workspace_edit::{EditError, EditResult, PreparedEdit, PreparedPathCreate};
use rustix::fd::OwnedFd;
use rustix::fs::{Dir, FileType, Mode};
use sha2::{Digest, Sha256};
use std::ffi::{CStr, CString};

impl OpenedWorkspace {
    pub fn prepare_absent_edit(
        &self,
        path: String,
        intended: &str,
        create_mode: u32,
    ) -> EditResult<PreparedPathCreate> {
        if intended.is_empty() {
            return Err(EditError::Conflict("create content must not be empty"));
        }
        if intended.len() > WORKSPACE_BYTES {
            return Err(EffectError::Bound("edit exceeds byte bound".into()).into());
        }
        let mut parts = valid_parts(&path)?;
        let target = parts
            .pop()
            .ok_or(EditError::Conflict("file path required"))?;
        let mut parent = rustix::io::dup(&self.root).map_err(io_error)?;
        let mut prefix = Vec::new();
        let mut missing = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            prefix.push(part.to_string_lossy().into_owned());
            match matching_name(&parent, part)? {
                NameState::Absent => {
                    missing.push(prefix.join("/"));
                    for rest in &parts[index + 1..] {
                        prefix.push(rest.to_string_lossy().into_owned());
                        missing.push(prefix.join("/"));
                    }
                    break;
                }
                NameState::Exact => {
                    parent = open_child(&parent, part)?;
                    require_type(&parent, FileType::Directory)?;
                }
                NameState::Collision => return Err(EditError::Conflict("case collision")),
            }
        }
        if missing.is_empty() {
            match matching_name(&parent, &target)? {
                NameState::Absent => {}
                NameState::Exact => return Err(EditError::Conflict("bound revision is stale")),
                NameState::Collision => return Err(EditError::Conflict("case collision")),
            }
        }
        let bytes = intended.as_bytes().to_vec();
        let mode = create_mode & 0o7777;
        let mut hash = Sha256::new();
        hash.update(path.as_bytes());
        hash.update(Sha256::digest([]));
        hash.update(Sha256::digest(&bytes));
        hash.update(mode.to_le_bytes());
        Ok(PreparedPathCreate {
            edit: PreparedEdit {
                path,
                prior_bytes: None,
                intended_bytes: bytes,
                expected_mode: None,
                intended_mode: mode,
                stage_identity: format!(".lkjagent-edit-{:x}", hash.finalize()),
            },
            missing_parents: missing,
        })
    }

    pub fn create_declared_directories(&self, paths: &[String]) -> EditResult<()> {
        for path in paths {
            let mut parts = valid_parts(path)?;
            let name = parts
                .pop()
                .ok_or(EditError::Conflict("directory path required"))?;
            let parent = open_directory_chain(&self.root, &parts)?;
            if matching_name(&parent, &name)? != NameState::Absent {
                return Err(EditError::Conflict("declared directory collided"));
            }
            rustix::fs::mkdirat(&parent, &name, Mode::from_raw_mode(0o755)).map_err(io_error)?;
            rustix::fs::fsync(&parent).map_err(io_error)?;
            let created = open_child(&parent, &name)?;
            require_type(&created, FileType::Directory)?;
            rustix::fs::fsync(&created).map_err(io_error)?;
        }
        Ok(())
    }
}

fn open_directory_chain(root: &OwnedFd, parts: &[CString]) -> EditResult<OwnedFd> {
    let mut parent = rustix::io::dup(root).map_err(io_error)?;
    for part in parts {
        parent = open_child(&parent, part)?;
        require_type(&parent, FileType::Directory)?;
    }
    Ok(parent)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NameState {
    Absent,
    Exact,
    Collision,
}

fn matching_name(parent: &OwnedFd, wanted: &CStr) -> EditResult<NameState> {
    let wanted = wanted
        .to_str()
        .map_err(|error| EffectError::Utf8(error.to_string()))?;
    let mut dir = Dir::read_from(parent).map_err(io_error)?;
    while let Some(entry) = dir.read() {
        let entry = entry.map_err(io_error)?;
        let name = entry
            .file_name()
            .to_str()
            .map_err(|error| EffectError::Utf8(error.to_string()))?;
        if name == wanted {
            let child = CString::new(name).map_err(|_| EffectError::Path("invalid name".into()))?;
            let _ = file_type(&open_child(parent, &child)?)?;
            return Ok(NameState::Exact);
        }
        if name.eq_ignore_ascii_case(wanted) {
            return Ok(NameState::Collision);
        }
    }
    Ok(NameState::Absent)
}
