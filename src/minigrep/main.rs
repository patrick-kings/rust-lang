mod lib;

//

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = lib::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = lib::run(config) {
        println!("Applicaton error: {e}");
        std::process::exit(1);
    }
}
