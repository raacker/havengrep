use colored::Colorize;
use std::error::Error;
use std::fs;

pub fn grep(input: Input) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", input.query.green());
    println!("From file {}", input.file_path.green());

    let file_contents = fs::read_to_string(&input.file_path)?;

    println!("File contents\n{file_contents}");

    Ok(())
}

pub struct Input {
    query: String,
    file_path: String,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Input { query, file_path })
    }
}
