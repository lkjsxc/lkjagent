use crate::runtime_state::{RuntimeSnapshot, StateCell};

pub fn cell_is_due(cell: &StateCell, now: Option<&str>) -> bool {
    match (cell.cooldown_until.as_deref(), now) {
        (Some(due), Some(now)) => match (utc_millis(due), utc_millis(now)) {
            (Some(due), Some(now)) => due <= now,
            _ => false,
        },
        _ => true,
    }
}

pub fn next_wake<'a>(snapshot: &'a RuntimeSnapshot, now: &str) -> Option<&'a str> {
    let now = utc_millis(now)?;
    snapshot
        .active_cells()
        .into_iter()
        .filter_map(|cell| cell.cooldown_until.as_deref())
        .filter_map(|due| {
            utc_millis(due)
                .filter(|value| *value > now)
                .map(|value| (value, due))
        })
        .min_by_key(|(value, _)| *value)
        .map(|(_, due)| due)
}

pub fn has_invalid_cooldown(snapshot: &RuntimeSnapshot) -> bool {
    snapshot
        .active_cells()
        .into_iter()
        .filter_map(|cell| cell.cooldown_until.as_deref())
        .any(|due| utc_millis(due).is_none())
}

pub fn utc_millis(value: &str) -> Option<u64> {
    let bytes = value.as_bytes();
    let suffix = match bytes.len() {
        20 if bytes[19] == b'Z' => 0,
        24 if bytes[19] == b'.'
            && bytes[23] == b'Z'
            && bytes[20..23].iter().all(u8::is_ascii_digit) =>
        {
            u64::from(bytes[20] - b'0') * 100
                + u64::from(bytes[21] - b'0') * 10
                + u64::from(bytes[22] - b'0')
        }
        _ => return None,
    };
    if !matches!(
        (bytes[4], bytes[7], bytes[10], bytes[13], bytes[16]),
        (b'-', b'-', b'T', b':', b':')
    ) {
        return None;
    }
    if !bytes[..19]
        .iter()
        .enumerate()
        .all(|(index, byte)| matches!(index, 4 | 7 | 10 | 13 | 16) || byte.is_ascii_digit())
    {
        return None;
    }
    let number = |range: std::ops::Range<usize>| value.get(range)?.parse::<i64>().ok();
    let (year, month, day) = (number(0..4)?, number(5..7)?, number(8..10)?);
    let (hour, minute, second) = (number(11..13)?, number(14..16)?, number(17..19)?);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    if !(1..=max_day).contains(&day) || hour >= 24 || minute >= 60 || second >= 60 {
        return None;
    }
    let adjusted = year - i64::from(month <= 2);
    let era = adjusted.div_euclid(400);
    let year_of_era = adjusted - era * 400;
    let shifted = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * shifted + 2) / 5 + day - 1;
    let days = era * 146_097 + year_of_era * 365 + year_of_era / 4 - year_of_era / 100
        + day_of_year
        - 719_468;
    let seconds = days
        .checked_mul(86_400)?
        .checked_add(hour * 3_600 + minute * 60 + second)?;
    u64::try_from(seconds)
        .ok()?
        .checked_mul(1_000)?
        .checked_add(suffix)
}
