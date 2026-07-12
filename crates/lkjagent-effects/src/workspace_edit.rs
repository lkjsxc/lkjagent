pub use crate::edit_types::*;
use crate::error::EffectError;
use crate::workspace_capability::{
    io_error, open_child, read_regular, require_type, valid_parts, OpenedWorkspace, WORKSPACE_BYTES,
};
use rustix::fd::OwnedFd;
use rustix::fs::{AtFlags, FileType, Mode, OFlags, RenameFlags};
use sha2::{Digest, Sha256};
use std::ffi::{CStr, CString};
use std::io::Write;
impl OpenedWorkspace {
    pub fn observe_edit_target(&self, path: &str) -> EditResult<ObservedTarget> {
        let (parent, name) = self.edit_parent(path)?;
        observe_at(&parent, &name)
    }
    pub fn prepare_exact_edit(
        &self,
        path: String,
        revision: Revision,
        old_text: &str,
        new_text: &str,
        create_mode: u32,
    ) -> EditResult<PreparedEdit> {
        let observed = self.observe_edit_target(&path)?;
        let (prior, mode, intended) = match (revision, observed) {
            (Revision::Absent, ObservedTarget::Absent) if old_text.is_empty() => {
                (None, None, new_text.as_bytes().to_vec())
            }
            (Revision::Sha256(bound), ObservedTarget::Present(value))
                if bound == value.revision =>
            {
                let text = std::str::from_utf8(&value.bytes)
                    .map_err(|error| EffectError::Utf8(error.to_string()))?;
                if old_text.is_empty() || crate::edit_types::exact_matches(text, old_text) != 1 {
                    return Err(EditError::Conflict("old_text must match exactly once"));
                }
                let output = text.replacen(old_text, new_text, 1).into();
                (Some(value.bytes), Some(value.mode), output)
            }
            _ => return Err(EditError::Conflict("bound revision is stale")),
        };
        if intended.len() > WORKSPACE_BYTES {
            return Err(EffectError::Bound("edit exceeds byte bound".into()).into());
        }
        if prior.as_ref() == Some(&intended) || prior.is_none() && intended.is_empty() {
            return Err(EditError::Conflict("edit must change content"));
        }
        let intended_mode = mode.unwrap_or(create_mode & 0o7777);
        let mut hash = Sha256::new();
        hash.update(path.as_bytes());
        hash.update(Sha256::digest(prior.as_deref().unwrap_or_default()));
        hash.update(Sha256::digest(&intended));
        hash.update(intended_mode.to_le_bytes());
        Ok(PreparedEdit {
            path,
            prior_bytes: prior,
            intended_bytes: intended,
            expected_mode: mode,
            intended_mode,
            stage_identity: format!(".lkjagent-edit-{:x}", hash.finalize()),
        })
    }
    pub fn advance_exact_edit(
        &self,
        edit: &PreparedEdit,
        phase: DurablePhase,
    ) -> EditResult<DurablePhase> {
        let (parent, target) = self.edit_parent(&edit.path)?;
        let stage = cstring(&edit.stage_identity)?;
        match phase {
            DurablePhase::Staged => {
                let flags = OFlags::WRONLY
                    | OFlags::CLOEXEC
                    | OFlags::NOFOLLOW
                    | OFlags::CREATE
                    | OFlags::EXCL;
                match rustix::fs::openat(&parent, &stage, flags, Mode::empty()) {
                    Ok(fd) => {
                        rustix::fs::fchmod(&fd, Mode::from_raw_mode(edit.intended_mode))
                            .map_err(io_error)?;
                        let mut file = std::fs::File::from(fd);
                        file.write_all(&edit.intended_bytes)
                            .map_err(EffectError::from)?;
                        file.sync_all().map_err(EffectError::from)?;
                    }
                    Err(error) if error == rustix::io::Errno::EXIST => {}
                    Err(error) => return Err(io_error(error).into()),
                }
            }
            DurablePhase::Exchanged => {
                self.require_layout(edit, Layout::Staged)?;
                rename(&parent, &stage, &target, edit, false)?;
            }
            DurablePhase::Settled => self.require_layout(edit, Layout::Exchanged)?,
            DurablePhase::Compensated => {
                self.require_layout(edit, Layout::Exchanged)?;
                rename(&parent, &target, &stage, edit, true)?;
            }
            DurablePhase::Cleaned => return Err(EditError::Conflict("cleanup needs outcome")),
        }
        rustix::fs::fsync(&parent).map_err(io_error)?;
        let wanted = match phase {
            DurablePhase::Staged | DurablePhase::Compensated => Layout::Staged,
            DurablePhase::Exchanged | DurablePhase::Settled => Layout::Exchanged,
            DurablePhase::Cleaned => return Err(EditError::Conflict("cleanup needs outcome")),
        };
        self.require_layout(edit, wanted)?;
        Ok(phase)
    }
    pub fn cleanup_exact_edit(
        &self,
        edit: &PreparedEdit,
        outcome: VerifiedOutcome,
    ) -> EditResult<DurablePhase> {
        let wanted = match outcome {
            VerifiedOutcome::Settled => Layout::Exchanged,
            VerifiedOutcome::Compensated => Layout::Staged,
        };
        self.require_layout(edit, wanted)?;
        let (parent, _) = self.edit_parent(&edit.path)?;
        let stage = cstring(&edit.stage_identity)?;
        if observe_at(&parent, &stage)? != ObservedTarget::Absent {
            rustix::fs::unlinkat(&parent, &stage, AtFlags::empty()).map_err(io_error)?;
            rustix::fs::fsync(&parent).map_err(io_error)?;
        }
        Ok(DurablePhase::Cleaned)
    }
    fn require_layout(&self, edit: &PreparedEdit, wanted: Layout) -> EditResult<()> {
        let (parent, target) = self.edit_parent(&edit.path)?;
        let stage = cstring(&edit.stage_identity)?;
        let actual = classify(
            edit,
            &observe_at(&parent, &target)?,
            &observe_at(&parent, &stage)?,
        );
        if actual == wanted {
            Ok(())
        } else {
            Err(EditError::Conflict("workspace layout changed"))
        }
    }
    fn edit_parent(&self, path: &str) -> EditResult<(OwnedFd, CString)> {
        let mut parts = valid_parts(path)?;
        let name = parts
            .pop()
            .ok_or(EditError::Conflict("file path required"))?;
        let mut parent = rustix::io::dup(&self.root).map_err(io_error)?;
        for part in parts {
            parent = open_child(&parent, &part)?;
            require_type(&parent, FileType::Directory)?;
        }
        Ok((parent, name))
    }
}

fn observe_at(parent: &OwnedFd, name: &CStr) -> EditResult<ObservedTarget> {
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    let fd = match rustix::fs::openat(parent, name, flags, Mode::empty()) {
        Ok(fd) => fd,
        Err(error) if error == rustix::io::Errno::NOENT => return Ok(ObservedTarget::Absent),
        Err(error) => return Err(io_error(error).into()),
    };
    require_type(&fd, FileType::RegularFile)?;
    let mode = rustix::fs::fstat(&fd).map_err(io_error)?.st_mode & 0o7777;
    let bytes = read_regular(fd, WORKSPACE_BYTES)?;
    let revision = format!("{:x}", Sha256::digest(&bytes));
    Ok(ObservedTarget::Present(FileValue {
        bytes,
        revision,
        mode,
    }))
}
fn rename(
    parent: &OwnedFd,
    from: &CStr,
    to: &CStr,
    edit: &PreparedEdit,
    reverse: bool,
) -> EditResult<()> {
    let flags = if edit.prior_bytes.is_some() {
        RenameFlags::EXCHANGE
    } else {
        RenameFlags::NOREPLACE
    };
    rustix::fs::renameat_with(parent, from, parent, to, flags).map_err(|error| match error {
        rustix::io::Errno::NOSYS | rustix::io::Errno::INVAL | rustix::io::Errno::OPNOTSUPP => {
            EditError::Unsupported
        }
        rustix::io::Errno::EXIST if reverse => {
            EditError::Conflict("owner value blocks compensation")
        }
        _ => EditError::Effect(io_error(error)),
    })
}
fn cstring(value: &str) -> Result<CString, EffectError> {
    CString::new(value).map_err(|_| EffectError::Path("invalid path component".into()))
}
