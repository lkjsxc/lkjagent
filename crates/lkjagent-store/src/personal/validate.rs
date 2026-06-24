use crate::error::{StoreError, StoreResult};
use crate::personal::model::PersonalRecordInput;

pub fn validate_input(input: &PersonalRecordInput<'_>) -> StoreResult<()> {
    non_empty(input.title, "title")?;
    validate_kind(input.kind)?;
    validate_status(input.status)?;
    validate_optional_time(input.start_at, "start_at")?;
    validate_optional_time(input.end_at, "end_at")?;
    validate_optional_time(input.due_at, "due_at")?;
    validate_priority(input.priority)?;
    validate_recurrence(input.recurrence)?;
    match input.kind {
        "diary" => validate_diary(input),
        "schedule" => validate_schedule(input),
        "todo" => Ok(()),
        _ => invalid("unknown personal record kind"),
    }
}

pub fn validate_status(value: &str) -> StoreResult<()> {
    match value {
        "open" | "doing" | "waiting" | "done" | "canceled" => Ok(()),
        _ => invalid("unknown personal record status"),
    }
}

fn validate_diary(input: &PersonalRecordInput<'_>) -> StoreResult<()> {
    non_empty(input.body, "body")?;
    if let Some(date) = input.start_at {
        validate_date(date, "date")?;
    }
    Ok(())
}

fn validate_schedule(input: &PersonalRecordInput<'_>) -> StoreResult<()> {
    let Some(start) = input.start_at else {
        return invalid("schedule requires start_at");
    };
    validate_rfc3339(start, "start_at")?;
    if let Some(end) = input.end_at {
        validate_rfc3339(end, "end_at")?;
        if end <= start {
            return invalid("schedule end_at must be after start_at");
        }
    }
    Ok(())
}

fn validate_kind(value: &str) -> StoreResult<()> {
    match value {
        "diary" | "schedule" | "todo" => Ok(()),
        _ => invalid("unknown personal record kind"),
    }
}

fn validate_optional_time(value: Option<&str>, label: &str) -> StoreResult<()> {
    let Some(value) = value else {
        return Ok(());
    };
    if value.len() == 10 {
        validate_date(value, label)
    } else {
        validate_rfc3339(value, label)
    }
}

fn validate_date(value: &str, label: &str) -> StoreResult<()> {
    let bytes = value.as_bytes();
    if bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
    {
        Ok(())
    } else {
        invalid(&format!("{label} must be YYYY-MM-DD"))
    }
}

fn validate_rfc3339(value: &str, label: &str) -> StoreResult<()> {
    let has_date_time = value.len() >= 20 && value.as_bytes().get(10) == Some(&b'T');
    let has_zone = value.ends_with('Z') || zone_offset(value);
    if has_date_time && has_zone {
        Ok(())
    } else {
        invalid(&format!("{label} must be RFC3339 with timezone"))
    }
}

fn validate_priority(value: Option<&str>) -> StoreResult<()> {
    match value.filter(|item| !item.trim().is_empty()) {
        None | Some("low" | "normal" | "high" | "urgent") => Ok(()),
        Some(_) => invalid("unknown priority"),
    }
}

fn validate_recurrence(value: Option<&str>) -> StoreResult<()> {
    match value.filter(|item| !item.trim().is_empty()) {
        None => Ok(()),
        Some(item) if item.starts_with("RRULE:") => Ok(()),
        Some("daily" | "weekly" | "monthly") => Ok(()),
        Some(_) => invalid("unsupported recurrence"),
    }
}

fn zone_offset(value: &str) -> bool {
    value.len() >= 6
        && value.as_bytes()[value.len() - 3] == b':'
        && matches!(value.as_bytes()[value.len() - 6], b'+' | b'-')
}

fn non_empty(value: &str, label: &str) -> StoreResult<()> {
    if value.trim().is_empty() {
        invalid(&format!("personal record {label} must not be empty"))
    } else {
        Ok(())
    }
}

fn invalid(message: &str) -> StoreResult<()> {
    Err(StoreError::InvalidState(message.to_string()))
}
