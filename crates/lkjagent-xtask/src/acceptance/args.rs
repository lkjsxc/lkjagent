use std::path::PathBuf;

pub(super) struct Args {
    pub(super) source: String,
    pub(super) evidence: PathBuf,
    pub(super) allow_incomplete: bool,
}

pub(super) fn parse(args: &[String]) -> Result<Args, Vec<String>> {
    if args.first().is_none_or(|value| value != "verify") {
        return Err(usage("expected acceptance verify"));
    }
    let mut source = None;
    let mut evidence = None;
    let mut allow_incomplete = false;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--allow-incomplete" if !allow_incomplete => {
                allow_incomplete = true;
                index += 1;
            }
            "--source" if source.is_none() => {
                source = args.get(index + 1).cloned();
                index += 2;
            }
            "--evidence" if evidence.is_none() => {
                evidence = args.get(index + 1).map(PathBuf::from);
                index += 2;
            }
            flag => return Err(usage(&format!("unknown or duplicate argument: {flag}"))),
        }
    }
    match (source, evidence) {
        (Some(source), Some(evidence)) if !source.is_empty() => Ok(Args {
            source,
            evidence,
            allow_incomplete,
        }),
        _ => Err(usage("--source and --evidence are required")),
    }
}

pub(super) fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("{error}");
    }
}
fn usage(problem: &str) -> Vec<String> {
    vec![
        "acceptance command failed".to_string(),
        format!("error: {problem}"),
        "use: acceptance verify --source SOURCE --evidence PATH [--allow-incomplete]".to_string(),
    ]
}
