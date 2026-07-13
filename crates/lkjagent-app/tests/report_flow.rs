#![cfg(target_os = "linux")]
use lkjagent_app::{
    cli,
    endpoint::{CompletionRecord, Endpoint, ScriptedEndpoint},
    public_loop,
};
use lkjagent_core::prompt::Prompt;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
type T<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
#[rustfmt::skip]
fn report_create_replace_lineage_receipt_and_stale_guard_are_exact()->T{
 let root=fixture("flow")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;
 send(&data,"Remember that the saved project color is amber.")?;let mut memory=ScriptedEndpoint{outputs:vec![memory_record("Project Color","The saved project color is amber."),final_message("Saved checked memory.")],index:0};public_loop::run_once(&data,&mut memory)?;public_loop::run_once(&data,&mut memory)?;
 send(&data,"Write one checked short report using the saved project color fact.")?;let mut report=ScriptedEndpoint{outputs:vec![report_record("Project Color Status","The saved project color remains amber in this short checked report."),final_message("Saved checked report.")],index:0};public_loop::run_once(&data,&mut report)?;
 let db=data.join("lkjagent.sqlite3");let path="artifacts/reports/project-color-status.md";let text=fs::read_to_string(workspace.join(path))?;assert!(text.contains("kind: report")&&text.contains("semantic-key: project-color-status")&&text.contains("# Project Color Status"));
 let c=Connection::open(&db)?;let decision:String=c.query_row("SELECT decision_id FROM effect_journal ORDER BY rowid DESC LIMIT 1",[],|r|r.get(0))?;assert_eq!(scalar(&c,"SELECT count(*) FROM context_items WHERE decision_id=(SELECT decision_id FROM effect_journal ORDER BY rowid DESC LIMIT 1) AND source_kind='memory'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-report' AND status='passed'")?,1);assert_eq!(inventory(&workspace)?,vec!["artifacts/reports/project-color-status.md","knowledge/notes/project-color.md"]);assert_lineage(&c,&decision,&text)?;drop(c);public_loop::run_once(&data,&mut report)?;
 let c=Connection::open(&db)?;let (receipt,body):(Vec<u8>,Vec<u8>)=c.query_row("SELECT receipt,body FROM conversation_messages WHERE role='agent' ORDER BY sequence DESC LIMIT 1",[],|r|Ok((r.get(0)?,r.get(1)?)))?;let receipt=String::from_utf8(receipt)?;let body=String::from_utf8(body)?;assert!(receipt.contains(path)&&receipt.contains("source_lineage")&&body.contains(&format!("Checked: {path}@")));drop(c);
 send(&data,"Replace the checked short report with the current saved color fact.")?;let mut replace=ScriptedEndpoint{outputs:vec![report_record("Project Color Status","The saved project color is still amber after the checked replacement."),final_message("Updated checked report.")],index:0};public_loop::run_once(&data,&mut replace)?;public_loop::run_once(&data,&mut replace)?;
 let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM workspace_documents")?,2);assert_eq!(scalar(&c,"SELECT count(*) FROM workspace_revisions")?,3);assert_eq!(scalar1(&c,"SELECT count(*) FROM workspace_revisions WHERE document_id=(SELECT id FROM workspace_documents WHERE CAST(current_path AS TEXT)=?1)",[path])?,2);assert_eq!(inventory(&workspace)?,vec!["artifacts/reports/project-color-status.md","knowledge/notes/project-color.md"]);drop(c);
 send(&data,"Revise the checked short report while preserving any owner change.")?;let file=workspace.join(path);let mut stale=Mutate{path:file.clone(),output:report_record("Project Color Status","Model bytes must not replace newer owner report bytes.")};public_loop::run_once(&data,&mut stale)?;assert_eq!(fs::read_to_string(file)?,"owner bytes win\n");let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,3);Ok(())
}

#[test]
#[rustfmt::skip]
fn report_collision_oversize_and_empty_slug_never_mutate()->T{
 let root=fixture("collision")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;fs::create_dir_all(workspace.join("artifacts/reports"))?;let file=workspace.join("artifacts/reports/collision.md");fs::write(&file,"owner bytes\n")?;
 send(&data,"Write one checked short report safely.")?;let mut collision=ScriptedEndpoint{outputs:vec![report_record("Collision","This must not replace unmanaged owner bytes.")],index:0};public_loop::run_once(&data,&mut collision)?;assert_eq!(fs::read_to_string(&file)?,"owner bytes\n");let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);drop(c);
 let second=fixture("oversize")?;let data2=second.join("data");fs::create_dir_all(&data2)?;config(&data2)?;send(&data2,"Write a bounded checked short report.")?;let mut huge=ScriptedEndpoint{outputs:vec![report_record("Bounded Report",&"grounded ".repeat(170))],index:0};public_loop::run_once(&data2,&mut huge)?;let c=Connection::open(data2.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);assert!(!second.join("workspace").exists());drop(c);
 let third=fixture("slug")?;let data3=third.join("data");fs::create_dir_all(&data3)?;config(&data3)?;send(&data3,"Write a checked short report only if the title is safe.")?;let mut empty=ScriptedEndpoint{outputs:vec![report_record("---","Safe body that still lacks a slug.")],index:0};public_loop::run_once(&data3,&mut empty)?;let c=Connection::open(data3.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);assert!(!third.join("workspace").exists());Ok(())
}

struct Mutate {
    path: PathBuf,
    output: String,
}
impl Endpoint for Mutate {
    fn complete(&mut self, _: &Prompt, _: u32) -> Result<CompletionRecord, String> {
        fs::write(&self.path, "owner bytes win\n").map_err(|error| error.to_string())?;
        Ok(CompletionRecord::scripted(self.output.clone()))
    }
}

fn config(data: &Path) -> std::io::Result<()> {
    fs::write(
        data.join("lkjagent.json"),
        "{\"workspace_root\":\"../workspace\",\"workspace_timezone\":\"UTC\"}",
    )
}
fn send(data: &Path, text: &str) -> T<String> {
    let root = data.to_string_lossy();
    Ok(cli::run(["--data", root.as_ref(), "send", text])?)
}
fn memory_record(title: &str, body: &str) -> String {
    format!("<tool_call><tool>write_record</tool><input><family>memory</family><title>{title}</title><body>{body}</body></input></tool_call>")
}
fn report_record(title: &str, body: &str) -> String {
    format!("<tool_call><tool>write_record</tool><input><family>report</family><title>{title}</title><body>{body}</body></input></tool_call>")
}
fn final_message(body: &str) -> String {
    format!("<final><message>{body}</message></final>")
}
fn scalar(connection: &Connection, sql: &str) -> T<i64> {
    Ok(connection.query_row(sql, [], |row| row.get(0))?)
}
fn scalar1(connection: &Connection, sql: &str, path: [&str; 1]) -> T<i64> {
    Ok(connection.query_row(sql, path, |row| row.get(0))?)
}
fn assert_lineage(connection: &Connection, decision: &str, text: &str) -> T {
    let expected = query_lineage(connection, decision)?;
    let actual = text
        .lines()
        .skip_while(|line| *line != "source-lineage:")
        .skip(1)
        .take_while(|line| *line != "---")
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    Ok(())
}
fn query_lineage(connection: &Connection, decision: &str) -> T<Vec<String>> {
    let mut q=connection.prepare("SELECT source_kind,CAST(source_revision AS TEXT) FROM context_items WHERE decision_id=?1 ORDER BY rowid")?;
    let rows = q
        .query_map([decision], |r| {
            Ok(format!(
                "- {}:{}",
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}
#[rustfmt::skip]
fn inventory(root:&Path)->T<Vec<String>>{fn walk(root:&Path,at:&Path,out:&mut Vec<String>)->T{for row in fs::read_dir(at)?{let path=row?.path();if path.is_dir(){walk(root,&path,out)?}else{out.push(path.strip_prefix(root)?.to_string_lossy().into_owned())}}Ok(())}let mut out=vec![];walk(root,root,&mut out)?;out.sort();Ok(out)}
fn fixture(name: &str) -> T<PathBuf> {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("lkjagent-report-{name}-{n}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
