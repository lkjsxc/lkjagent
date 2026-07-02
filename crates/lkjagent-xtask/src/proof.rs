use std::path::Path;

pub fn run(_args: &[String], _root: &Path) -> i32 {
    eprintln!("proof failed");
    eprintln!("exit status: 1");
    eprintln!("proof bundle collection is not yet rebuilt for the plan engine");
    1
}
