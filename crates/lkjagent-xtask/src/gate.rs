pub enum Gate {
    CheckDocs,
    CheckLines,
    CheckFiles,
    CheckStyle,
    QuietTest,
    QuietVerify,
    HygieneCheck,
    Benchmark(Vec<String>),
    Experiment(Vec<String>),
    Proof(Vec<String>),
    Smoke(Vec<String>),
    Structure(Vec<String>),
}

pub fn parse_gate(args: &[String]) -> Result<Gate, Vec<String>> {
    match args {
        [one] if one == "check-docs" || one == "docs-check" => Ok(Gate::CheckDocs),
        [one] if one == "check-lines" => Ok(Gate::CheckLines),
        [one] if one == "check-files" => Ok(Gate::CheckFiles),
        [one] if one == "check-style" => Ok(Gate::CheckStyle),
        [one] if one == "hygiene-check" => Ok(Gate::HygieneCheck),
        [first, second] if first == "quiet" && second == "test" => Ok(Gate::QuietTest),
        [first, second] if first == "quiet" && second == "verify" => Ok(Gate::QuietVerify),
        [first, rest @ ..] if first == "benchmark" || first == "bench" => {
            Ok(Gate::Benchmark(rest.to_vec()))
        }
        [first, rest @ ..] if first == "experiment" => Ok(Gate::Experiment(rest.to_vec())),
        [first, rest @ ..] if first == "proof" => Ok(Gate::Proof(rest.to_vec())),
        [first, rest @ ..] if first == "smoke" => Ok(Gate::Smoke(rest.to_vec())),
        [first, rest @ ..] if first == "structure" => Ok(Gate::Structure(rest.to_vec())),
        _ => Err(vec![
            "xtask failed".to_string(),
            "exit status: 2".to_string(),
            "use: check-docs | check-lines | check-files | check-style | hygiene-check | quiet test | quiet verify | bench ... | experiment ... | proof ... | smoke ... | structure ...".to_string(),
        ]),
    }
}
