use crate::case_context::ContextBinding;
use crate::model::{ContextPackage, GraphDefinition, PackagePriority};
use crate::policy::ContextPressureLevel;
use crate::state::TaskGraphState;

pub fn select_context_packages(
    graph: &GraphDefinition,
    state: &TaskGraphState,
) -> Vec<ContextBinding> {
    graph
        .packages
        .iter()
        .filter(|package| applies(package, state))
        .filter(|package| pressure_admits(package.priority, state.context.pressure))
        .map(|package| ContextBinding {
            package: package.id.0.to_string(),
            reason: format!(
                "node={} family={}",
                state.active_node.0,
                state.family.as_str()
            ),
            priority: priority_name(package.priority).to_string(),
        })
        .collect()
}

fn applies(package: &ContextPackage, state: &TaskGraphState) -> bool {
    package.applies_to.contains(&state.active_node) && package.families.contains(&state.family)
}

fn pressure_admits(priority: PackagePriority, pressure: ContextPressureLevel) -> bool {
    match pressure {
        ContextPressureLevel::Green => true,
        ContextPressureLevel::Yellow => !matches!(priority, PackagePriority::Helpful),
        ContextPressureLevel::Orange | ContextPressureLevel::Red => {
            matches!(priority, PackagePriority::Core | PackagePriority::Recovery)
        }
        ContextPressureLevel::BlackInvalid => false,
    }
}

fn priority_name(priority: PackagePriority) -> &'static str {
    match priority {
        PackagePriority::Core => "core",
        PackagePriority::Helpful => "helpful",
        PackagePriority::Recovery => "recovery",
    }
}
