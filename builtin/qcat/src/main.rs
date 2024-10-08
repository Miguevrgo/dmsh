use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, IsTerminal, Write};
use std::{env, io};
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
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    writeln!(
        handle,
        "\u{250F}{:\u{2501}<width$}\u{2513}",
        "",
        width = term_width - 15,
    )
    .unwrap();
    writeln!(
        handle,
        "\u{2503} {: ^width$} \u{2503}",
        file_name.green(),
        width = term_width - 17,
    )
    .unwrap();
    writeln!(
        handle,
        "\u{2517}{:\u{2501}<width$}\u{251B}",
        "",
        width = term_width - 15
    )
    .unwrap();

    let lines_joined = lines
        .iter()
        .enumerate()
        .map(|(index, line)| format!("\u{2503} {: >4} \u{2503} {}", index + 1, line))
        .collect::<Vec<String>>()
        .join("\n");

    writeln!(handle, "{}", lines_joined).unwrap();

    writeln!(
        handle,
        "\u{2517}{:\u{2501}<width$}\u{2501}",
        "",
        width = term_width - 10,
    )
    .unwrap();
}

fn print_plain_content(lines: &[String]) {
    println!("{}", lines.join("\n"));
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
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
                if std::io::stdout().is_terminal() {
                    print_boxed_content(file_name, &lines);
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
