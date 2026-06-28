use crate::error::ParseResult;
use crate::model::{
    Action, EnvelopeMode, ParseFault, ParseOutcome, ParseSettings, ACTION_CLOSE, ACTION_OPEN,
};
use crate::tag_line::{classify_tag_line, TagLineClass};
use crate::xml_parse::{is_action_open, parse_tag_action};

pub fn parse_completion(text: &str) -> ParseResult<Action> {
    let outcome = parse_live_completion(text, ParseSettings::default());
    match (outcome.action, outcome.fault) {
        (Some(action), None) => Ok(action),
        (_, Some(fault)) => Err(fault),
        (None, None) => Err(ParseFault::MissingActionEnvelope),
    }
}

pub fn parse_live_completion(text: &str, settings: ParseSettings) -> ParseOutcome {
    let parsed = parse_natural(text).or_else(|fault| parse_implicit(text, &settings, fault));
    let normalized_text_hash = text_hash(text);
    match parsed {
        Ok((action, envelope_mode)) => ParseOutcome {
            action: Some(action),
            fault: None,
            envelope_mode,
            normalized_text_hash,
        },
        Err(fault) => ParseOutcome {
            action: None,
            envelope_mode: envelope_mode_for_fault(&fault),
            fault: Some(fault),
            normalized_text_hash,
        },
    }
}

fn parse_natural(text: &str) -> ParseResult<(Action, EnvelopeMode)> {
    if text.trim_start().starts_with('{') {
        return Err(ParseFault::JsonActionRejected);
    }
    let lines: Vec<&str> = text.lines().collect();
    let start = find_action_start(&lines)?;
    reject_prefix(&lines, start)?;
    let body = start + 1;
    let (action, next) = parse_tag_action(&lines, body)?;
    reject_suffix(&lines, next)?;
    Ok((action, EnvelopeMode::Natural))
}

fn parse_implicit(
    text: &str,
    settings: &ParseSettings,
    original: ParseFault,
) -> ParseResult<(Action, EnvelopeMode)> {
    if !settings.allow_implicit_envelope || has_action_envelope(text) {
        return Err(original);
    }
    if !starts_implicit_body(text) {
        return Err(original);
    }
    let wrapped = format!("{ACTION_OPEN}\n{}\n{ACTION_CLOSE}", text.trim());
    parse_natural(&wrapped).map(|(action, _)| (action, EnvelopeMode::Implicit))
}

fn find_action_start(lines: &[&str]) -> ParseResult<usize> {
    lines
        .iter()
        .position(|line| matches!(classify_tag_line(line), TagLineClass::ActionOpen))
        .ok_or(ParseFault::MissingActionEnvelope)
}

fn reject_prefix(lines: &[&str], start: usize) -> ParseResult<()> {
    if lines.iter().take(start).any(|line| !line.trim().is_empty()) {
        return Err(ParseFault::BadEnvelope {
            reason: "prose before action envelope".to_string(),
        });
    }
    Ok(())
}

fn reject_suffix(lines: &[&str], next: usize) -> ParseResult<()> {
    if lines.iter().skip(next).any(|line| is_action_open(line)) {
        return Err(ParseFault::MultipleActionEnvelopes);
    }
    if lines.iter().skip(next).any(|line| !line.trim().is_empty()) {
        return Err(ParseFault::BadEnvelope {
            reason: "prose after action envelope".to_string(),
        });
    }
    Ok(())
}

fn has_action_envelope(text: &str) -> bool {
    text.lines().any(|line| {
        matches!(
            classify_tag_line(line),
            TagLineClass::ActionOpen | TagLineClass::ActionClose
        )
    })
}

fn starts_implicit_body(text: &str) -> bool {
    text.lines()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("<tool>")
        })
}

fn envelope_mode_for_fault(fault: &ParseFault) -> EnvelopeMode {
    match fault {
        ParseFault::UnclosedActionEnvelope | ParseFault::UnclosedTag { .. } => {
            EnvelopeMode::Unclosed
        }
        _ => EnvelopeMode::Natural,
    }
}

fn text_hash(text: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}
