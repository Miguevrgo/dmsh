use core::ffi::c_int;              /// TODO:Choose between this line 
use libc::{isatty, STDOUT_FILENO}; ///      and this one (see fn is_terminal)
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use term_size::dimensions;
use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - concatenate files and print on the standard output",
        "qcat".green(),
    );
    eprintln!("Usage: qcat [OPTION]... [FILE]...");
}

fn print_boxed_content(file_name: &str, lines: &[String]) {
    let term_width: usize = dimensions().map(|(w, _)| w).unwrap_or(80);

    println!(
        "\u{250F}{:\u{2501}<width$}\u{2513}",
        "",
        width = term_width - 15,
    );
    println!(
        "\u{2503} {: ^width$} \u{2503}",
        file_name.green(),
        width = term_width - 17,
    );
    println!(
        "\u{2517}{:\u{2501}<width$}\u{251B}",
        "",
        width = term_width - 15
    );

    for (index, line) in lines.iter().enumerate() {
        println!(
            "\u{2503} {} \u{2503} {}",
            format!("{: >4}", index + 1),
            line,
        );
    }

    println!(
        "\u{2517}{:\u{2501}<width$}\u{2501}",
        "",
        width = term_width - 10,
    );
}

fn print_plain_content(lines: &[String]) {
    println!("{}", lines.join("\n"));
}

fn is_terminal() -> bool {
    unsafe { isatty(STDOUT_FILENO) != 0 }
}

/// fn is_terminal() -> bool {
///     extern "C" {
///         pub fn isatty(fd: c_int) -> c_int;
///     }
///     unsafe { isatty(STDOUT_FILENO) != 0 }
/// }

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected > 1, got {}.",
            "Error:".red().bold(),
            args.len(),
        );
        std::process::exit(1);
    }

    for file_name in &args[0..] {
        match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let lines: Vec<String> = reader
                    .lines()
                    .map(|l| l.unwrap_or_else(|_| String::new()))
                    .collect();
                if is_terminal() {
                    print_boxed_content(&file_name, &lines);
                } else {
                    print_plain_content(&lines);
                }
            }
            Err(e) => {
                eprintln!("Error opening {}: {}", file_name, e);
                std::process::exit(1);
            }
        }
    }
}
