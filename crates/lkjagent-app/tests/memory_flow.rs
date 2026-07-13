#![cfg(target_os = "linux")]
use lkjagent_app::{
    cli,
    endpoint::{CompletionRecord, Endpoint, ScriptedEndpoint},
    memory_record::semantic_slug,
    public_loop,
};
use lkjagent_core::prompt::Prompt;
use lkjagent_store::transactions::NativeStore;
use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
type T<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
#[rustfmt::skip]
fn memory_create_replace_retrieve_and_correction_are_revision_exact()->T{
 let root=fixture("flow")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Remember that the project color is amber.")?;
 let mut first=ScriptedEndpoint{outputs:vec![record("Project Color","The owner says the project color is amber."),final_message("Saved checked memory.")],index:0};public_loop::run_once(&data,&mut first)?;public_loop::run_once(&data,&mut first)?;assert!(public_loop::run_once(&data,&mut first)?.starts_with("idle:"));
 let path="knowledge/notes/project-color.md";let file=workspace.join(path);let text=fs::read_to_string(&file)?;assert!(text.contains("kind: memory")&&text.contains("semantic-key: project-color")&&text.contains("# Project Color"));let db=data.join("lkjagent.sqlite3");let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-memory' AND status='passed'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal WHERE status='settled'")?,1);drop(c);
 send(&data,"Replace the project color memory with the current owner correction.")?;let mut replace=ScriptedEndpoint{outputs:vec![record("Project Color","The owner now says the project color is cobalt."),final_message("Updated checked memory.")],index:0};public_loop::run_once(&data,&mut replace)?;public_loop::run_once(&data,&mut replace)?;let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM workspace_documents")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM workspace_revisions")?,2);let current:String=c.query_row("SELECT current_revision_id FROM workspace_documents WHERE CAST(current_path AS TEXT)=?1",[path],|r|r.get(0))?;let stale:String=c.query_row("SELECT id FROM workspace_revisions WHERE id<>?1",[&current],|r|r.get(0))?;drop(c);
 fs::create_dir_all(workspace.join("knowledge/notes"))?;fs::write(workspace.join("knowledge/notes/rogue.md"),"ROGUE-FILESYSTEM-CONTENT")?;send(&data,"Use the saved color fact in one concise answer.")?;let mut capture=Capture::new(list_complete());public_loop::run_once(&data,&mut capture)?;let prompt=capture.prompt.ok_or("prompt")?;assert_eq!(prompt.matches("project color is cobalt").count(),1);assert!(!prompt.contains("project color is amber")&&!prompt.contains("ROGUE-FILESYSTEM-CONTENT"));let c=Connection::open(&db)?;let revision:String=c.query_row("SELECT CAST(source_revision AS TEXT) FROM context_items WHERE source_kind='memory' ORDER BY rowid DESC LIMIT 1",[],|r|r.get(0))?;assert_eq!(revision,current);assert_ne!(revision,stale);drop(c);let mut finish=ScriptedEndpoint{outputs:vec![final_message("The checked answer is ready.")],index:0};public_loop::run_once(&data,&mut finish)?;
 send(&data,"Prepare a matter that will be durably reopened.")?;let c=Connection::open(&db)?;let reopened:String=c.query_row("SELECT id FROM matters WHERE lifecycle='open' ORDER BY created_sequence DESC LIMIT 1",[],|r|r.get(0))?;drop(c);let mut store=NativeStore::open(&db)?;let sequence=store.next_event_sequence(&reopened)?;store.block_budget(&reopened,None,"memory-reopen-block",sequence,1,"test-wall",b"test block",b"test-block-fingerprint")?;send(&data,"Use the saved color fact after reopen.")?;let mut reopened_capture=Capture::new(list_complete());public_loop::run_once(&data,&mut reopened_capture)?;assert_eq!(reopened_capture.prompt.ok_or("reopen prompt")?.matches("project color is cobalt").count(),1);let mut reopened_finish=ScriptedEndpoint{outputs:vec![final_message("The reopened checked answer is ready.")],index:0};public_loop::run_once(&data,&mut reopened_finish)?;
 send(&data,"forget project-color: answer without the saved memory.")?;let mut corrected=Capture::new(final_message("Current owner correction wins."));public_loop::run_once(&data,&mut corrected)?;assert!(!corrected.prompt.ok_or("prompt")?.contains("project color is cobalt"));let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM context_items i JOIN runtime_decisions d ON d.id=i.decision_id JOIN matters m ON m.id=d.matter_id WHERE i.source_kind='memory' AND CAST(m.objective AS TEXT) LIKE 'forget project-color:%'")?,0);Ok(())
}

#[test]
#[rustfmt::skip]
fn collision_stale_bytes_oversize_and_empty_slug_never_mutate_as_memory()->T{
 assert_eq!(semantic_slug("  Project COLOR / 2026  ").as_deref(),Some("project-color-2026"));assert_eq!(semantic_slug("---"),None);
 let stale=fixture("stale")?;let stale_data=stale.join("data");fs::create_dir_all(&stale_data)?;config(&stale_data)?;send(&stale_data,"Remember an owner fact safely.")?;let mut create=ScriptedEndpoint{outputs:vec![record("Stable Fact","The owner supplied stable source content."),final_message("Saved checked memory.")],index:0};public_loop::run_once(&stale_data,&mut create)?;public_loop::run_once(&stale_data,&mut create)?;send(&stale_data,"Update the stable fact without losing owner bytes.")?;let stale_file=stale.join("workspace/knowledge/notes/stable-fact.md");let mut mutate=Mutate{path:stale_file.clone(),output:record("Stable Fact","Model bytes must not replace a concurrent owner edit.")};public_loop::run_once(&stale_data,&mut mutate)?;assert_eq!(fs::read_to_string(stale_file)?,"owner bytes win\n");let c=Connection::open(stale_data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,1);drop(c);
 let root=fixture("collision")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;fs::create_dir_all(workspace.join("knowledge/notes"))?;let file=workspace.join("knowledge/notes/collision.md");fs::write(&file,"owner bytes\n")?;send(&data,"Remember this collision safely.")?;let mut collision=ScriptedEndpoint{outputs:vec![record("Collision","Must not replace owner bytes.")],index:0};public_loop::run_once(&data,&mut collision)?;assert_eq!(fs::read_to_string(&file)?,"owner bytes\n");let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);drop(c);
 let second=fixture("oversize")?;let data2=second.join("data");fs::create_dir_all(&data2)?;config(&data2)?;send(&data2,"Remember bounded content.")?;let mut huge=ScriptedEndpoint{outputs:vec![record("Bounded Memory",&"grounded ".repeat(170))],index:0};public_loop::run_once(&data2,&mut huge)?;let c=Connection::open(data2.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);assert!(!second.join("workspace").exists());Ok(())
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

struct Capture {
    output: String,
    prompt: Option<String>,
}
impl Capture {
    fn new(output: String) -> Self {
        Self {
            output,
            prompt: None,
        }
    }
}
impl Endpoint for Capture {
    fn complete(&mut self, prompt: &Prompt, _: u32) -> Result<CompletionRecord, String> {
        self.prompt = Some(format!("{}\n{}", prompt.system, prompt.user));
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
fn record(title: &str, body: &str) -> String {
    format!("<tool_call><tool>write_record</tool><input><family>memory</family><title>{title}</title><body>{body}</body></input></tool_call>")
}
fn final_message(body: &str) -> String {
    format!("<final><message>{body}</message></final>")
}
fn list_complete() -> String {
    "<tool_call><tool>list_directory</tool><input><path>.</path><complete>true</complete></input></tool_call>".into()
}
fn scalar(connection: &Connection, sql: &str) -> T<i64> {
    Ok(connection.query_row(sql, [], |row| row.get(0))?)
}
fn fixture(name: &str) -> T<PathBuf> {
    let n = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("lkjagent-memory-{name}-{n}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
