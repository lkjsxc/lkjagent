use crate::parse::{ParseFault, PlanLine};

pub(crate) fn parse_plan(body: &str) -> Result<Vec<PlanLine>, ParseFault> {
    body.lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_plan_line)
        .collect()
}

fn parse_plan_line(line: &str) -> Result<PlanLine, ParseFault> {
    let parts = line.split(" | ").collect::<Vec<_>>();
    match parts.as_slice() {
        [left, title, words] if left.starts_with("write ") => write_line(line, left, title, words),
        ["explore", goal, budget] => explore_line(line, goal, budget),
        ["respond", summary] => Ok(PlanLine::Respond {
            summary: (*summary).to_string(),
        }),
        _ => Err(ParseFault::BadPlanLine(line.to_string())),
    }
}

fn write_line(line: &str, left: &str, title: &str, words: &str) -> Result<PlanLine, ParseFault> {
    let path = left.trim_start_matches("write ").to_string();
    let Some(number) = words.strip_prefix("words=") else {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    };
    if path.starts_with('/') || path.contains("..") {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    }
    let Ok(words) = number.parse::<usize>() else {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    };
    Ok(PlanLine::Write {
        path,
        title: title.to_string(),
        words,
    })
}

fn explore_line(line: &str, goal: &str, budget: &str) -> Result<PlanLine, ParseFault> {
    let Some(number) = budget.strip_prefix("budget=") else {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    };
    let Ok(budget) = number.parse::<u32>() else {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    };
    Ok(PlanLine::Explore {
        goal: goal.to_string(),
        budget,
    })
}
