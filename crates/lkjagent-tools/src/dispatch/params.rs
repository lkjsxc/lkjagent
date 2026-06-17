use std::collections::BTreeMap;

use crate::error::{ToolError, ToolResult};

pub fn param(params: &BTreeMap<String, String>, name: &str) -> String {
    params.get(name).map_or_else(String::new, Clone::clone)
}

pub fn parse_usize(value: &str) -> ToolResult<usize> {
    value
        .parse::<usize>()
        .map_err(|_| ToolError::invalid(format!("expected positive integer: {value}")))
}

pub fn parse_i64(value: &str) -> ToolResult<i64> {
    value
        .parse::<i64>()
        .map_err(|_| ToolError::invalid(format!("expected integer id: {value}")))
}

pub fn parse_u64(value: &str) -> ToolResult<u64> {
    value
        .parse::<u64>()
        .map_err(|_| ToolError::invalid(format!("expected positive timeout: {value}")))
}
