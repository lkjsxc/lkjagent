pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const CYAN: &str = "\x1b[36m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";

pub fn muted(text: &str) -> String {
    format!("{DIM}{text}{RESET}")
}

pub fn title(text: &str) -> String {
    format!("{BOLD}{CYAN}{text}{RESET}")
}

pub fn state_badge(state: &str) -> String {
    let (color, label) = match state {
        "idle" => (GREEN, "IDLE"),
        "working" => (CYAN, "WORKING"),
        "waiting" => (YELLOW, "WAITING"),
        "error" => (RED, "ERROR"),
        _ => (DIM, "STOPPED"),
    };
    format!("{color}{label}{RESET}")
}

pub fn prompt(state: &str) -> String {
    let label = if state == "waiting" {
        "answer>"
    } else {
        "send>"
    };
    let color = if state == "waiting" { YELLOW } else { CYAN };
    format!("{BOLD}{color}{label}{RESET}")
}
