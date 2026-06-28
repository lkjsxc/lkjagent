use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedBatch {
    pub files: Vec<BatchFile>,
    pub format: InputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    LineProtocol,
}

impl InputFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            InputFormat::LineProtocol => "line-protocol",
        }
    }
}

pub fn parse_files(input: &str) -> ToolResult<ParsedBatch> {
    let trimmed = input.trim();
    if has_child_file_tags(trimmed) {
        return Err(ToolError::invalid(
            "schema fault: unsupported <file> child tags in fs.batch_write files; use path: and content: line protocol",
        ));
    }
    if starts_object_literal(trimmed) {
        return Err(ToolError::invalid(
            "schema fault: object-literal fs.batch_write files are not live output; use path: and content: line protocol",
        ));
    }
    parse_line_protocol(input)
}

fn has_child_file_tags(input: &str) -> bool {
    input.contains("<file>") || input.contains("</file>")
}

fn starts_object_literal(input: &str) -> bool {
    input.starts_with('[') || input.starts_with('{')
}

fn parse_line_protocol(input: &str) -> ToolResult<ParsedBatch> {
    let mut files = Vec::new();
    for block in input.split("-- lkjagent-next-file --") {
        let Some(file) = parse_block(block)? else {
            continue;
        };
        files.push(file);
    }
    if files.is_empty() {
        return Err(ToolError::invalid(
            "files must contain at least one file block",
        ));
    }
    Ok(ParsedBatch {
        files,
        format: InputFormat::LineProtocol,
    })
}

fn parse_block(block: &str) -> ToolResult<Option<BatchFile>> {
    let trimmed = block.trim_start_matches(['\n', '\r', ' ', '\t']);
    if trimmed.trim().is_empty() {
        return Ok(None);
    }
    let Some((header, body)) = trimmed.split_once('\n') else {
        return Err(ToolError::invalid("each block needs content:"));
    };
    let path = parse_path_header(header)?;
    let content = parse_content(body)?;
    Ok(Some(BatchFile {
        path,
        content: content.to_string(),
    }))
}

fn parse_path_header(header: &str) -> ToolResult<String> {
    let trimmed = header.trim();
    let path = trimmed.strip_prefix("path:").map(str::trim);
    let Some(path) = path.filter(|path| !path.is_empty()) else {
        return Err(ToolError::invalid("each block must start with path: "));
    };
    Ok(path.to_string())
}

fn parse_content(body: &str) -> ToolResult<&str> {
    if let Some(content) = body.strip_prefix("content:\n") {
        return Ok(content);
    }
    if let Some(content) = body.strip_prefix("content:\r\n") {
        return Ok(content);
    }
    Err(ToolError::invalid("each block needs content:"))
}
