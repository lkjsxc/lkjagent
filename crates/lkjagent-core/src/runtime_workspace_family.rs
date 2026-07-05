pub(crate) fn operation(namespace: &str) -> Option<(&'static str, u8, &'static str)> {
    match namespace {
        "todo" => Some(("todo.review", 35, "todo")),
        "calendar" => Some(("calendar.review", 36, "calendar")),
        "routine" => Some(("routine.run", 37, "routine")),
        "index" => Some(("index.rebuild", 38, "index")),
        "proof" => Some(("proof.collect", 39, "proof")),
        "dev" => Some(("dev.review", 40, "dev")),
        "project" => Some(("project.advance", 41, "project")),
        _ => None,
    }
}
