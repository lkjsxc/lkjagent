#![cfg(target_os = "linux")]

use lkjagent_app::{cli, endpoint::ScriptedEndpoint, public_loop};
use lkjagent_store::native_schema::{self, NATIVE_TABLES};
use rusqlite::Connection;
use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
#[rustfmt::skip]
fn public_native_loop_exact_edit_second_matter_restart_and_stale_guard()->TestResult {
 let root=fixture()?; let data=root.join("data"); let workspace=root.join("workspace"); fs::create_dir_all(workspace.join("notes"))?;
 fs::write(workspace.join("notes/exact-base.txt"),"alpha is the known phrase\n")?; fs::write(workspace.join("notes/second.txt"),"one is the second phrase\n")?;
 fs::set_permissions(workspace.join("notes/exact-base.txt"),fs::Permissions::from_mode(0o640))?; fs::create_dir_all(&data)?; fs::write(data.join("lkjagent.json"),"{\"workspace_root\":\"../workspace\"}")?;
 let data_arg=data.to_string_lossy(); let first=cli::run(["--data",data_arg.as_ref(),"send","Replace the exact known phrase and verify it."])?; assert!(first.contains("matter=")&&first.contains("turn=")&&first.contains("message=owner-turn/")); assert_eq!(inventory(&workspace)?,vec!["notes/exact-base.txt","notes/second.txt"]);
 let mut endpoint=ScriptedEndpoint{outputs:vec!["<tool_call><tool>list_directory</tool><input><path>notes</path></input></tool".into(),read("notes/exact-base.txt"),edit("notes/exact-base.txt","alpha is the known phrase","beta is the checked phrase"),final_message("Updated the exact phrase with current checks.")],index:0};
 for _ in 0..4 { public_loop::run_once(&data,&mut endpoint)?; }
 assert_eq!(fs::read_to_string(workspace.join("notes/exact-base.txt"))?,"beta is the checked phrase\n"); assert_eq!(fs::metadata(workspace.join("notes/exact-base.txt"))?.mode()&0o7777,0o640); assert_eq!(inventory(&workspace)?,vec!["notes/exact-base.txt","notes/second.txt"]);
 let db=data.join("lkjagent.sqlite3"); let c=Connection::open(&db)?; assert_eq!(counts(&c)?,(1,1,1,3,3,2)); assert_eq!(scalar(&c,"SELECT count(*) FROM matters WHERE lifecycle='closed'")?,1); assert_eq!(scalar(&c,"SELECT count(*) FROM conversation_messages WHERE role='agent' AND receipt IS NOT NULL")?,1); assert_eq!(roles(&c)?,vec![(1,"owner".into()),(2,"agent".into())]); assert_native_only(&c)?;
 let before=counts(&c)?; drop(c); let mut empty=ScriptedEndpoint{outputs:vec![],index:0}; assert!(public_loop::run_once(&data,&mut empty)?.contains("idle")); let c=Connection::open(&db)?; assert_eq!(counts(&c)?,before); drop(c);
 let second=cli::run(["--data",data_arg.as_ref(),"send","Re-read the checked file, make no changes, and report it."])?; assert_ne!(identity(&first,"message="),identity(&second,"message=")); let mut endpoint=ScriptedEndpoint{outputs:vec![read_complete("notes/exact-base.txt"),final_message("The checked phrase remains current.")],index:0}; for _ in 0..2{public_loop::run_once(&data,&mut endpoint)?;}
 assert_eq!(fs::read_to_string(workspace.join("notes/second.txt"))?,"one is the second phrase\n"); let c=Connection::open(&db)?; assert_eq!(scalar(&c,"SELECT count(*) FROM matters WHERE lifecycle='closed'")?,2); assert_eq!(roles(&c)?,vec![(1,"owner".into()),(2,"agent".into()),(3,"owner".into()),(4,"agent".into())]); assert_eq!(scalar(&c,"SELECT count(*) FROM checks WHERE current=1 AND passed=1")?,6); drop(c);
 let stale=cli::run(["--data",data_arg.as_ref(),"send","Make a revision-bound update."])?; assert_ne!(identity(&second,"message="),identity(&stale,"message=")); let mut endpoint=ScriptedEndpoint{outputs:vec![read("notes/second.txt"),edit("notes/second.txt","one is the second phrase","gamma must not win")],index:0}; public_loop::run_once(&data,&mut endpoint)?; fs::write(workspace.join("notes/second.txt"),"owner bytes remain\n")?; let outcome=public_loop::run_once(&data,&mut endpoint)?; assert!(outcome.contains("fault:")); assert_eq!(fs::read_to_string(workspace.join("notes/second.txt"))?,"owner bytes remain\n");
 let c=Connection::open(&db)?; assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,1); assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE CAST(namespace AS TEXT)='recovery' AND CAST(cell_key AS TEXT)='stale' AND status='active'")?,1); assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE CAST(namespace AS TEXT)='recovery' AND CAST(cell_key AS TEXT)='malformed' AND status='suppressed'")?,1); let status=cli::run(["--data",data_arg.as_ref(),"status"])?; assert!(status.contains("roots: data=")&&status.contains("workspace=")&&status.contains("unfinished:")&&status.contains("checks-ready=")); assert!(!status.contains("task:")); Ok(())
}

fn read(path: &str) -> String {
    format!("<tool_call><tool>read_file</tool><input><path>{path}</path></input></tool_call>")
}
fn read_complete(path: &str) -> String {
    format!("<tool_call><tool>read_file</tool><input><path>{path}</path><complete>true</complete></input></tool_call>")
}
fn edit(path: &str, old: &str, new: &str) -> String {
    format!("<tool_call><tool>edit_file</tool><input><path>{path}</path><old_text>{old}</old_text><new_text>{new}</new_text></input></tool_call>")
}
fn final_message(body: &str) -> String {
    format!("<final><message>{body}</message></final>")
}
fn scalar(c: &Connection, sql: &str) -> TestResult<i64> {
    Ok(c.query_row(sql, [], |r| r.get(0))?)
}
#[rustfmt::skip]
fn counts(c:&Connection)->TestResult<(i64,i64,i64,i64,i64,i64)>{Ok(c.query_row("SELECT (SELECT count(*) FROM effect_journal),(SELECT count(*) FROM tool_admissions WHERE effectful=1),(SELECT count(*) FROM workspace_revisions),(SELECT count(*) FROM checks),(SELECT count(*) FROM checks WHERE current=1 AND passed=1),(SELECT count(*) FROM conversation_messages)",[],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?,r.get(5)?)))?)}
#[rustfmt::skip]
fn roles(c:&Connection)->TestResult<Vec<(i64,String)>>{let mut s=c.prepare("SELECT sequence,role FROM conversation_messages ORDER BY sequence")?;let rows=s.query_map([],|r|Ok((r.get(0)?,r.get(1)?)))?.collect::<Result<Vec<_>,_>>()?;Ok(rows)}
#[rustfmt::skip]
fn assert_native_only(c:&Connection)->TestResult{let mut s=c.prepare("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?;let rows=s.query_map([],|r|r.get::<_,String>(0))?.collect::<Result<Vec<_>,_>>()?;assert_eq!(rows,NATIVE_TABLES.iter().map(|x|x.to_string()).collect::<Vec<_>>());assert_eq!(native_schema::conversation(c,None,10)?.len(),2);Ok(())}
fn identity<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.split_whitespace()
        .find_map(|x| x.strip_prefix(prefix))
        .unwrap_or("")
}
#[rustfmt::skip]
fn inventory(root:&Path)->TestResult<Vec<String>>{fn walk(root:&Path,at:&Path,out:&mut Vec<String>)->TestResult{for row in fs::read_dir(at)?{let path=row?.path();if path.is_dir(){walk(root,&path,out)?}else{out.push(path.strip_prefix(root)?.to_string_lossy().into_owned())}}Ok(())}let mut out=vec![];walk(root,root,&mut out)?;out.sort();Ok(out)}
#[rustfmt::skip]
fn fixture()->TestResult<PathBuf>{let n=SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();let root=std::env::temp_dir().join(format!("lkjagent-public-loop-{n}"));if root.exists(){fs::remove_dir_all(&root)?}fs::create_dir_all(&root)?;Ok(root)}
