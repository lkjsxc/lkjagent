use super::plan::StructurePlan;

pub fn render_audit(plan: &StructurePlan) -> Vec<String> {
    let mut lines = Vec::new();
    if plan.findings.is_empty() {
        lines.push(format!("ok structure audit {}", plan.root));
        return lines;
    }
    lines.push(format!(
        "structure audit {} findings={}",
        plan.root,
        plan.findings.len()
    ));
    lines.extend(plan.findings.iter().map(|finding| finding.message()));
    lines
}

pub fn render_plan(plan: &StructurePlan) -> Vec<String> {
    let mut lines = render_audit(plan);
    lines.push("verification commands:".to_string());
    lines.extend(
        plan.verification_commands
            .iter()
            .map(|command| format!("- {command}")),
    );
    lines
}
