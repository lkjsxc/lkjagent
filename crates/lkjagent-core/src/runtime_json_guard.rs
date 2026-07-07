use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonGuardError {
    JsonMalformed,
    DuplicateKey(String),
}

pub fn reject_duplicate_keys(text: &str) -> Result<(), JsonGuardError> {
    let bytes = text.as_bytes();
    let mut at = 0;
    scan_value(bytes, &mut at, &mut Vec::new())
}

fn scan_value(bytes: &[u8], at: &mut usize, path: &mut Vec<String>) -> Result<(), JsonGuardError> {
    skip_ws(bytes, at);
    match bytes.get(*at) {
        Some(b'{') => scan_object(bytes, at, path),
        Some(b'[') => scan_array(bytes, at, path),
        Some(b'"') => parse_string(bytes, at).map(|_| ()),
        Some(_) => {
            while matches!(bytes.get(*at), Some(b) if !matches!(b, b',' | b']' | b'}')) {
                *at += 1;
            }
            Ok(())
        }
        None => Err(JsonGuardError::JsonMalformed),
    }
}

fn scan_object(bytes: &[u8], at: &mut usize, path: &mut Vec<String>) -> Result<(), JsonGuardError> {
    *at += 1;
    let mut seen = BTreeSet::new();
    loop {
        skip_ws(bytes, at);
        if matches!(bytes.get(*at), Some(b'}')) {
            *at += 1;
            return Ok(());
        }
        let key = parse_string(bytes, at)?;
        if !seen.insert(key.clone()) {
            return Err(JsonGuardError::DuplicateKey(pointer(path, &key)));
        }
        skip_ws(bytes, at);
        if !matches!(bytes.get(*at), Some(b':')) {
            return Err(JsonGuardError::JsonMalformed);
        }
        *at += 1;
        path.push(key);
        scan_value(bytes, at, path)?;
        path.pop();
        skip_ws(bytes, at);
        match bytes.get(*at) {
            Some(b',') => *at += 1,
            Some(b'}') => continue,
            _ => return Err(JsonGuardError::JsonMalformed),
        }
    }
}

fn scan_array(bytes: &[u8], at: &mut usize, path: &mut Vec<String>) -> Result<(), JsonGuardError> {
    *at += 1;
    let mut index = 0usize;
    loop {
        skip_ws(bytes, at);
        if matches!(bytes.get(*at), Some(b']')) {
            *at += 1;
            return Ok(());
        }
        path.push(index.to_string());
        scan_value(bytes, at, path)?;
        path.pop();
        index += 1;
        skip_ws(bytes, at);
        match bytes.get(*at) {
            Some(b',') => *at += 1,
            Some(b']') => continue,
            _ => return Err(JsonGuardError::JsonMalformed),
        }
    }
}

fn parse_string(bytes: &[u8], at: &mut usize) -> Result<String, JsonGuardError> {
    let start = *at;
    if !matches!(bytes.get(*at), Some(b'"')) {
        return Err(JsonGuardError::JsonMalformed);
    }
    *at += 1;
    while let Some(byte) = bytes.get(*at) {
        match byte {
            b'\\' => *at += 2,
            b'"' => {
                *at += 1;
                return serde_json::from_slice(&bytes[start..*at])
                    .map_err(|_| JsonGuardError::JsonMalformed);
            }
            _ => *at += 1,
        }
    }
    Err(JsonGuardError::JsonMalformed)
}

fn skip_ws(bytes: &[u8], at: &mut usize) {
    while matches!(bytes.get(*at), Some(b' ' | b'\n' | b'\r' | b'\t')) {
        *at += 1;
    }
}

fn pointer(path: &[String], key: &str) -> String {
    let mut parts = path
        .iter()
        .map(|part| escape_pointer(part))
        .collect::<Vec<_>>();
    parts.push(escape_pointer(key));
    format!("/{}", parts.join("/"))
}

fn escape_pointer(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
