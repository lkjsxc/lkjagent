pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const CYAN: &str = "\x1b[36m";
pub const YELLOW: &str = "\x1b[33m";

pub fn muted(text: &str) -> String {
    format!("{DIM}{text}{RESET}")
}

pub fn prompt(state: &str) -> String {
    let label = "send>";
    let color = if state == "waiting" { YELLOW } else { CYAN };
    format!("{BOLD}{color}{label}{RESET}")
}
