use std::time::{SystemTime, UNIX_EPOCH};

pub trait Clock {
    fn now(&mut self) -> String;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&mut self) -> String {
        utc_now()
    }
}

#[derive(Debug, Clone)]
pub struct FixedClock {
    value: String,
}

impl FixedClock {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Clock for FixedClock {
    fn now(&mut self) -> String {
        self.value.clone()
    }
}

pub fn utc_now() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!(
            "unix:{}.{:09}Z",
            duration.as_secs(),
            duration.subsec_nanos()
        ),
        Err(_) => "unix:0.000000000Z".to_string(),
    }
}
