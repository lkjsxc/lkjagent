use std::ffi::{CStr, CString};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use crate::error::{sha256, EffectError, EffectResult};
use rustix::fd::OwnedFd;
use rustix::fs::{Dir, FileType, Mode, OFlags};

pub const READ_LINES: usize = 200;
pub const LIST_ENTRIES: usize = 200;
pub const TREE_ENTRIES: usize = 150;
pub const SEARCH_HITS: usize = 50;
pub const WORKSPACE_FILES: usize = 500;
pub const WORKSPACE_BYTES: usize = 1_048_576;
pub const LINE_BYTES: usize = 16_384;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryListing { pub entries: Vec<DirectoryEntry>, pub truncated: bool }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry { pub name: String, pub kind: EntryKind }
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind { File, Directory }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult { pub hits: Vec<SearchHit>, pub files_read: usize, pub bytes_read: usize, pub truncated: bool }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit { pub path: String, pub line: usize, pub text: String }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePage { pub path: String, pub revision: String, pub lines: Vec<NumberedLine>, pub total_lines: usize, pub next_line: Option<usize>, pub truncated: bool, pub final_newline: bool }
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberedLine { pub number: usize, pub text: String }

#[rustfmt::skip]
#[derive(Debug)]
pub struct OpenedWorkspace { root: OwnedFd }

#[rustfmt::skip]
impl OpenedWorkspace {
    pub fn open(root: &Path) -> EffectResult<Self> { let flags=OFlags::RDONLY|OFlags::DIRECTORY|OFlags::CLOEXEC|OFlags::NOFOLLOW; Ok(Self{root:rustix::fs::open(root,flags,Mode::empty()).map_err(io_error)?}) }
    pub fn list_directory(&self,path:&str)->EffectResult<DirectoryListing>{let dir=self.open_path(path)?;require_type(&dir,FileType::Directory)?;let (names,truncated)=directory_entries(&dir,LIST_ENTRIES)?;let mut entries=Vec::with_capacity(names.len());for name in names{let target=open_child(&dir,&name)?;let kind=match file_type(&target)?{FileType::RegularFile=>EntryKind::File,FileType::Directory=>EntryKind::Directory,_=>return Err(EffectError::Unsafe("non-regular directory entry".into()))};entries.push(DirectoryEntry{name:name.to_string_lossy().into_owned(),kind});}Ok(DirectoryListing{entries,truncated})}
    pub fn search_text(&self,path:&str,query:&str)->EffectResult<SearchResult>{if query.is_empty()||query.len()>LINE_BYTES{return Err(EffectError::Bound("invalid search query size".into()));}let mut out=SearchResult{hits:Vec::new(),files_read:0,bytes_read:0,truncated:false};search_fd(self.open_path(path)?,path.into(),query,&mut out)?;Ok(out)}
    pub fn read_file(&self,path:&str,first_line:usize,line_count:usize)->EffectResult<FilePage>{if first_line==0||line_count==0||line_count>READ_LINES{return Err(EffectError::Bound("invalid line page".into()));}let bytes=read_regular(self.open_path(path)?,WORKSPACE_BYTES)?;let revision=sha256(&bytes);let text=String::from_utf8(bytes).map_err(|e|EffectError::Utf8(e.to_string()))?;let final_newline=text.ends_with('\n');if text.split('\n').any(|line|line.len()>LINE_BYTES){return Err(EffectError::Bound("line exceeds byte bound".into()));}let mut split=text.split('\n').collect::<Vec<_>>();if final_newline{split.pop();}let total_lines=split.len();let lines=split.iter().enumerate().skip(first_line-1).take(line_count).map(|(i,text)|NumberedLine{number:i+1,text:(*text).into()}).collect::<Vec<_>>();let consumed=first_line.saturating_sub(1).saturating_add(lines.len());let next_line=(consumed<total_lines).then_some(consumed+1);Ok(FilePage{path:path.into(),revision,lines,total_lines,next_line,truncated:next_line.is_some(),final_newline})}
    fn open_path(&self,path:&str)->EffectResult<OwnedFd>{let mut current=rustix::io::dup(&self.root).map_err(io_error)?;for part in valid_parts(path)?{current=open_child(&current,&part)?;}Ok(current)}
}

#[rustfmt::skip]
fn valid_parts(path:&str)->EffectResult<Vec<CString>>{if path.is_empty()||path.len()>4096||path.starts_with('/')||path.contains("//")||path.ends_with('/') {return Err(EffectError::Path("path must be a normalized relative name".into()));}path.split('/').map(|part|{if part=="."||part==".."||part.to_ascii_lowercase().starts_with(".lkjagent"){return Err(EffectError::Path("path contains a forbidden component".into()));}CString::new(part).map_err(|_|EffectError::Path("invalid path component".into()))}).collect()}
fn open_child(parent: &OwnedFd, name: &CStr) -> EffectResult<OwnedFd> {
    let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    rustix::fs::openat(parent, name, flags, Mode::empty()).map_err(|e| {
        if e == rustix::io::Errno::LOOP {
            EffectError::Unsafe("symlink rejected".into())
        } else {
            io_error(e)
        }
    })
}
fn file_type(fd: &OwnedFd) -> EffectResult<FileType> {
    Ok(FileType::from_raw_mode(
        rustix::fs::fstat(fd).map_err(io_error)?.st_mode,
    ))
}
fn require_type(fd: &OwnedFd, expected: FileType) -> EffectResult<()> {
    if file_type(fd)? == expected {
        Ok(())
    } else {
        Err(EffectError::Unsafe("target has an unsafe file type".into()))
    }
}
#[rustfmt::skip]
fn read_regular(fd:OwnedFd,max:usize)->EffectResult<Vec<u8>>{require_type(&fd,FileType::RegularFile)?;let size=usize::try_from(rustix::fs::fstat(&fd).map_err(io_error)?.st_size).map_err(|_|EffectError::Bound("file size is not representable".into()))?;if size>max{return Err(EffectError::Bound("file exceeds byte bound".into()));}let mut bytes=Vec::with_capacity(size);std::fs::File::from(fd).take((max+1)as u64).read_to_end(&mut bytes)?;if bytes.len()>max{return Err(EffectError::Bound("file grew beyond byte bound".into()));}Ok(bytes)}
#[rustfmt::skip]
fn directory_entries(fd:&OwnedFd,limit:usize)->EffectResult<(Vec<CString>,bool)>{let mut dir=Dir::read_from(fd).map_err(io_error)?;let mut names=Vec::with_capacity(limit);let mut truncated=false;while let Some(entry)=dir.read(){let entry=entry.map_err(io_error)?;let raw=entry.file_name();if matches!(raw.to_bytes(),b"."|b".."){continue;}let text=raw.to_str().map_err(|e|EffectError::Utf8(e.to_string()))?;if text.to_ascii_lowercase().starts_with(".lkjagent"){return Err(EffectError::Path("reserved internal name".into()));}let name=CString::new(text).map_err(|_|EffectError::Path("invalid name".into()))?;let kind=file_type(&open_child(fd,&name)?)?;if !matches!(kind,FileType::RegularFile|FileType::Directory){return Err(EffectError::Unsafe("non-regular directory entry".into()));}
if names.len()<limit{names.push(name);names.sort();}else{truncated=true;if names.last().is_some_and(|last|name<*last){names.pop();names.push(name);names.sort();}}}Ok((names,truncated))}
#[rustfmt::skip]
fn search_fd(fd:OwnedFd,path:String,query:&str,out:&mut SearchResult)->EffectResult<()>{if out.files_read>=WORKSPACE_FILES||out.bytes_read>=WORKSPACE_BYTES{out.truncated=true;return Ok(());}match file_type(&fd)?{FileType::RegularFile=>{let remaining=WORKSPACE_BYTES-out.bytes_read;let size=usize::try_from(rustix::fs::fstat(&fd).map_err(io_error)?.st_size).map_err(|_|EffectError::Bound("file size is not representable".into()))?;if size>remaining{out.truncated=true;return Ok(());}let bytes=read_regular(fd,remaining)?;out.files_read+=1;out.bytes_read+=bytes.len();let text=String::from_utf8(bytes).map_err(|e|EffectError::Utf8(e.to_string()))?;for(line,text)in text.split('\n').enumerate(){if text.len()>LINE_BYTES{return Err(EffectError::Bound("line exceeds byte bound".into()));}
if text.contains(query){if out.hits.len()==SEARCH_HITS{out.truncated=true;}else{out.hits.push(SearchHit{path:path.clone(),line:line+1,text:text.into()});}}}Ok(())}FileType::Directory=>{let(names,cut)=directory_entries(&fd,WORKSPACE_FILES)?;out.truncated|=cut;for name in names{let child=format!("{path}/{}",name.to_string_lossy());search_fd(open_child(&fd,&name)?,child,query,out)?;}Ok(())}_=>Err(EffectError::Unsafe("target has an unsafe file type".into()))}}
fn io_error(error: rustix::io::Errno) -> EffectError {
    EffectError::Io(error.to_string())
}

#[rustfmt::skip]
pub fn resolve(root:&Path,path:&str)->EffectResult<PathBuf>{let relative=Path::new(path);if path.trim().is_empty()||relative.is_absolute()||relative.components().any(|p|matches!(p,Component::ParentDir|Component::RootDir|Component::Prefix(_))){return Err(EffectError::Path("path must stay inside workspace".into()));}let root=root.canonicalize()?;let candidate=root.join(relative);let mut check=candidate.clone();while !check.exists(){check=check.parent().ok_or_else(||EffectError::Path("no existing parent".into()))?.to_path_buf();}if !check.canonicalize()?.starts_with(&root){return Err(EffectError::Path("path resolves outside workspace".into()));}Ok(candidate)}
#[rustfmt::skip]
pub fn read(root:&Path,path:&str,offset:usize,count:usize)->EffectResult<String>{let text=fs::read_to_string(resolve(root,path)?)?;let lines=text.lines().collect::<Vec<_>>();let count=if count==0{READ_LINES}else{count.min(READ_LINES)};Ok(format!("path={path} offset={offset} count={count} total={} truncated={}\n{}",lines.len(),offset.saturating_add(count)<lines.len(),lines.into_iter().skip(offset).take(count).collect::<Vec<_>>().join("\n")))}
#[rustfmt::skip]
pub fn write(root:&Path,path:&str,content:&str)->EffectResult<String>{let full=resolve(root,path)?;if let Some(parent)=full.parent(){fs::create_dir_all(parent)?;}fs::write(full,content)?;Ok(format!("path={path}\nbytes={}",content.len()))}
#[rustfmt::skip]
pub fn list(root:&Path,path:&str,depth:usize)->EffectResult<String>{let mut rows=Vec::new();legacy_walk(root,&resolve(root,path)?,0,depth,&mut rows)?;rows.sort();rows.truncate(LIST_ENTRIES);Ok(rows.join("\n"))}
pub fn tree(root: &Path, path: &str, depth: usize) -> EffectResult<String> {
    list(root, path, depth)
}
#[rustfmt::skip]
pub fn search(root:&Path,path:&str,query:&str)->EffectResult<String>{let mut rows=Vec::new();legacy_search(root,&resolve(root,path)?,query,&mut rows)?;rows.truncate(SEARCH_HITS);Ok(rows.join("\n"))}
#[rustfmt::skip]
fn legacy_walk(root:&Path,path:&Path,level:usize,depth:usize,rows:&mut Vec<String>)->EffectResult<()>{let meta=fs::metadata(path)?;let rel=path.strip_prefix(root).map_err(|e|EffectError::Io(e.to_string()))?;rows.push(format!("{} {}",if meta.is_dir(){"dir"}else{"file"},if rel.as_os_str().is_empty(){".".into()}else{rel.display().to_string()}));if meta.is_dir()&&level<depth{for entry in fs::read_dir(path)?.filter_map(Result::ok){legacy_walk(root,&entry.path(),level+1,depth,rows)?;}}Ok(())}
#[rustfmt::skip]
fn legacy_search(root:&Path,path:&Path,query:&str,rows:&mut Vec<String>)->EffectResult<()>{if path.is_dir(){for entry in fs::read_dir(path)?.filter_map(Result::ok){legacy_search(root,&entry.path(),query,rows)?;}}else if let Ok(text)=fs::read_to_string(path){let rel=path.strip_prefix(root).map_err(|e|EffectError::Io(e.to_string()))?;for(line,text)in text.lines().enumerate(){if text.to_ascii_lowercase().contains(&query.to_ascii_lowercase()){rows.push(format!("{}:{}: {text}",rel.display(),line+1));}}}Ok(())}
