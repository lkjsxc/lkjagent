use std::{fs, path::Path};

#[derive(Debug)]
pub struct PtyFacts {
    pub cast_fingerprint: String,
    pub frame_count: usize,
    pub input_frames: usize,
    pub output_frames: usize,
    pub resize_frames: usize,
    pub japanese_inputs: usize,
    pub search_inputs: usize,
    pub alt_screen_enter: usize,
    pub alt_screen_exit: usize,
    pub activity_frames: usize,
    pub slow_interval_ms: u64,
}

pub fn validate(path: &Path) -> Result<PtyFacts, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    if bytes.len() > 1_048_576 || bytes.contains(&0) {
        return Err("PTY cast is not a bounded text capture".into());
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| "PTY cast is not UTF-8")?;
    let mut lines = text.lines();
    let header: serde_json::Value = serde_json::from_str(lines.next().ok_or("PTY cast is empty")?)
        .map_err(|_| "PTY cast header is malformed")?;
    if header["version"].as_u64() != Some(2) {
        return Err("PTY cast version is unsupported".into());
    }
    let mut facts = PtyFacts::empty(super::hash::bytes(&bytes));
    let mut slow_start = None;
    for line in lines.take(10_000) {
        let row: serde_json::Value =
            serde_json::from_str(line).map_err(|_| "PTY cast row is malformed")?;
        let values = row
            .as_array()
            .filter(|row| row.len() == 3)
            .ok_or("PTY cast row shape is invalid")?;
        let at = values[0]
            .as_f64()
            .filter(|value| value.is_finite() && *value >= 0.0)
            .ok_or("PTY cast time is invalid")?;
        let kind = values[1].as_str().ok_or("PTY cast event kind is invalid")?;
        let body = values[2].as_str().ok_or("PTY cast event body is invalid")?;
        facts.frame_count += 1;
        match kind {
            "i" => input(&mut facts, body),
            "o" => output(&mut facts, body),
            "r" => facts.resize_frames += 1,
            "m" if body == "slow-start" => slow_start = Some(at),
            "m" if body == "slow-end" => {
                if let Some(start) = slow_start.take() {
                    facts.slow_interval_ms = ((at - start) * 1000.0).round().max(0.0) as u64;
                }
            }
            "m" => {}
            _ => return Err("PTY cast event kind is unsupported".into()),
        }
    }
    if facts.frame_count == 0 || slow_start.is_some() {
        return Err("PTY cast is incomplete".into());
    }
    Ok(facts)
}

fn input(facts: &mut PtyFacts, body: &str) {
    facts.input_frames += 1;
    facts.japanese_inputs += usize::from(!body.is_ascii());
    facts.search_inputs += usize::from(body.contains('/') || body.contains('\u{6}'));
}
fn output(facts: &mut PtyFacts, body: &str) {
    facts.output_frames += 1;
    facts.alt_screen_enter += body.matches("\u{1b}[?1049h").count();
    facts.alt_screen_exit += body.matches("\u{1b}[?1049l").count();
    facts.activity_frames += usize::from(body.to_ascii_lowercase().contains("activity"));
}
impl PtyFacts {
    fn empty(cast_fingerprint: String) -> Self {
        Self {
            cast_fingerprint,
            frame_count: 0,
            input_frames: 0,
            output_frames: 0,
            resize_frames: 0,
            japanese_inputs: 0,
            search_inputs: 0,
            alt_screen_enter: 0,
            alt_screen_exit: 0,
            activity_frames: 0,
            slow_interval_ms: 0,
        }
    }
}
