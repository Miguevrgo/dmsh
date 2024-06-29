use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - concatenate files and print on the standard output",
        "qcat".green()
    );
    eprintln!("Usage: qcat [OPTION]... [FILE]...");
}

/// TODO: Change this behaviour, uploaded for contributors to know the idea
/// behind, missing adjusting numbers and lines, maybe changing filename to exclude
/// route
fn print_boxed_content(file_name: &str, lines: Vec<String>) {
    let max_line_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let width = max_line_width + 4;
    let max_width = width;

    println!("┏{:━<width$}┓", "", width = width);
    println!("┃ {: ^width$} ┃", file_name.green(), width = width - 2);
    println!("┗{:━<width$}┛", "", width = width);

    for (index, line) in lines.iter().enumerate() {
        println!(
            "┃ {: >4} ┃ {: ^max_width$} ┃",
            index + 1,
            line,
            max_width = max_width
        );
    }

    println!("┗{:━<width$}┛", "", width = width);
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected > 1, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    for file_name in &args[1..] {
        match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let lines: Vec<String> = reader
                    .lines()
                    .map(|l| l.unwrap_or_else(|_| String::new()))
                    .collect();
                print_boxed_content(&file_name, lines);
            }
            Err(e) => {
                eprintln!("Error opening {}: {}", file_name, e);
                std::process::exit(1);
            }
        }
    }
}

