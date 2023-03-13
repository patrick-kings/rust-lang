pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    println!("With text: \n {contents}");

    // This is the idiomatic way of indicating that we are calling the run() function for its side effects only. i.e It doesn't return a value we need.
    Ok(())
}

pub struct Config {
   pub query: String,
   pub file_path: String,
}

impl Config {
    // &'static str is a static string literal that has a 'static lifetime.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}
