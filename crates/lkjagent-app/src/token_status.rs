use rusqlite::Connection;

pub fn token_line(conn: &Connection) -> Result<String, String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM token_usage", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if count == 0 {
        return Ok("unknown".to_string());
    }
    let row: (
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        i64,
        i64,
        i64,
        i64,
    ) = conn
        .query_row(
            "SELECT SUM(input_total_tokens), SUM(input_cached_tokens),
             SUM(input_uncached_tokens), SUM(output_tokens),
             SUM(cache_status = 'known'), SUM(cache_status = 'unknown'),
             SUM(cache_status = 'provider_specific'), SUM(cache_status = 'not_supported')
             FROM token_usage",
            [],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                ))
            },
        )
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "input_uncached={} input_cached={} input_total={} output={} cache={}",
        fmt_token(row.2),
        fmt_token(row.1),
        fmt_token(row.0),
        fmt_token(row.3),
        cache_label(row.4, row.5, row.6, row.7)
    ))
}

fn fmt_token(value: Option<i64>) -> String {
    value.map_or_else(|| "unknown".to_string(), |value| value.to_string())
}

fn cache_label(known: i64, unknown: i64, provider: i64, unsupported: i64) -> &'static str {
    if provider > 0 {
        "provider_specific"
    } else if unsupported > 0 {
        "not_supported"
    } else if unknown > 0 || known == 0 {
        "unknown"
    } else {
        "known"
    }
}
