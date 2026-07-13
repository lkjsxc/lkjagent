use std::collections::BTreeSet;
use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

use super::*;
use crate::native_schema;

fn path(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("tui-frame-{label}-{nonce}.db")))
}

#[rustfmt::skip]
fn page(conversation_limit: usize, activity_limit: usize) -> SnapshotPage {
    SnapshotPage { conversation_before: None, conversation_limit, activity_before: None, activity_limit }
}

fn fixture(label: &str) -> Result<(PathBuf, Connection), Box<dyn Error>> {
    let path = path(label)?;
    let connection = native_schema::open(&path)?;
    connection.execute_batch(
        "BEGIN;
         INSERT INTO matters VALUES('m',x'6f626a','open',0,1,4,NULL,0,0);
         INSERT INTO owner_turns VALUES('t1',1,x'6f6e65','delivered','m','now');
         INSERT INTO owner_turns VALUES('t3',2,x'7468726565','delivered','m','now');
         INSERT INTO runtime_events VALUES('e1','m',1,'owner-intake',10,'now',x'7261772d6572726f72','owner-turn','t1');
         INSERT INTO runtime_events VALUES('e2','m',2,'reply',20,'now',x'7061796c6f6164','harness','reply');
         INSERT INTO runtime_events VALUES('e3','m',3,'decision-selected',30,'now',x'7061796c6f6164','harness','d');
         INSERT INTO runtime_events VALUES('e4','m',4,'observed',40,'now',x'7061796c6f6164','tool','o');
         INSERT INTO conversation_messages VALUES('msg-1',1,'owner',x'6f6e65',x'6631',NULL,NULL,'active','m','t1','e1',NULL);
         INSERT INTO conversation_messages VALUES('msg-2',2,'agent',x'74776f',x'6632',x'72656365697074',x'726670','active','m',NULL,'e2',NULL);
         INSERT INTO conversation_messages VALUES('msg-3',3,'owner',x'7468726565',x'6633',NULL,NULL,'active','m','t3','e2',NULL);
         INSERT INTO obligations VALUES('ob','m','exact',x'70726976617465',1,'open',NULL,NULL);
         INSERT INTO runtime_decisions VALUES('d','m','e3',x'6f70',x'6964656d',30,x'7365637265742d70726f6d7074',x'63',x'74',x'67',x'62',x'72',x'63',x'65','compiling',NULL,NULL,NULL,NULL,'selected',NULL);
         INSERT INTO provider_exchanges VALUES('p','d',x'7365637265742d72657175657374',x'7365637265742d726573706f6e7365',1,1,31,32,x'7261772d6572726f72',x'7261772d6572726f72',x'706172736564','succeeded');
         INSERT INTO tool_admissions VALUES('a-reject','d',0,x'6131','model',0,'rejected',x'7261772d6572726f72',x'7365637265742d63616c6c',x'73706563',NULL);
         INSERT INTO tool_admissions VALUES('a-effect','d',1,x'6132','model',1,'accepted',x'6f6b',x'7365637265742d63616c6c',x'73706563','j');
         INSERT INTO effect_journal VALUES('j','a-effect','d',1,x'6a69','prepared',x'696e74656e646564',NULL,NULL,NULL);
         INSERT INTO observations VALUES('o',NULL,'d','failed',x'7261772d6572726f72',x'7365637265742d726566',x'666f','clean','e4');
         INSERT INTO checks VALUES('ck','m','ob','d','exact',x'7365637265742d706172616d73',1,0,x'7261772d6572726f72',x'6566',x'726576','e4');
         INSERT INTO state_cells VALUES('m',x'7365637265742d6e616d657370616365',x'7365637265742d6b6579',x'7365637265742d7061796c6f6164','active','e1',x'66696e6765727072696e74');
         COMMIT;",
    )?;
    Ok((path, connection))
}

#[test]
#[rustfmt::skip]
fn canonical_order_and_conversation_activity_separation() -> Result<(), Box<dyn Error>> {
    let (_path, mut connection) = fixture("order")?;
    let frame = snapshot(&mut connection, &page(20, 20))?;
    assert_eq!(frame.conversation.iter().map(|row| row.sequence).collect::<Vec<_>>(), [1, 2, 3]);
    assert_eq!(frame.activity.len(), 8);
    assert!(frame.activity.windows(2).all(|rows| {
        (rows[0].monotonic_ms, rows[0].cursor.kind_rank, rows[0].cursor.raw_id.as_str())
            <= (rows[1].monotonic_ms, rows[1].cursor.kind_rank, rows[1].cursor.raw_id.as_str())
    }));
    assert!(frame.activity.iter().all(|row| !row.kind.contains("conversation")));
    let visible = frame.activity.iter().map(|row| {
        format!("{} {} {} {}", row.id, row.kind, row.matter_id, row.status)
    }).collect::<String>();
    for forbidden in ["secret-prompt", "secret-request", "secret-response", "secret-call", "raw-error", "secret-payload"] {
        assert!(!visible.contains(forbidden), "exposed {forbidden}");
    }
    assert_eq!((frame.status.open_matters, frame.status.rejected_admissions,
        frame.status.failed_observations, frame.status.current_checks), (1, 1, 1, 1));
    Ok(())
}

#[test]
fn activity_ids_are_unique_and_stable_across_polls() -> Result<(), Box<dyn Error>> {
    let (_path, mut connection) = fixture("identity")?;
    let first = snapshot(&mut connection, &page(20, 20))?;
    connection.execute(
        "UPDATE provider_exchanges SET status='failed' WHERE id='p'",
        [],
    )?;
    let second = snapshot(&mut connection, &page(20, 20))?;
    let first_ids = first
        .activity
        .iter()
        .map(|row| row.id.clone())
        .collect::<Vec<_>>();
    let second_ids = second
        .activity
        .iter()
        .map(|row| row.id.clone())
        .collect::<Vec<_>>();
    assert_eq!(first_ids, second_ids);
    assert_eq!(
        first_ids.iter().collect::<BTreeSet<_>>().len(),
        first_ids.len()
    );
    assert!(first_ids
        .iter()
        .any(|id| id.starts_with("state-cell/fnv1a64:")));
    Ok(())
}

#[test]
fn pages_and_rows_obey_canonical_bounds() -> Result<(), Box<dyn Error>> {
    let (_path, mut connection) = fixture("bounds")?;
    let newest = snapshot(&mut connection, &page(2, 3))?;
    assert_eq!(
        newest
            .conversation
            .iter()
            .map(|row| row.sequence)
            .collect::<Vec<_>>(),
        [2, 3]
    );
    let older_page = SnapshotPage {
        conversation_before: Some(2),
        conversation_limit: 2,
        activity_before: newest.activity.first().map(ActivityRow::cursor),
        activity_limit: 3,
    };
    let older = snapshot(&mut connection, &older_page)?;
    assert_eq!(
        older
            .conversation
            .iter()
            .map(|row| row.sequence)
            .collect::<Vec<_>>(),
        [1]
    );
    assert!(newest
        .activity
        .iter()
        .all(|row| !older.activity.iter().any(|old| old.id == row.id)));
    connection.execute_batch(
        "UPDATE conversation_messages SET body=zeroblob(20000) WHERE id='msg-1';
         WITH RECURSIVE n(x) AS (VALUES(4) UNION ALL SELECT x+1 FROM n WHERE x<110)
         INSERT INTO conversation_messages(id,sequence,role,body,body_fingerprint,receipt,receipt_fingerprint,lifecycle,matter_id)
         SELECT 'bulk-'||x,x,'agent',x'62',x'66',x'72',x'66','active','m' FROM n;
         WITH RECURSIVE n(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM n WHERE x<205)
         INSERT INTO state_cells(matter_id,namespace,cell_key,payload,status,source_event_id,fingerprint)
         SELECT 'm',x'62',CAST(printf('bulk-%03d',x) AS BLOB),x'70','active','e1',x'66' FROM n;",
    )?;
    let bounded = snapshot(&mut connection, &page(usize::MAX, usize::MAX))?;
    assert_eq!(bounded.conversation.len(), MAX_CONVERSATION_ROWS);
    assert_eq!(bounded.activity.len(), MAX_ACTIVITY_ROWS);
    let first = SnapshotPage {
        conversation_before: Some(2),
        ..page(10, 10)
    };
    let message = snapshot(&mut connection, &first)?.conversation.remove(0);
    assert_eq!(message.body.len(), MAX_CONVERSATION_BODY_BYTES);
    assert!(message.body_truncated);
    assert!(snapshot(&mut connection, &page(0, 1)).is_err());
    Ok(())
}

#[test]
fn wal_writer_commit_between_queries_does_not_split_frame() -> Result<(), Box<dyn Error>> {
    let (path, mut reader) = fixture("wal")?;
    let writer = native_schema::open(&path)?;
    let first = snapshot_with(&mut reader, &page(20, 20), || {
        writer.execute_batch(
            "BEGIN; INSERT INTO matters VALUES('new',x'6e','open',0,1,2,NULL,0,0);
             INSERT INTO runtime_events VALUES('new-e','new',1,'decision-selected',50,'now',x'70','harness','new-d');
             INSERT INTO runtime_decisions VALUES('new-d','new','new-e',x'6f70',x'6e6577',50,x'73',x'63',x'74',x'67',x'62',x'72',x'63',x'65','compiling',NULL,NULL,NULL,NULL,'selected',NULL); COMMIT;",
        )?;
        Ok(())
    })?;
    assert_eq!(first.status.open_matters, 1);
    assert!(first.activity.iter().all(|row| row.id != "decision/new-d"));
    let second = snapshot(&mut reader, &page(20, 20))?;
    assert_eq!(second.status.open_matters, 2);
    assert!(second.activity.iter().any(|row| row.id == "decision/new-d"));
    Ok(())
}
