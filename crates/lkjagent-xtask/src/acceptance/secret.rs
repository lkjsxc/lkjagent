pub fn kind(bytes: &[u8]) -> Option<&'static str> {
    let lower = bytes.iter().map(u8::to_ascii_lowercase).collect::<Vec<_>>();
    if authorization(&lower) {
        return Some("authorization");
    }
    let private_key = [
        45, 45, 45, 45, 45, 98, 101, 103, 105, 110, 32, 112, 114, 105, 118, 97, 116, 101, 32, 107,
        101, 121, 45, 45, 45, 45, 45,
    ];
    if contains(&lower, &private_key) {
        return Some("private-key");
    }
    if token_after(&lower, b"sk-", 16)
        || token_after(&lower, b"ghp_", 20)
        || token_after(&lower, b"github_pat_", 20)
        || aws_key(bytes)
    {
        return Some("credential");
    }
    None
}

pub fn contains_loaded(bytes: &[u8]) -> bool {
    std::env::vars()
        .filter(|(name, value)| secret_name(name) && value.len() >= 8)
        .any(|(_, value)| {
            bytes
                .windows(value.len())
                .any(|part| part == value.as_bytes())
        })
}

fn authorization(bytes: &[u8]) -> bool {
    bytes.split(|byte| *byte == b'\n').any(authorization_line)
}

fn authorization_line(line: &[u8]) -> bool {
    let marker = b"authorization";
    let Some(at) = line.windows(marker.len()).position(|part| part == marker) else {
        return false;
    };
    let after = trim_ascii(&line[at + marker.len()..]);
    let Some(value) = after.strip_prefix(b":") else {
        return false;
    };
    let value = trim_ascii(value);
    credential_after(value, b"bearer ", 24) || credential_after(value, b"basic ", 24)
}

fn credential_after(bytes: &[u8], prefix: &[u8], minimum: usize) -> bool {
    bytes.strip_prefix(prefix).is_some_and(|value| {
        value
            .iter()
            .take_while(|byte| !byte.is_ascii_whitespace())
            .count()
            >= minimum
    })
}

fn trim_ascii(mut bytes: &[u8]) -> &[u8] {
    while bytes.first().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[1..];
    }
    bytes
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn token_after(bytes: &[u8], prefix: &[u8], minimum: usize) -> bool {
    bytes
        .windows(prefix.len())
        .enumerate()
        .filter(|(_, part)| *part == prefix)
        .any(|(at, _)| {
            let boundary = at == 0 || !bytes[at - 1].is_ascii_alphanumeric();
            boundary
                && bytes[at + prefix.len()..]
                    .iter()
                    .take_while(|byte| {
                        byte.is_ascii_alphanumeric() || **byte == b'_' || **byte == b'-'
                    })
                    .count()
                    >= minimum
        })
}

fn aws_key(bytes: &[u8]) -> bool {
    bytes.windows(20).any(|part| {
        part.starts_with(b"AKIA")
            && part[4..]
                .iter()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
    })
}

fn secret_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper.contains("SECRET")
        || upper.contains("TOKEN")
        || upper.contains("PASSWORD")
        || upper.ends_with("API_KEY")
}
