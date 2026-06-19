mod support;

use lkjagent_store::state::{delete, get, heartbeat_lock, take_lock, LockDecision};
use support::{memory_store, TestResult};

#[test]
fn lock_take_heartbeat_refuse_and_reclaim_behaviors_work() -> TestResult<()> {
    let conn = memory_store()?;
    assert_eq!(take_lock(&conn, "pid1", "100", "0")?, LockDecision::Taken);
    assert_eq!(get(&conn, "daemon lock")?, Some("pid1|100|100".to_string()));
    assert!(heartbeat_lock(&conn, "pid1", "250")?);
    assert_eq!(get(&conn, "daemon lock")?, Some("pid1|100|250".to_string()));
    assert_eq!(take_lock(&conn, "pid1", "260", "0")?, LockDecision::Taken);
    assert_eq!(get(&conn, "daemon lock")?, Some("pid1|260|260".to_string()));

    assert!(matches!(
        take_lock(&conn, "pid2", "270", "200")?,
        LockDecision::Refused { .. }
    ));
    assert!(matches!(
        take_lock(&conn, "pid2", "600", "300")?,
        LockDecision::Reclaimed { .. }
    ));
    delete(&conn, "daemon lock")?;
    assert_eq!(get(&conn, "daemon lock")?, None);
    Ok(())
}
