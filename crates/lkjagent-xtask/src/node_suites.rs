use std::path::Path;
use std::process::Command;

pub struct Suite {
    pub package: &'static str,
    pub target: &'static str,
    pub minimum_tests: usize,
}

pub fn check(root: &Path, node: &str, suites: &[Suite]) -> Result<(), Vec<String>> {
    if suites.is_empty() {
        return Err(vec![format!("{node} has no named suites")]);
    }
    let failures = suites
        .iter()
        .filter_map(|suite| run_suite(root, node, suite))
        .collect::<Vec<_>>();
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

fn run_suite(root: &Path, node: &str, suite: &Suite) -> Option<String> {
    let args = [
        "test",
        "--locked",
        "-p",
        suite.package,
        "--test",
        suite.target,
    ];
    let output = Command::new("cargo").args(args).current_dir(root).output();
    let label = format!("{node} suite {}:{}", suite.package, suite.target);
    let output = match output {
        Ok(output) => output,
        Err(error) => return Some(format!("{label} could not start: {error}")),
    };
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if !output.status.success() {
        return Some(format!("{label} failed: {}", tail(&text)));
    }
    let passed = passed_tests(&text);
    if passed < suite.minimum_tests {
        return Some(format!(
            "{label} ran {passed} tests, expected at least {}",
            suite.minimum_tests
        ));
    }
    None
}

fn passed_tests(text: &str) -> usize {
    text.lines()
        .filter_map(|line| line.trim().strip_prefix("test result: ok. "))
        .filter_map(|line| line.split_once(" passed;"))
        .filter_map(|(count, _)| count.trim().parse::<usize>().ok())
        .sum()
}

fn tail(text: &str) -> String {
    let lines = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let start = lines.len().saturating_sub(20);
    lines[start..].join(" | ")
}

#[cfg(test)]
mod tests {
    use super::passed_tests;

    #[test]
    fn sums_only_passing_test_result_rows() {
        let text =
            "test result: ok. 3 passed; 0 failed;\nother\ntest result: ok. 2 passed; 0 failed;";
        assert_eq!(passed_tests(text), 5);
    }

    #[test]
    fn ignores_non_passing_and_malformed_rows() {
        let text = "test result: FAILED. 3 passed; 1 failed;\ntest result: ok. nope passed;";
        assert_eq!(passed_tests(text), 0);
    }
}
