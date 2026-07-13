#![cfg(target_os = "linux")]
use lkjagent_app::public_loop;
use rusqlite::{Connection, OptionalExtension};
use std::{fs, os::unix::fs::PermissionsExt};

mod support;
use support::report_long::{
    child_record, config, endpoint, final_message, fixture, inventory, map_record, scalar, scalar1,
    send, T,
};

#[test]
#[rustfmt::skip]
fn long_report_map_children_restart_receipt_and_replacement_are_exact()->T{
 let root=fixture("flow")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;
 let readme="artifacts/documents/launch-plan/README.md";let summary="artifacts/documents/launch-plan/summary.md";let risks="artifacts/documents/launch-plan/risks.md";
 send(&data,"Write one checked long report with summary and risks sections.")?;
 public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","This map anchors the checked launch plan.","launch-plan","summary,risks",12)))?;
 let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-report-map'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-report-member'")?,2);assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-report-complete'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM matters WHERE lifecycle='open'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE namespace='check' AND cell_key='current-passed' AND status='active'")?,0);let pending:String=c.query_row("SELECT CAST(payload AS TEXT) FROM state_cells WHERE namespace='report' AND cell_key='pending' AND status='active'",[],|r|r.get(0))?;assert!(pending.contains("summary")&&pending.contains("risks"));drop(c);
 let text=fs::read_to_string(workspace.join(readme))?;assert!(text.contains("## Sections\n- [summary](summary.md)\n- [risks](risks.md)\n"));assert_eq!(fs::metadata(workspace.join(readme))?.permissions().mode()&0o777,0o600);assert_eq!(inventory(&workspace)?,vec![readme.to_string()]);
 public_loop::run_once(&data,&mut endpoint(child_record("Summary","The summary covers launch scope, owners, and sequence.","launch-plan","summary")))?;
 public_loop::run_once(&data,&mut endpoint(child_record("Summary","The summary now adds rollback and staffing detail before launch.","launch-plan","summary")))?;
 let c=Connection::open(data.join("lkjagent.sqlite3"))?;let pending:String=c.query_row("SELECT CAST(payload AS TEXT) FROM state_cells WHERE namespace='report' AND cell_key='pending' AND status='active'",[],|r|r.get(0))?;assert!(!pending.contains("summary")&&pending.contains("risks"));assert_eq!(scalar1(&c,"SELECT count(*) FROM workspace_revisions WHERE document_id=(SELECT id FROM workspace_documents WHERE CAST(current_path AS TEXT)=?1)",summary)?,2);assert!(scalar(&c,"SELECT count(*) FROM checks WHERE obligation_id LIKE '%report-member/launch-plan/summary' AND current=0")?>=1);assert_eq!(scalar(&c,"SELECT count(*) FROM matters WHERE lifecycle='open'")?,1);drop(c);
 public_loop::run_once(&data,&mut endpoint(child_record("Risks","The risks section names supply delay, budget pressure, and rollback triggers.","launch-plan","risks")))?;
 let c=Connection::open(data.join("lkjagent.sqlite3"))?;let still_pending:Option<String>=c.query_row("SELECT CAST(payload AS TEXT) FROM state_cells WHERE namespace='report' AND cell_key='pending' AND status='active'",[],|r|r.get(0)).optional()?;assert_eq!(still_pending,None);assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE namespace='check' AND cell_key='current-passed' AND status='active'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE predicate_kind='managed-report-complete' AND status='passed'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM workspace_documents")?,3);assert_eq!(scalar(&c,"SELECT count(*) FROM workspace_revisions")?,4);drop(c);
 public_loop::run_once(&data,&mut endpoint(final_message("Saved checked long report.")))?;
 let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%'")?,18);let (body,receipt):(Vec<u8>,Vec<u8>)=c.query_row("SELECT body,receipt FROM conversation_messages WHERE role='agent' ORDER BY sequence DESC LIMIT 1",[],|r|Ok((r.get(0)?,r.get(1)?)))?;let body=String::from_utf8(body)?;let receipt=String::from_utf8(receipt)?;for path in [readme,summary,risks]{assert!(body.contains(path));assert!(receipt.contains(path));}drop(c);
 assert_eq!(inventory(&workspace)?,vec![readme.to_string(),risks.to_string(),summary.to_string()]);Ok(())
}
