#![cfg(target_os = "linux")]
use lkjagent_app::{
    cli,
    endpoint::{CompletionRecord, Endpoint, ScriptedEndpoint},
    public_loop,
};
use lkjagent_core::prompt::Prompt;
use lkjagent_store::native_schema::NATIVE_TABLES;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
type T<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
#[rustfmt::skip]
fn public_scripted_journal_create_replace_lineage_checks_and_stale_guard()->T{
 let root=fixture("flow")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;write_config(&data,"+14:00")?;send(&data,"Record a grounded journal reflection about my owner-provided walk.")?;
 let mut first=ChangingEndpoint{data:data.clone(),workspace_file:None,output:record("First grounded day","The owner said they took a measured walk today."),timezone:Some("-14:00")};public_loop::run_once(&data,&mut first)?;
 let db=data.join("lkjagent.sqlite3");let connection=Connection::open(&db)?;let context=selected_context(&connection,1)?;let date=context["local_date"].as_str().ok_or("date")?.to_string();let wall=context["selected_wall_time"].as_str().ok_or("wall")?;assert_ne!(date,lkjagent_app::clock::local_date(wall,"-14:00")?);let path=journal_path(&date);let file=workspace.join(&path);let text=fs::read_to_string(&file)?;
 assert!(text.contains("kind: journal")&&text.contains(&format!("date: {date}"))&&text.contains("# First grounded day"));assert_eq!(scalar(&connection,"SELECT count(*) FROM effect_targets WHERE operation='mkdir'")?,5);assert_eq!(scalar(&connection,"SELECT count(*) FROM checks WHERE current=1 AND passed=1")?,4);assert_eq!(scalar(&connection,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-journal' AND status='passed'")?,1);
 let source:String=connection.query_row("SELECT CAST(source_revision AS TEXT) FROM context_items WHERE source_kind='owner' LIMIT 1",[],|row|row.get(0))?;assert!(text.contains(&format!("- {source}")));assert_native_only(&connection)?;drop(connection);let mut final_endpoint=ScriptedEndpoint{outputs:vec![final_message("Recorded the checked journal path.")],index:0};public_loop::run_once(&data,&mut final_endpoint)?;
 write_config(&data,"+14:00")?;send(&data,"Update today's journal with another grounded owner fact.")?;let mut replacement=ScriptedEndpoint{outputs:vec![record("Updated grounded day","The owner added a second exact fact for the same day."),final_message("Updated the checked journal path.")],index:0};public_loop::run_once(&data,&mut replacement)?;public_loop::run_once(&data,&mut replacement)?;
 let connection=Connection::open(&db)?;assert_eq!(scalar(&connection,"SELECT count(*) FROM workspace_documents")?,1);assert_eq!(scalar(&connection,"SELECT count(*) FROM workspace_revisions")?,2);assert_eq!(scalar(&connection,"SELECT count(*) FROM workspace_revisions WHERE parent_id IS NOT NULL")?,1);assert_eq!(scalar(&connection,"SELECT count(*) FROM effect_journal WHERE status='settled'")?,2);assert_eq!(inventory(&workspace)?,vec![path.clone()]);let (receipt,body):(Vec<u8>,Vec<u8>)=connection.query_row("SELECT receipt,body FROM conversation_messages WHERE role='agent' ORDER BY sequence DESC LIMIT 1",[],|row|Ok((row.get(0)?,row.get(1)?)))?;let receipt=String::from_utf8(receipt)?;let body=String::from_utf8(body)?;assert!(receipt.contains(&path)&&receipt.contains("revision")&&body.contains(&format!("Checked: {path}@")));drop(connection);
 send(&data,"Revise today's journal while preserving any owner change.")?;let mut stale=ChangingEndpoint{data:data.clone(),workspace_file:Some(file.clone()),output:record("Must not win","This model body must not replace newer owner bytes."),timezone:None};public_loop::run_once(&data,&mut stale)?;assert_eq!(fs::read_to_string(&file)?,"owner bytes win\n");let connection=Connection::open(&db)?;assert_eq!(scalar(&connection,"SELECT count(*) FROM effect_journal")?,2);assert_eq!(scalar(&connection,"SELECT count(*) FROM workspace_revisions")?,2);Ok(())
}

#[test]
#[rustfmt::skip]
fn unmanaged_collision_and_oversize_record_mutate_nothing()->T{
 let root=fixture("reject")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;write_config(&data,"UTC")?;let date=lkjagent_app::clock::local_date(&lkjagent_app::clock::utc_now(),"UTC")?;let path=journal_path(&date);fs::create_dir_all(workspace.join(Path::new(&path).parent().ok_or("parent")?))?;fs::write(workspace.join(&path),"unmanaged owner bytes\n")?;
 send(&data,"Write today's grounded journal.")?;let mut endpoint=ScriptedEndpoint{outputs:vec![record("Collision","This must not replace unmanaged owner bytes.")],index:0};public_loop::run_once(&data,&mut endpoint)?;assert_eq!(fs::read_to_string(workspace.join(&path))?,"unmanaged owner bytes\n");let db=data.join("lkjagent.sqlite3");let connection=Connection::open(&db)?;assert_eq!(scalar(&connection,"SELECT count(*) FROM effect_journal")?,0);drop(connection);
 let second=fixture("oversize")?;let data=second.join("data");fs::create_dir_all(&data)?;write_config(&data,"UTC")?;send(&data,"Write a bounded journal.")?;let body="grounded ".repeat(170);let mut endpoint=ScriptedEndpoint{outputs:vec![record("Bounded",&body)],index:0};public_loop::run_once(&data,&mut endpoint)?;let connection=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&connection,"SELECT count(*) FROM effect_journal")?,0);assert!(!second.join("workspace").exists());Ok(())
}

struct ChangingEndpoint {
    data: PathBuf,
    workspace_file: Option<PathBuf>,
    output: String,
    timezone: Option<&'static str>,
}
#[allow(clippy::possible_missing_else)]
#[rustfmt::skip]
impl Endpoint for ChangingEndpoint{fn complete(&mut self,_:&Prompt,_:u32)->Result<CompletionRecord,String>{if let Some(zone)=self.timezone{write_config(&self.data,zone).map_err(|error|error.to_string())?;}if let Some(path)=&self.workspace_file{fs::write(path,"owner bytes win\n").map_err(|error|error.to_string())?;}Ok(CompletionRecord::scripted(self.output.clone()))}}
fn write_config(data: &Path, zone: &str) -> std::io::Result<()> {
    fs::write(
        data.join("lkjagent.json"),
        format!("{{\"workspace_root\":\"../workspace\",\"workspace_timezone\":\"{zone}\"}}"),
    )
}
fn send(data: &Path, text: &str) -> T<String> {
    let value = data.to_string_lossy();
    Ok(cli::run(["--data", value.as_ref(), "send", text])?)
}
fn record(title: &str, body: &str) -> String {
    format!("<tool_call><tool>write_record</tool><input><family>journal</family><title>{title}</title><body>{body}</body></input></tool_call>")
}
fn final_message(body: &str) -> String {
    format!("<final><message>{body}</message></final>")
}
fn selected_context(connection: &Connection, n: i64) -> T<serde_json::Value> {
    let bytes:Vec<u8>=connection.query_row("SELECT context_spec FROM runtime_decisions ORDER BY selected_monotonic_ms,id LIMIT 1 OFFSET ?1",[n-1],|row|row.get(0))?;
    Ok(serde_json::from_slice(&bytes)?)
}
fn journal_path(date: &str) -> String {
    format!("life/journal/{}/entry.md", date.replace('-', "/"))
}
fn scalar(connection: &Connection, sql: &str) -> T<i64> {
    Ok(connection.query_row(sql, [], |row| row.get(0))?)
}
#[rustfmt::skip]
fn assert_native_only(connection:&Connection)->T{let mut query=connection.prepare("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?;let rows=query.query_map([],|row|row.get::<_,String>(0))?.collect::<Result<Vec<_>,_>>()?;assert_eq!(rows,NATIVE_TABLES.iter().map(|value|value.to_string()).collect::<Vec<_>>());Ok(())}
#[rustfmt::skip]
fn inventory(root:&Path)->T<Vec<String>>{fn walk(root:&Path,at:&Path,out:&mut Vec<String>)->T{for row in fs::read_dir(at)?{let path=row?.path();if path.is_dir(){walk(root,&path,out)?}else{out.push(path.strip_prefix(root)?.to_string_lossy().into_owned())}}Ok(())}let mut out=vec![];walk(root,root,&mut out)?;out.sort();Ok(out)}
fn fixture(name: &str) -> T<PathBuf> {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("lkjagent-journal-{name}-{n}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
