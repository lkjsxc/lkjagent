mod support;

use lkjagent_store::state::{delete, get, take_lock, LockDecision};
use support::{memory_store, TestResult};

#[test]
fn lock_take_refuse_and_reclaim_behaviors_work() -> TestResult<()> {
    let conn = memory_store()?;
    assert_eq!(
        take_lock(
            &conn,
            "pid1",
            "2026-01-01T00:00:00Z",
            "2025-01-01T00:00:00Z"
        )?,
        LockDecision::Taken
    );
    assert!(matches!(
        take_lock(
            &conn,
            "pid2",
            "2026-01-01T00:00:01Z",
            "2025-01-01T00:00:00Z"
        )?,
        LockDecision::Refused { .. }
    ));
    assert!(matches!(
        take_lock(
            &conn,
            "pid2",
            "2026-01-01T00:00:01Z",
            "2027-01-01T00:00:00Z"
        )?,
        LockDecision::Reclaimed { .. }
    ));
    delete(&conn, "daemon lock")?;
    assert_eq!(get(&conn, "daemon lock")?, None);
    Ok(())
}
