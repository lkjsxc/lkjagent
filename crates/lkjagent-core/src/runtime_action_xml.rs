#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionXmlError {
    EnvelopeMalformed,
    UnclosedTag(String),
    CrossedTag(String),
    Attribute(String),
    UnknownTag(String),
    BadEntity(String),
}

pub fn next_element<'a>(
    src: &'a str,
    at: &mut usize,
    allow_children: bool,
) -> Result<Option<(&'a str, &'a str)>, ActionXmlError> {
    *at += src[*at..].len() - src[*at..].trim_start().len();
    if *at >= src.len() {
        return Ok(None);
    }
    if !src[*at..].starts_with('<') {
        return Err(ActionXmlError::EnvelopeMalformed);
    }
    let end = src[*at..]
        .find('>')
        .map(|pos| *at + pos)
        .ok_or(ActionXmlError::EnvelopeMalformed)?;
    let tag = &src[*at + 1..end];
    validate_tag(tag)?;
    let content_start = end + 1;
    let close = format!("</{tag}>");
    let mut scan = content_start;
    loop {
        let lt = src[scan..]
            .find('<')
            .map(|pos| scan + pos)
            .ok_or_else(|| ActionXmlError::UnclosedTag(tag.into()))?;
        if src[lt..].starts_with(&close) {
            *at = lt + close.len();
            return Ok(Some((tag, &src[content_start..lt])));
        }
        if src[lt..].starts_with("</") {
            return Err(ActionXmlError::CrossedTag(tag.into()));
        }
        if !allow_children {
            return Err(ActionXmlError::UnknownTag(peek_tag(&src[lt..])));
        }
        let mut child_at = lt;
        let _ = next_element(src, &mut child_at, false)?;
        scan = child_at;
    }
}

pub fn decode_entities(text: &str) -> Result<String, ActionXmlError> {
    let mut out = String::new();
    let mut rest = text;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + 1..];
        let Some(end) = after.find(';') else {
            return Err(ActionXmlError::BadEntity(rest[pos..].into()));
        };
        let entity = &after[..end];
        let decoded = match entity {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "apos" => '\'',
            "quot" => '"',
            _ => return Err(ActionXmlError::BadEntity(entity.into())),
        };
        out.push(decoded);
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

fn validate_tag(tag: &str) -> Result<(), ActionXmlError> {
    if tag.is_empty() || tag.starts_with('/') {
        return Err(ActionXmlError::EnvelopeMalformed);
    }
    if tag.chars().any(char::is_whitespace) || tag.contains('=') {
        return Err(ActionXmlError::Attribute(tag.to_string()));
    }
    Ok(())
}

fn peek_tag(src: &str) -> String {
    src.trim_start_matches('<')
        .split('>')
        .next()
        .unwrap_or("")
        .trim_start_matches('/')
        .to_string()
}
