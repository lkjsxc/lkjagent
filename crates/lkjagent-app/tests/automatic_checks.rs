use std::fs;

use lkjagent_app::automatic_checks::reduce_committed_edit;
use lkjagent_store::native_schema;
use rusqlite::Connection;

mod support;
use support::automatic_checks_fixture::{close, scalar, settled_create, settled_edit, text};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn committed_edit_schedules_checks_once_and_closes() -> TestResult<()> {
    let (root, workspace) = settled_edit("close")?;
    let db = root.join("native.sqlite3");
    let mut connection = native_schema::open(&db)?;
    assert!(close(&db).is_err(), "premature final must remain blocked");
    let reduced = reduce_committed_edit(&mut connection, &workspace, "j", 4, "now")?;
    assert_eq!((reduced.scheduled, reduced.passed), (3, 3));
    assert_eq!(
        scalar(&connection, "SELECT count(*) FROM state_cells WHERE namespace='check' AND cell_key='current-passed' AND status='active'")?,
        1
    );
    let repeated = reduce_committed_edit(&mut connection, &workspace, "j", 5, "later")?;
    assert_eq!((repeated.scheduled, repeated.passed), (0, 0));
    drop(connection);
    close(&db)?;
    let connection = Connection::open(db)?;
    assert_eq!(scalar(&connection, "SELECT count(*) FROM checks")?, 3);
    assert_eq!(
        text(&connection, "SELECT lifecycle FROM matters WHERE id='m'")?,
        "closed"
    );
    Ok(())
}

#[test]
fn committed_create_gets_native_checks() -> TestResult<()> {
    let (root, workspace) = settled_create("create")?;
    let db = root.join("native.sqlite3");
    let mut connection = native_schema::open(&db)?;
    let reduced = reduce_committed_edit(&mut connection, &workspace, "j", 4, "now")?;
    assert_eq!((reduced.scheduled, reduced.passed), (3, 3));
    Ok(())
}

#[test]
fn later_bytes_invalidate_checks_and_block_final() -> TestResult<()> {
    let (root, workspace) = settled_edit("stale")?;
    let db = root.join("native.sqlite3");
    let mut connection = native_schema::open(&db)?;
    let first = reduce_committed_edit(&mut connection, &workspace, "j", 4, "now")?;
    assert_eq!((first.scheduled, first.passed), (3, 3));
    fs::write(root.join("workspace/notes/a.md"), "gamma is current\n")?;
    let stale = reduce_committed_edit(&mut connection, &workspace, "j", 5, "later")?;
    assert_eq!((stale.scheduled, stale.passed), (3, 1));
    assert_eq!(
        scalar(&connection, "SELECT count(*) FROM state_cells WHERE namespace='check' AND cell_key='failed' AND status='active'")?,
        1
    );
    assert_eq!(
        scalar(&connection, "SELECT count(*) FROM checks WHERE current=1")?,
        3
    );
    assert_eq!(
        scalar(&connection, "SELECT count(*) FROM checks WHERE current=0")?,
        3
    );
    drop(connection);
    assert!(close(&db).is_err());
    Ok(())
}
