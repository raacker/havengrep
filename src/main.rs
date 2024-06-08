use colored::Colorize;
use std::env;
use std::process;

use havengrep::Input;

fn main() {
    // Force type of the vector to avoid wrong inference
    let args: Vec<String> = env::args().collect();

    let input = Input::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err.red());
        process::exit(1);
    });

    if let Err(e) = havengrep::grep(input) {
        println!("Problem reading the file: {}", e.to_string().red());
        process::exit(1);
    }
}
