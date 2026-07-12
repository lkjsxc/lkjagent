use crate::parse::{ParseFault, PlanLine};

pub(crate) fn parse_plan(body: &str) -> Result<Vec<PlanLine>, ParseFault> {
    logical_lines(body)
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_plan_line(&line))
        .collect()
}

fn logical_lines(body: &str) -> Vec<String> {
    body.lines()
        .flat_map(|line| {
            line.replace(", write ", "\nwrite ")
                .replace(", explore |", "\nexplore |")
                .replace(", respond |", "\nrespond |")
                .lines()
                .map(|part| part.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn parse_plan_line(line: &str) -> Result<PlanLine, ParseFault> {
    let parts = line.split(" | ").collect::<Vec<_>>();
    match parts.as_slice() {
        [left, title, words] if left.starts_with("write ") => write_line(line, left, title, words),
        ["explore", goal, budget] => explore_line(line, goal, budget),
        ["respond", parts @ ..] if !parts.is_empty() => {
            let summary = parts.join(" | ");
            if !concrete(&summary, &["SUMMARY"]) {
                return Err(ParseFault::BadPlanLine(line.to_string()));
            }
            Ok(PlanLine::Respond { summary })
        }
        _ => Err(ParseFault::BadPlanLine(line.to_string())),
    }
}

fn write_line(line: &str, left: &str, title: &str, words: &str) -> Result<PlanLine, ParseFault> {
    let path = left.trim_start_matches("write ").to_string();
    let Some(number) = words.strip_prefix("words=") else {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    };
    if !safe_path(&path) || !concrete(title, &["TITLE"]) {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    }
    let Ok(words) = number.parse::<usize>() else {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    };
    if words == 0 {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    }
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
    if budget == 0 || !concrete(goal, &["GOAL"]) {
        return Err(ParseFault::BadPlanLine(line.to_string()));
    }
    Ok(PlanLine::Explore {
        goal: goal.to_string(),
        budget,
    })
}

fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && path.split('/').all(|part| {
            !part.is_empty() && part != "." && part != ".." && !part.eq_ignore_ascii_case("PATH")
        })
}

fn concrete(value: &str, placeholders: &[&str]) -> bool {
    let value = value.trim();
    !value.is_empty()
        && !placeholders
            .iter()
            .any(|placeholder| value.eq_ignore_ascii_case(placeholder))
}
