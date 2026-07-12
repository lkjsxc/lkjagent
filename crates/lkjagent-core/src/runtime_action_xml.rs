#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionXmlError {
    Malformed,
    Attribute,
    ForbiddenSyntax,
    SelfClosing,
    Unclosed,
    Crossed,
    Nested,
    BadEntity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element<'a> {
    pub name: &'a str,
    pub text: &'a str,
    pub children: Vec<Element<'a>>,
}

pub fn parse_document(src: &str) -> Result<Element<'_>, ActionXmlError> {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return Err(ActionXmlError::Malformed);
    }
    let (root, at) = parse_element(trimmed, 0)?;
    if at != trimmed.len() {
        return Err(ActionXmlError::Malformed);
    }
    Ok(root)
}

fn parse_element(src: &str, at: usize) -> Result<(Element<'_>, usize), ActionXmlError> {
    let (name, mut cursor) = opening(src, at)?;
    let text_start = cursor;
    let mut children = Vec::new();
    loop {
        let Some(relative) = src[cursor..].find('<') else {
            return Err(ActionXmlError::Unclosed);
        };
        let lt = cursor + relative;
        if src[lt..].starts_with("</") {
            let (closing, end) = closing(src, lt)?;
            if closing != name {
                return Err(ActionXmlError::Crossed);
            }
            let text = if children.is_empty() {
                &src[text_start..lt]
            } else {
                if !src[cursor..lt].trim().is_empty() {
                    return Err(ActionXmlError::Nested);
                }
                ""
            };
            return Ok((
                Element {
                    name,
                    text,
                    children,
                },
                end,
            ));
        }
        if forbidden(&src[lt..]) {
            return Err(ActionXmlError::ForbiddenSyntax);
        }
        if children.is_empty() && !src[text_start..lt].trim().is_empty() {
            return Err(ActionXmlError::Nested);
        }
        let (child, end) = parse_element(src, lt)?;
        children.push(child);
        cursor = end;
    }
}

fn opening(src: &str, at: usize) -> Result<(&str, usize), ActionXmlError> {
    if forbidden(&src[at..]) {
        return Err(ActionXmlError::ForbiddenSyntax);
    }
    if !src[at..].starts_with('<') {
        return Err(ActionXmlError::Malformed);
    }
    let end = src[at..]
        .find('>')
        .map(|value| at + value)
        .ok_or(ActionXmlError::Malformed)?;
    let token = &src[at + 1..end];
    if token.ends_with('/') {
        return Err(ActionXmlError::SelfClosing);
    }
    if token
        .bytes()
        .any(|byte| byte.is_ascii_whitespace() || byte == b'=')
    {
        return Err(ActionXmlError::Attribute);
    }
    if !valid_name(token) {
        return Err(ActionXmlError::Malformed);
    }
    Ok((token, end + 1))
}

fn closing(src: &str, at: usize) -> Result<(&str, usize), ActionXmlError> {
    let end = src[at..]
        .find('>')
        .map(|value| at + value)
        .ok_or(ActionXmlError::Unclosed)?;
    let token = &src[at + 2..end];
    if !valid_name(token) {
        return Err(ActionXmlError::Malformed);
    }
    Ok((token, end + 1))
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
}

fn forbidden(src: &str) -> bool {
    src.starts_with("<!--")
        || src.starts_with("<![CDATA[")
        || src.starts_with("<?")
        || src.starts_with("<!")
}

pub fn decode_entities(text: &str) -> Result<String, ActionXmlError> {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + 1..];
        let end = after.find(';').ok_or(ActionXmlError::BadEntity)?;
        let decoded = match &after[..end] {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "apos" => '\'',
            "quot" => '"',
            _ => return Err(ActionXmlError::BadEntity),
        };
        out.push(decoded);
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    if contains_entity(&out) {
        return Err(ActionXmlError::BadEntity);
    }
    Ok(out)
}

fn contains_entity(value: &str) -> bool {
    ["&amp;", "&lt;", "&gt;", "&apos;", "&quot;"]
        .iter()
        .any(|entity| value.contains(entity))
}
