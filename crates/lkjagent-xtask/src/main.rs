use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = lkjagent_xtask::run(&args, Path::new("."));
    std::process::exit(code);
}
