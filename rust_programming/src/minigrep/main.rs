mod lib;

// cargo run -p rust_programming --bin minigrep hey rust_programming/poem.txt

fn main() {
    let _args: Vec<String> = std::env::args().collect();

    let config = lib::Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = lib::run(config) {
        println!("Applicaton error: {e}");
        std::process::exit(1);
    }
}
