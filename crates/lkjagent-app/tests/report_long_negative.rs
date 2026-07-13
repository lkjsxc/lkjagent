#![cfg(target_os = "linux")]
use lkjagent_app::public_loop;
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
 let root=fixture("stale")?;let data=root.join("data");let workspace=root.join("workspace");fs::create_dir_all(&data)?;config(&data)?;send(&data,"Write a checked long report.")?;public_loop::run_once(&data,&mut endpoint(map_record("Launch Plan","Map body","launch-plan","summary,risks",12)))?;public_loop::run_once(&data,&mut endpoint(child_record("Summary","Current summary body.","launch-plan","summary")))?;let file=workspace.join("artifacts/documents/launch-plan/summary.md");public_loop::run_once(&data,&mut Mutate{path:file.clone(),output:child_record("Summary","Model bytes lose to owner bytes.","launch-plan","summary")})?;let c=Connection::open(data.join("lkjagent.sqlite3"))?;assert_eq!(scalar(&c,"SELECT count(*) FROM effect_journal")?,2);assert_eq!(fs::read_to_string(file)?,"owner bytes win\n");Ok(())
}
