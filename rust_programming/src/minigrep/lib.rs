pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    let _results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // This is the idiomatic way of indicating that we are calling the run() function for its side effects only. i.e It doesn't return a value we need.
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // &'static str is a static string literal that has a 'static lifetime.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // the first value in the return value of `env::args` is the name of the
                     // program, so we call `next()` to ignore it.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}

// 'a is an explicit lifetime
// lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. i.e the returned vector should contain string slices that refernce slices of the argument 'contents' rather than the argument 'query'
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insesitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn one_result() {
        let query = "safe, fast, productive.";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            crate::lib::search(query, contents)
        );
    }
}
