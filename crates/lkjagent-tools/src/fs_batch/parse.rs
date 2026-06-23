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
    JsonArray,
    JsonObjectFiles,
}

impl InputFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            InputFormat::LineProtocol => "line-protocol",
            InputFormat::JsonArray => "json-array",
            InputFormat::JsonObjectFiles => "json-object-files",
        }
    }
}

pub fn parse_files(input: &str) -> ToolResult<ParsedBatch> {
    let trimmed = input.trim();
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        return parse_json_files(trimmed);
    }
    parse_line_protocol(input)
}

fn parse_json_files(input: &str) -> ToolResult<ParsedBatch> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|_| ToolError::invalid("invalid JSON fs.batch_write files payload"))?;
    match value {
        serde_json::Value::Array(values) => Ok(ParsedBatch {
            files: json_array(values)?,
            format: InputFormat::JsonArray,
        }),
        serde_json::Value::Object(mut object) => {
            let Some(serde_json::Value::Array(values)) = object.remove("files") else {
                return Err(ToolError::invalid(
                    "JSON object files payload needs files array",
                ));
            };
            Ok(ParsedBatch {
                files: json_array(values)?,
                format: InputFormat::JsonObjectFiles,
            })
        }
        _ => Err(ToolError::invalid(
            "JSON files payload needs array or object",
        )),
    }
}

fn json_array(values: Vec<serde_json::Value>) -> ToolResult<Vec<BatchFile>> {
    let mut out = Vec::new();
    for value in values {
        let serde_json::Value::Object(mut object) = value else {
            return Err(ToolError::invalid("each JSON file must be an object"));
        };
        let path = take_string(&mut object, "path")?;
        let content = take_string(&mut object, "content")?;
        out.push(BatchFile { path, content });
    }
    if out.is_empty() {
        return Err(ToolError::invalid(
            "files must contain at least one file block",
        ));
    }
    Ok(out)
}

fn take_string(
    object: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> ToolResult<String> {
    match object.remove(key) {
        Some(serde_json::Value::String(value)) if !value.trim().is_empty() => Ok(value),
        _ => Err(ToolError::invalid("each JSON file needs path and content")),
    }
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
    let path = trimmed
        .strip_prefix("path:")
        .map(str::trim)
        .or_else(|| xml_path(trimmed))
        .or_else(|| angled_path(trimmed));
    let Some(path) = path.filter(|path| !path.is_empty()) else {
        return Err(ToolError::invalid("each block must start with path: "));
    };
    Ok(path.to_string())
}

fn xml_path(header: &str) -> Option<&str> {
    header
        .strip_prefix("<path>")
        .and_then(|rest| rest.strip_suffix("</path>"))
        .map(str::trim)
}

fn angled_path(header: &str) -> Option<&str> {
    header
        .strip_prefix("<path:")
        .map(|path| path.trim_end_matches('>').trim())
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
