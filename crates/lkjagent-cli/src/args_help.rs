pub fn help_text() -> &'static str {
    "usage: lkjagent [--data DIR] <command> [args]\n\
\n\
commands:\n\
  run                         start the daemon in the foreground\n\
  send <text>                 enqueue an owner message\n\
  status                      print daemon and task status\n\
  log [--limit N] [--follow]  print transcript events\n\
  console                     open the owner console\n\
  memory <query>              search distilled memory\n\
  graph                       print graph state\n\
  model-log [command]         inspect model exchange logs\n\
  personal <command>          inspect personal records\n\
\n\
global options:\n\
  --data DIR                  runtime data directory, accepted before or after command\n\
  -h, --help                  print this help\n\
\n\
Use -- after a command when the command argument must start with --."
}

pub fn is_help_arg(arg: &str) -> bool {
    matches!(arg, "--help" | "-h")
}

pub fn is_help_invocation(args: &[String]) -> bool {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--" {
            return false;
        }
        if is_help_arg(arg) || arg == "help" {
            return true;
        }
        if arg == "--data" {
            iter.next();
        }
    }
    false
}
