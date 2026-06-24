use crate::model::{ACTION_CLOSE, ACTION_OPEN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagLineClass {
    ActionOpen,
    ActionClose,
    OpenTag {
        name: String,
    },
    InlineTag {
        name: String,
        value: String,
    },
    ClosingTag {
        name: String,
    },
    AttributeLikeTag {
        tag_name: String,
        value_hint: Option<String>,
    },
    MalformedAngleText,
    Payload,
}

pub fn classify_tag_line(line: &str) -> TagLineClass {
    let trimmed = line.trim_end();
    if trimmed == ACTION_OPEN {
        return TagLineClass::ActionOpen;
    }
    if trimmed == ACTION_CLOSE {
        return TagLineClass::ActionClose;
    }
    if let Some((name, value)) = inline_pair(trimmed) {
        return TagLineClass::InlineTag { name, value };
    }
    if let Some(name) = open_name(trimmed) {
        return TagLineClass::OpenTag { name };
    }
    if let Some(name) = closing_name(trimmed) {
        return TagLineClass::ClosingTag { name };
    }
    if let Some((tag_name, value_hint)) = attribute_like(trimmed) {
        return TagLineClass::AttributeLikeTag {
            tag_name,
            value_hint,
        };
    }
    if trimmed.contains('<') || trimmed.contains('>') {
        TagLineClass::MalformedAngleText
    } else {
        TagLineClass::Payload
    }
}

pub fn valid_tag_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

fn inline_pair(line: &str) -> Option<(String, String)> {
    let open_end = line.find('>')?;
    if !line.starts_with('<') || line.starts_with("</") {
        return None;
    }
    let name = &line[1..open_end];
    if !valid_tag_name(name) {
        return None;
    }
    let close = format!("</{name}>");
    if !line.ends_with(&close) {
        return None;
    }
    let value_start = open_end + 1;
    let value_end = line.len().saturating_sub(close.len());
    (value_start <= value_end).then(|| (name.to_string(), line[value_start..value_end].to_string()))
}

fn open_name(line: &str) -> Option<String> {
    if !line.starts_with('<') || line.starts_with("</") || !line.ends_with('>') {
        return None;
    }
    let name = &line[1..line.len().saturating_sub(1)];
    valid_tag_name(name).then(|| name.to_string())
}

fn closing_name(line: &str) -> Option<String> {
    if !line.starts_with("</") || !line.ends_with('>') {
        return None;
    }
    let name = &line[2..line.len().saturating_sub(1)];
    valid_tag_name(name).then(|| name.to_string())
}

fn attribute_like(line: &str) -> Option<(String, Option<String>)> {
    if !line.starts_with('<') || line.starts_with("</") {
        return None;
    }
    let end = line.find("</").or_else(|| line.find('>'))?;
    let tag_name = line[1..end].trim().to_string();
    if !tag_name.contains('=') {
        return None;
    }
    let value_hint = tag_name
        .split_once('=')
        .map(|(_, value)| value.trim().to_string())
        .filter(|value| !value.is_empty());
    Some((tag_name, value_hint))
}
