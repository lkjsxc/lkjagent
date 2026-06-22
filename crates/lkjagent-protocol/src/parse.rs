use crate::error::ParseResult;
use crate::line_parse::{parse_line_act, starts_line_act};
use crate::model::{Action, ParseFault};
use crate::xml_parse::{is_open, parse_xml_act};

pub fn parse_completion(text: &str) -> ParseResult<Action> {
    let lines: Vec<&str> = text.lines().collect();
    let start = find_act_start(&lines)?;
    let body = start + 1;
    let (action, next) = if starts_line_act(&lines, body) {
        parse_line_act(&lines, body)?
    } else {
        parse_xml_act(&lines, body)?
    };
    if lines.iter().skip(next).any(|line| is_open(line, "act")) {
        Err(ParseFault::MultipleAct)
    } else {
        Ok(action)
    }
}

fn find_act_start(lines: &[&str]) -> ParseResult<usize> {
    lines
        .iter()
        .position(|line| is_open(line, "act"))
        .ok_or(ParseFault::MissingAct)
}
