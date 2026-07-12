#![cfg(unix)]
use lkjagent_effects::workspace::{
    OpenedWorkspace, LIST_ENTRIES, READ_LINES, SEARCH_HITS, WORKSPACE_BYTES,
};
use std::ffi::OsString;
use std::fs;
use std::os::unix::{ffi::OsStringExt, fs::symlink, net::UnixListener};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
type TestResult = Result<(), Box<dyn std::error::Error>>;
static NEXT: AtomicU64 = AtomicU64::new(1);

#[test]
#[rustfmt::skip]
fn workspace_safety_rejects_traversal_reserved_and_all_symlink_positions()->TestResult{
 let root=fixture("paths")?;fs::create_dir(root.join("dir"))?;fs::write(root.join("dir/file"),"safe")?;let outside=fixture("out")?;fs::write(outside.join("file"),"secret")?;symlink(&outside,root.join("link-dir"))?;symlink(outside.join("file"),root.join("link-file"))?;let ws=OpenedWorkspace::open(&root)?;
 for path in ["",".","..","/tmp","dir/../file","dir/./file","dir//file","dir/",".lkjagent",".lkjagent-state/x","link-dir/file","link-file"]{assert!(ws.read_file(path,1,1).is_err(),"accepted {path}");}
 assert!(ws.list_directory("link-dir").is_err());assert!(ws.search_text("link-dir","secret").is_err());Ok(())
}

#[test]
#[rustfmt::skip]
fn workspace_safety_root_rename_replacement_and_symlink_race()->TestResult{
 let root=fixture("root")?;fs::write(root.join("value"),"original")?;let ws=OpenedWorkspace::open(&root)?;let moved=root.with_extension("moved");fs::rename(&root,&moved)?;fs::create_dir(&root)?;fs::write(root.join("value"),"replacement")?;assert_eq!(ws.read_file("value",1,1)?.lines[0].text,"original");let link=root.with_extension("link");symlink(&root,&link)?;assert!(OpenedWorkspace::open(&link).is_err());
 let race=fixture("race")?;let outside=fixture("race-out")?;fs::write(outside.join("file"),"outside")?;fs::create_dir(race.join("live"))?;fs::write(race.join("live/file"),"inside")?;let ws=OpenedWorkspace::open(&race)?;let stop=Arc::new(AtomicBool::new(false));let writer_stop=Arc::clone(&stop);let writer=std::thread::spawn(move||while !writer_stop.load(Ordering::Relaxed){let _=fs::remove_file(race.join("live/file"));let _=fs::remove_dir(race.join("live"));let _=symlink(&outside,race.join("live"));let _=fs::remove_file(race.join("live"));let _=fs::create_dir(race.join("live"));let _=fs::write(race.join("live/file"),"inside");});
 for _ in 0..500{if let Ok(page)=ws.read_file("live/file",1,1){assert_eq!(page.lines[0].text,"inside");}}stop.store(true,Ordering::Relaxed);writer.join().map_err(|_|"race writer panicked")?;Ok(())
}

#[test]
#[rustfmt::skip]
fn workspace_safety_special_files_return_promptly()->TestResult{
 let root=fixture("special")?;assert!(Command::new("mkfifo").arg(root.join("pipe")).status()?.success());let _socket=UnixListener::bind(root.join("socket"))?;let ws=OpenedWorkspace::open(&root)?;let start=Instant::now();assert!(ws.read_file("pipe",1,1).is_err());assert!(ws.read_file("socket",1,1).is_err());assert!(start.elapsed()<Duration::from_secs(1));assert!(OpenedWorkspace::open(std::path::Path::new("/dev"))?.read_file("null",1,1).is_err());Ok(())
}

#[test]
#[rustfmt::skip]
fn workspace_safety_order_and_all_bounds_are_deterministic()->TestResult{
 let root=fixture("bounds")?;fs::create_dir(root.join("many"))?;for i in(0..LIST_ENTRIES+7).rev(){fs::write(root.join(format!("many/file-{i:03}")),"needle\n")?;}let ws=OpenedWorkspace::open(&root)?;let list=ws.list_directory("many")?;assert_eq!(list.entries.len(),LIST_ENTRIES);assert!(list.truncated);assert_eq!(list.entries[0].name,"file-000");assert_eq!(list.entries[LIST_ENTRIES-1].name,"file-199");let search=ws.search_text("many","needle")?;assert_eq!(search.hits.len(),SEARCH_HITS);assert!(search.truncated);assert!(search.hits.windows(2).all(|p|p[0].path<=p[1].path));fs::write(root.join("large"),vec![b'x';WORKSPACE_BYTES+1])?;assert!(ws.read_file("large",1,1).is_err());assert!(ws.search_text("large","x")?.truncated);Ok(())
}

#[test]
#[rustfmt::skip]
fn workspace_safety_numbered_pages_newlines_multibyte_and_revision()->TestResult{
 let root=fixture("read")?;fs::write(root.join("abc"),"abc")?;fs::write(root.join("lines"),"alpha\nβeta\n\n")?;fs::write(root.join("invalid"),[0xff,0xfe])?;let ws=OpenedWorkspace::open(&root)?;let abc=ws.read_file("abc",1,READ_LINES)?;assert_eq!(abc.revision,"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");assert_eq!(abc.revision.len(),64);assert!(abc.revision.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b)));let first=ws.read_file("lines",1,2)?;assert_eq!(first.lines.iter().map(|l|(l.number,l.text.as_str())).collect::<Vec<_>>(),[(1,"alpha"),(2,"βeta")]);assert_eq!(first.total_lines,3);assert_eq!(first.next_line,Some(3));assert!(first.truncated&&first.final_newline);let last=ws.read_file("lines",3,1)?;assert_eq!(last.lines[0].text,"");assert_eq!(last.next_line,None);assert!(!last.truncated);assert!(ws.read_file("invalid",1,1).is_err());assert!(ws.read_file("abc",0,1).is_err());assert!(ws.read_file("abc",1,READ_LINES+1).is_err());Ok(())
}

#[test]
#[rustfmt::skip]
fn workspace_safety_invalid_utf8_names_are_rejected()->TestResult{
 let root=fixture("utf8")?;fs::create_dir(root.join("sub"))?;fs::write(root.join("sub").join(OsString::from_vec(vec![0xff])),"x")?;let ws=OpenedWorkspace::open(&root)?;assert!(ws.list_directory("sub").is_err());assert!(ws.search_text("sub","x").is_err());Ok(())
}

#[rustfmt::skip]
fn fixture(name:&str)->Result<PathBuf,std::io::Error>{let id=NEXT.fetch_add(1,Ordering::Relaxed);let path=std::env::temp_dir().join(format!("lkjagent-workspace-{name}-{}-{id}",std::process::id()));if path.exists(){fs::remove_dir_all(&path)?;}fs::create_dir_all(&path)?;Ok(path)}
