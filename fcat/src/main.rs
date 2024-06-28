use std::env;
use std::io::{self, BufRead, BufReader, Write};
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



}