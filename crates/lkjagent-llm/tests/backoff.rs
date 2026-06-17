use std::time::Duration;

use lkjagent_llm::backoff::{delay_for_attempt, delays, BACKOFF_CAP};

#[test]
fn exponential_backoff_caps_at_fifteen_minutes() {
    let expected = vec![
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::from_secs(4),
        Duration::from_secs(8),
        Duration::from_secs(16),
    ];
    assert_eq!(delays(5), expected);
    assert_eq!(delay_for_attempt(20), BACKOFF_CAP);
}
