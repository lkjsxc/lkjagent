use std::time::{SystemTime, UNIX_EPOCH};

pub fn utc_now() -> String {
    match SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
    {
        Some(milliseconds) => iso_from_unix_millis(milliseconds),
        None => "1970-01-01T00:00:00Z".to_string(),
    }
}

fn iso_from_unix(seconds: u64) -> String {
    let days = seconds / 86_400;
    let rem = seconds % 86_400;
    let (year, month, day) = civil_from_days(days as i64);
    let hour = rem / 3_600;
    let minute = (rem % 3_600) / 60;
    let second = rem % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn iso_from_unix_millis(milliseconds: u64) -> String {
    let mut value = iso_from_unix(milliseconds / 1_000);
    let fraction = milliseconds % 1_000;
    if fraction != 0 {
        let _ = value.pop();
        value.push_str(&format!(".{fraction:03}Z"));
    }
    value
}

fn civil_from_days(days: i64) -> (i64, u64, u64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_param = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_param + 2) / 5 + 1;
    let month = month_param + if month_param < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    (year, month as u64, day as u64)
}

#[cfg(test)]
mod tests {
    use super::iso_from_unix;

    #[test]
    fn unix_seconds_render_human_utc() {
        assert_eq!(iso_from_unix(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_from_unix(86_400), "1970-01-02T00:00:00Z");
        assert_eq!(super::iso_from_unix_millis(50), "1970-01-01T00:00:00.050Z");
        assert_eq!(iso_from_unix(1_788_739_200), "2026-09-07T00:00:00Z");
    }
}
