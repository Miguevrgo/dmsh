use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use text_colorizer::*;

fn print_usage() {
    eprintln!("{} - concatenate files and print on the standard output",
              "qcat".green());
    eprintln!("Usage: qcat [OPTION]... [FILE]...");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected > 1, got {}.",
                  "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    for file_name in &args[1..] {
        match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    match line {
                        Ok(line_content) => println!("{}", line_content),
                        Err(e) => eprintln!("Error reading {}: {}", file_name, e)
                    }
                }
            }
            Err(e) => {
                eprintln!("{} opening {}: {}", "Error".red().bold(), file_name, e);
            }
        }
    }



}