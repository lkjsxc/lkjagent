pub(super) fn retry_deadline(now: &str, retry_after_secs: Option<u64>) -> Option<String> {
    let delay = retry_after_secs?;
    now.parse::<u64>()
        .ok()
        .map(|stamp| stamp.saturating_add(delay).to_string())
}

pub(super) fn seconds_before(now: &str, deadline: &str) -> bool {
    match (now.parse::<u64>(), deadline.parse::<u64>()) {
        (Ok(now), Ok(deadline)) => now < deadline,
        _ => false,
    }
}
