const GROUP_SIZE: usize = 10;

pub(crate) fn design_path(index: usize) -> String {
    format!("docs/designs/{}/design-{index:03}.md", design_group(index))
}

pub(crate) fn main_path(index: usize) -> String {
    format!("main/arcs/{}/part-{index:03}.md", main_group(index))
}

pub(crate) fn design_group(index: usize) -> String {
    group_name("set", index)
}

pub(crate) fn main_group(index: usize) -> String {
    group_name("arc", index)
}

fn group_name(prefix: &str, index: usize) -> String {
    let group = index.saturating_sub(1) / GROUP_SIZE + 1;
    format!("{prefix}-{group:03}")
}
