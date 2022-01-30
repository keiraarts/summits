use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(query);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let search = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(query);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "dawn";
        let contents: &str = "Rust awakes you at dawn";
        assert_eq!(vec!["dawn"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "dawn";
        let contents = "triple hero Dawn";
        assert_ne!(vec!["dawn"], search(query, contents))
    }

    fn case_insensitive() {
        let query = "ruSt";
        let contents = "nonense poetry rust";

        assert_eq!(
            vec!["Rust", "Trust me"],
            search_case_insensitive(query, contents)
        )
    }
}
