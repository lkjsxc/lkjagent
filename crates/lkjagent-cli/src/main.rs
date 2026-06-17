fn main() {
    let outcome = lkjagent_cli::run_cli(std::env::args().skip(1));
    if !outcome.stdout.is_empty() {
        println!("{}", outcome.stdout);
    }
    if !outcome.stderr.is_empty() {
        eprintln!("{}", outcome.stderr);
    }
    std::process::exit(outcome.code);
}
