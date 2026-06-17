use std::fs;
use std::path::Path;

use crate::error::CliError;

pub fn load(path: &Path) -> Result<(), CliError> {
    if !path.exists() {
        return Ok(());
    }
    let text = fs::read_to_string(path)?;
    for assignment in parse(&text)? {
        if std::env::var_os(&assignment.key).is_none() {
            std::env::set_var(assignment.key, assignment.value);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub key: String,
    pub value: String,
}

pub fn parse(text: &str) -> Result<Vec<Assignment>, CliError> {
    let mut assignments = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let line = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((key, value)) = line.split_once('=') else {
            return Err(CliError::failure(format!(
                ".env line {} is missing '='",
                index + 1
            )));
        };
        let key = key.trim();
        if !valid_key(key) {
            return Err(CliError::failure(format!(
                ".env line {} has an invalid key",
                index + 1
            )));
        }
        assignments.push(Assignment {
            key: key.to_string(),
            value: clean_value(value.trim())?,
        });
    }
    Ok(assignments)
}

fn valid_key(key: &str) -> bool {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn clean_value(value: &str) -> Result<String, CliError> {
    if let Some(inner) = value.strip_prefix('"') {
        return quoted(inner, '"');
    }
    if let Some(inner) = value.strip_prefix('\'') {
        return quoted(inner, '\'');
    }
    Ok(value.to_string())
}

fn quoted(value: &str, quote: char) -> Result<String, CliError> {
    let Some(inner) = value.strip_suffix(quote) else {
        return Err(CliError::failure(".env has an unterminated quoted value"));
    };
    Ok(inner.to_string())
}
