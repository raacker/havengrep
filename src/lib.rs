use colored::Colorize;
use std::error::Error;
use std::fs;

pub fn grep(input: Input) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&input.file_path)?;

    let results = if input.ignore_case {
        search_case_insensitive(&input.query, &file_contents)
    } else {
        search(&input.query, &file_contents)
    };
    
    if results.is_empty() {
        return Err("No matches found".into())
    }

    for line in results.iter() {
        println!("{}", line.green());
    }

    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Input {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = true;

        Ok(Input {
            query,
            file_path,
            ignore_case
        })
    }
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
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
    fn no_matches() {
        let query = "rust-";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert!(
            search(query, contents).is_empty()
        );

        assert!(
            search_case_insensitive(query, contents).is_empty()
        );
    }
}