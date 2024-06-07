use std::env;
use std::fs;
use colored::Colorize;

fn main() {
    // Force type of the vector to avoid wrong inference
    let args: Vec<String> = env::args().collect();

    let input = Input::new(&args);

    println!("Searching for {}", input.query.green());
    println!("From file {}", input.file_path.green());

    let file_contents = fs::read_to_string(input.file_path).expect("Should be able to read the file.");

    println!("File contents\n{file_contents}");
}

struct Input {
    query: String,
    file_path: String,
}

impl Input {
    fn new(args: &[String]) -> Input {
        if args.len() < 3 {
            panic!("{}", "Insufficient arguments".red());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Input { query, file_path }
    }
}
