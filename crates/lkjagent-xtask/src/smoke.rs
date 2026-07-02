use std::path::Path;

pub fn run(_args: &[String], _root: &Path) -> i32 {
    eprintln!("smoke failed");
    eprintln!("exit status: 1");
    eprintln!("smoke replay is not yet rebuilt for the plan engine");
    1
}
