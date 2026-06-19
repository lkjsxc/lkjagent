use lkjagent_protocol::Action;

pub(super) fn action_param(action: &Action, name: &str) -> String {
    action
        .params
        .iter()
        .find(|param| param.name == name)
        .map_or_else(String::new, |param| param.value.clone())
}
