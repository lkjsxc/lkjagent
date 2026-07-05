use crate::args::Command;

pub fn parse_record(rest: Vec<String>) -> Result<Command, String> {
    match rest.as_slice() {
        [action, kind, title @ ..] if action == "add" && !title.is_empty() => {
            let (title, body) = title_body(title)?;
            Ok(Command::RecordAdd {
                kind: kind.clone(),
                title,
                body,
            })
        }
        [action] if action == "list" => Ok(Command::RecordList { kind: None }),
        [action, kind] if action == "list" => Ok(Command::RecordList {
            kind: Some(kind.clone()),
        }),
        [action, id] if action == "show" => Ok(Command::RecordShow { id: id.clone() }),
        [action, id] if action == "archive" => Ok(Command::RecordArchive { id: id.clone() }),
        [action, id, target] if action == "link" => Ok(Command::RecordLink {
            id: id.clone(),
            target: target.clone(),
        }),
        _ => Err("use record add KIND TITLE [--body TEXT] | list [KIND] | show ID | link ID REF | archive ID".to_string()),
    }
}

fn title_body(parts: &[String]) -> Result<(String, String), String> {
    let mut title = Vec::new();
    let mut body = Vec::new();
    let mut in_body = false;
    for part in parts {
        if part == "--body" {
            in_body = true;
        } else if in_body {
            body.push(part.clone());
        } else {
            title.push(part.clone());
        }
    }
    if title.is_empty() {
        Err("record add requires TITLE".to_string())
    } else {
        Ok((title.join(" "), body.join(" ")))
    }
}
