use crate::error::ParseResult;
use crate::tag_line::{classify_tag_line, TagLineClass};

use super::{open_name_and_tail, parse_pair};

pub(super) fn parse_batch_files_pair(
    lines: &[&str],
    index: usize,
) -> ParseResult<(String, String, usize)> {
    let mut cursor = index;
    let mut chunks = Vec::new();
    loop {
        let (name, value, next) = parse_pair(lines, cursor)?;
        if name != "files" {
            return Ok((name, value, next));
        }
        if !value.trim().is_empty() {
            chunks.push(value);
        }
        cursor = skip_extra_closes(lines, next);
        if !lines
            .get(cursor)
            .is_some_and(|line| starts_files_pair(line))
        {
            break;
        }
    }
    Ok((
        "files".to_string(),
        chunks.join("\n-- lkjagent-next-file --\n"),
        cursor,
    ))
}

pub(super) fn starts_files_pair(line: &str) -> bool {
    match classify_tag_line(line) {
        TagLineClass::InlineTag { name, .. } | TagLineClass::OpenTag { name } => name == "files",
        _ => open_name_and_tail(line).is_some_and(|(name, _)| name == "files"),
    }
}

fn skip_extra_closes(lines: &[&str], mut cursor: usize) -> usize {
    while lines
        .get(cursor)
        .is_some_and(|line| line.trim_end() == "</files>")
    {
        cursor += 1;
    }
    cursor
}
