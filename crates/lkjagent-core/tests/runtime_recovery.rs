use lkjagent_core::runtime_recovery::{
    bounded_diagnostic, normalized_signature, plan, tuple_fingerprint, FailureClass,
    RecoveryStrategy,
};

type TestResult<T> = Result<T, String>;

#[test]
fn typed_ladders_advance_and_exhaust() {
    let parse = plan(FailureClass::Parse, 0);
    assert_eq!(parse.next_strategy, Some(RecoveryStrategy::GrammarRepair));
    assert_eq!(parse.remaining_budget, 3);
    assert!(!parse.exhausted);

    let endpoint = plan(FailureClass::Endpoint, 4);
    assert_eq!(endpoint.next_strategy, Some(RecoveryStrategy::WaitExternal));
    assert!(endpoint.wait_external);
    assert!(plan(FailureClass::Endpoint, 5).exhausted);
}

#[test]
fn signatures_are_normalized_and_diagnostics_are_bounded() -> TestResult<()> {
    let left = normalized_signature("  BAD   Envelope ").map_err(|error| error.message)?;
    let right = normalized_signature("bad envelope").map_err(|error| error.message)?;
    assert_eq!(left, right);
    let first = normalized_signature("HTTP 503 request_id=req-ab12 at 2026-07-11T10:20:30 retry 1")
        .map_err(|error| error.message)?;
    let second =
        normalized_signature("http 503 request_id=req-ff99 at 2027-08-12T11:21:31 retry 9")
            .map_err(|error| error.message)?;
    assert_eq!(first, second);
    let colon_a = normalized_signature("unavailable request-id: abcdefghijklmnop")
        .map_err(|error| error.message)?;
    let colon_b = normalized_signature("unavailable request-id: zyxwvutsrqponmlk")
        .map_err(|error| error.message)?;
    assert_eq!(colon_a, colon_b);
    assert_eq!(bounded_diagnostic(&"x".repeat(600)).chars().count(), 512);
    Ok(())
}

#[test]
fn failure_tuple_binds_every_no_repeat_dimension() -> TestResult<()> {
    let fp = |op, prompt, tools, budget, fault| {
        tuple_fingerprint(op, prompt, tools, budget, fault).map_err(|error| error.message)
    };
    let base = fp("op", "prompt", "tools", "budget", "fault")?;
    for changed in [
        fp("other", "prompt", "tools", "budget", "fault")?,
        fp("op", "other", "tools", "budget", "fault")?,
        fp("op", "prompt", "other", "budget", "fault")?,
        fp("op", "prompt", "tools", "other", "fault")?,
        fp("op", "prompt", "tools", "budget", "other")?,
    ] {
        assert_ne!(base, changed);
    }
    assert_eq!(
        FailureClass::from_fault("endpoint", "maximum tokens reached"),
        FailureClass::OutputLimit
    );
    Ok(())
}
