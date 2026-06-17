use std::time::Duration;

pub const BACKOFF_CAP: Duration = Duration::from_secs(900);

pub fn delay_for_attempt(attempt: u32) -> Duration {
    let seconds = match 1_u64.checked_shl(attempt) {
        Some(value) => value,
        None => BACKOFF_CAP.as_secs(),
    };
    Duration::from_secs(seconds.min(BACKOFF_CAP.as_secs()))
}

pub fn delays(count: usize) -> Vec<Duration> {
    let mut schedule = Vec::new();
    for attempt in 0..count {
        schedule.push(delay_for_attempt(attempt as u32));
    }
    schedule
}
