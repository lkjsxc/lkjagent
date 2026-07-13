mod args;
mod build_manifest;
mod campaign_evidence;
mod campaign_predicates;
mod command_evidence;
mod evidence;
mod experiments;
mod git;
mod history;
mod markers;
mod plans;
mod review;
mod secret;
mod source;
mod source_audit;
mod synthetic;
mod table;
mod workgraph;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use args::print_errors;

pub fn run(args: &[String], root: &Path) -> i32 {
    let parsed = match args::parse(args) {
        Ok(value) => value,
        Err(errors) => {
            print_errors(&errors);
            return 2;
        }
    };
    let report = verify(root, &parsed.source, &parsed.evidence);
    if report.errors.is_empty() && report.missing.is_empty() {
        println!("ok acceptance verify");
        return 0;
    }
    let mode = if parsed.allow_incomplete {
        "acceptance verify incomplete"
    } else {
        "acceptance verify failed"
    };
    eprintln!("{mode}");
    print_errors(&report.errors);
    for id in &report.missing {
        eprintln!("missing derivation: {id}");
    }
    1
}

#[derive(Debug, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub missing: Vec<String>,
}

pub fn verify(root: &Path, source: &str, evidence: &Path) -> Report {
    let mut report = Report::default();
    if let Err(error) = source::validate(root, source) {
        report.errors.push(error);
    }
    report.errors.extend(git::validate_plan_inputs(root));
    let required = match plans::validate(root) {
        Ok(value) => value,
        Err(errors) => {
            report.errors.extend(errors);
            Vec::new()
        }
    };
    let mut evidence_derived = BTreeSet::new();
    match git::evidence_files(root, source, evidence) {
        Ok(files) => {
            inspect_files(&files, source, &mut report.errors);
            for path in files {
                if let Ok(bytes) = fs::read(&path) {
                    evidence_derived.extend(evidence::derivations_at(root, &path, &bytes, source));
                }
            }
        }
        Err(errors) => report.errors.extend(errors),
    }
    let mut derived = static_derivations(root, source, &mut report.errors);
    derived.extend(evidence_derived);
    if derived.contains("E04-candidate")
        && ["E05", "E06", "E07", "E08", "E09"]
            .iter()
            .all(|id| derived.contains(*id))
        && source_audit::rejected_profiles_absent(root)
    {
        derived.insert("E04".into());
        derived.insert("S06".into());
    }
    if derived.contains("E17-candidate")
        && required
            .iter()
            .filter(|id| id.as_str() != "E17")
            .all(|id| derived.contains(id))
    {
        derived.insert("E17".into());
    }
    report.missing = required
        .into_iter()
        .filter(|id| !derived.contains(id))
        .collect();
    report.errors.sort();
    report.errors.dedup();
    report.missing.sort();
    report.missing.dedup();
    report
}

pub fn inspect_attachment(path: &Path, bytes: &[u8], source: &str) -> Vec<String> {
    evidence::inspect(path, bytes, source)
}
pub fn derive_attachment(path: &Path, bytes: &[u8], source: &str) -> BTreeSet<String> {
    evidence::derivations(path, bytes, source)
}
#[rustfmt::skip]
pub fn derive_campaign_attachment(root: &Path, path: &Path, bytes: &[u8], source: &str) -> BTreeSet<String> {
    evidence::derivations_at(root, path, bytes, source) }

pub fn validate_plans(root: &Path) -> Result<Vec<String>, Vec<String>> {
    plans::validate(root)
}

pub fn scan_history(root: &Path) -> Vec<String> {
    history::secret_errors(root)
}
pub fn source_contracts(root: &Path) -> BTreeSet<String> {
    source::contract_derivations(root)
}
pub fn source_contract_files() -> Vec<&'static str> {
    source::contract_files()
}

fn static_derivations(root: &Path, source: &str, errors: &mut Vec<String>) -> BTreeSet<String> {
    let mut derived = source::contract_derivations(root);
    derived.extend(history::derivations(root, source));
    if source::boundary_matches(root, source) {
        derived.insert("D01".into());
    }
    if synthetic::valid(root) {
        derived.insert("E15".into());
    }
    if crate::node_gate::check_docs_authority(root).is_ok() {
        derived.insert("D02".to_string());
    }
    if let Ok(files) = crate::facts::collect_files(root) {
        let docs = crate::docs::check_docs(&files);
        if docs.is_empty() {
            derived.insert("D04".to_string());
        }
        let lines = crate::lines::check_lines(&files);
        if lines.is_empty() {
            derived.insert("S01".to_string());
        }
    }
    let secret_errors = history::secret_errors(root);
    if secret_errors.is_empty() {
        derived.insert("E16".to_string());
    } else {
        errors.extend(secret_errors);
    }
    derived
}

fn inspect_files(files: &[PathBuf], source: &str, errors: &mut Vec<String>) {
    for path in files {
        match fs::read(path) {
            Ok(bytes) => errors.extend(evidence::inspect(path, &bytes, source)),
            Err(error) => errors.push(format!(
                "{}: cannot read evidence: {error}",
                path.to_string_lossy()
            )),
        }
    }
}
