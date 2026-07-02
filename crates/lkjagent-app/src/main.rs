fn main() {
    let code = match lkjagent_app::cli::run(std::env::args().skip(1)) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    };
    std::process::exit(code);
}
