use super::model::ProofBundle;

pub fn warn_if_empty(bundle: &mut ProofBundle) {
    if bundle.cases.is_empty() {
        bundle.warnings.push("no graph case rows".to_string());
    }
    if bundle.readiness.is_empty() {
        bundle
            .warnings
            .push("no artifact readiness rows".to_string());
    }
    if bundle.decisions.is_empty() {
        bundle
            .warnings
            .push("no runtime authority decisions".to_string());
    }
}

pub fn compact(value: String) -> String {
    let mut out = value.lines().take(6).collect::<Vec<_>>().join(", ");
    if out.len() > 160 {
        out.truncate(160);
        out.push_str("...");
    }
    out
}
