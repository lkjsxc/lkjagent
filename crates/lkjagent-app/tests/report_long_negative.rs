#![cfg(target_os = "linux")]
use lkjagent_app::{
    endpoint::{CompletionRecord, Endpoint},
    public_loop,
};
use lkjagent_core::prompt::Prompt;
use rusqlite::Connection;
use std::fs;

mod support;
use support::report_long::{
    child_record, config, endpoint, fixture, map_record, scalar, send, Mutate, T,
};

#[test]
#[rustfmt::skip]
fn long_report_rejects_bad_shapes_collisions_oversize_placeholder_and_stale_bytes()->T{
 let root=fixture("dupe")?;let data=root.join("data");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a long report only if the topology is safe.")?;public_loop::run_once(&data,&mut endpoint(map_record("Bad","Safe body","launch-plan","summary,summary",12)))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);drop(c);
 let root=fixture("anon")?;let data=root.join("data");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a long report only if the slug is safe.")?;public_loop::run_once(&data,&mut endpoint(map_record("Bad","Safe body","part-1","summary,risks",12)))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);drop(c);
 let root=fixture("collision")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(workspace.join("artifacts/documents/launch-plan"))?;fs::write(workspace.join("artifacts/documents/launch-plan/README.md"),"owner bytes\n")?;fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a long report safely.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Safe body","launch-plan","summary,risks",12)))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);assert_eq!(fs::read_to_string(workspace.join("artifacts/documents/launch-plan/README.md"))?,"owner bytes\n");drop(c);
 let root=fixture("oversize")?;let data=root.join("data");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a bounded long report.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan",&"grounded ".repeat(260),"launch-plan","summary,risks",12)))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,0);drop(c);
 let root=fixture("topology")?;let data=root.join("data");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a checked long report.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Map body","launch-plan","summary,risks",12)))?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Changed body","launch-plan","summary,owners",12)))?;public_loop::run_once(&data,&mut endpoint(child_record("Owners","Unknown member.","launch-plan","owners")))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,1);drop(c);
 let root=fixture("placeholder")?;let data=root.join("data");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a checked long report.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Map body","launch-plan","summary,risks",12)))?;public_loop::run_once(&data,&mut endpoint(child_record("Summary","[placeholder]","launch-plan","summary")))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,1);drop(c);
 let root=fixture("stale")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a checked long report.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Map body","launch-plan","summary,risks",12)))?;public_loop::run_once(&data,&mut endpoint(child_record("Summary","Current summary body.","launch-plan","summary")))?;let file=workspace.join("artifacts/documents/launch-plan/summary.md");public_loop::run_once(&data,&mut Mutate{path:file.clone(),output:child_record("Summary","Model bytes lose to owner bytes.","launch-plan","summary")})?;public_loop::run_once(&data,&mut endpoint(child_record("Risks","Current risks remain bounded and grounded.","launch-plan","risks")))?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,3);assert_eq!(scalar(&c,"SELECT count(*) FROM obligations WHERE id LIKE '%report-member/launch-plan/summary' AND status='open'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM checks WHERE obligation_id LIKE '%report-member/launch-plan/summary' AND current=1")?,0);assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE namespace='check' AND cell_key='current-passed' AND status='active'")?,0);assert_eq!(fs::read_to_string(file)?,"owner bytes win\n");Ok(())
}

#[test]
#[rustfmt::skip]
fn output_limit_retries_one_pending_unit_without_persisting_truncated_bytes()->T{
 let root=fixture("limit")?;let data=root.join("data");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a checked long report and recover by reducing one unit if output is limited.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Map body","launch-plan","summary,risks",12)))?;public_loop::run_once(&data,&mut Limited)?;
 let db=data.join("lkjagent.sqlite3");let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE namespace='recovery' AND CAST(cell_key AS TEXT)='output-limit' AND status='active'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM provider_exchanges WHERE CAST(response_ref AS TEXT)='output-limit' AND CAST(parse_result AS TEXT)='output-limit'")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM runtime_events WHERE instr(CAST(payload AS TEXT),'TRUNCATED-BODY-MUST-NOT-PERSIST')>0")?,0);drop(c);
 public_loop::run_once(&data,&mut endpoint(child_record("Summary","The bounded summary names scope, owners, and sequence.","launch-plan","summary")))?;let c=Connection::open(&db)?;assert_eq!(scalar(&c,"SELECT count(*) FROM runtime_decisions WHERE CAST(operation_key AS TEXT)='modify.report.reduce-unit' AND instr(CAST(recovery_spec AS TEXT),'reduce-unit')>0")?,1);assert_eq!(scalar(&c,"SELECT count(*) FROM state_cells WHERE namespace='recovery' AND CAST(cell_key AS TEXT)='output-limit' AND status='active'")?,0);assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,2);Ok(())
}

struct Limited;
impl Endpoint for Limited {
    fn complete(&mut self, _: &Prompt, _: u32) -> Result<CompletionRecord, String> {
        let mut record =
            CompletionRecord::scripted("<tool_call>TRUNCATED-BODY-MUST-NOT-PERSIST".into());
        record.finish_reason = "Length".into();
        Ok(record)
    }
}
