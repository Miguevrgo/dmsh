use regex::Regex;
use std::io::BufRead;
use std::sync::{mpsc, Arc};
use std::thread;
use text_colorizer::*;

struct Config {
    pattern: String,
    path: String,
    recursive: bool,
    ignore_case: bool,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        let mut recursive = false;
        let mut ignore_case = false;
        let mut pattern = String::new();
        let mut path = String::new();

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-r" => recursive = true,
                "-i" => ignore_case = true,
                _ if pattern.is_empty() => pattern.clone_from(arg),
                _ => path.clone_from(arg),
            }
        }

        if pattern.is_empty() || path.is_empty() {
            return Err("Pattern or path missing");
        }

        Ok(Config {
            pattern,
            path,
            recursive,
            ignore_case,
        })
    }
}

fn search_in_file(regex: &Regex, path: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut has_match = false;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if regex.is_match(&line) {
            if !has_match {
                println!("\n{}:", path.bold().blue());
                has_match = true;
            }
            println!("{}: {}", line_number + 1, line);
        }
    }

    Ok(())
}

fn search_in_path(regex: &Regex, path: &str, recursive: bool) -> Result<(), std::io::Error> {
    let metadata = std::fs::metadata(path)?;
    if metadata.is_file() {
        search_in_file(regex, path)?;
    } else if metadata.is_dir() {
        let (tx, rx) = mpsc::channel();
        let path = path.to_string();
        let regex = Arc::new(regex.clone());

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            let tx = tx.clone();
            let regex = Arc::clone(&regex);

            thread::spawn(move || {
                if path.is_file() {
                    let _ = search_in_file(&regex, path.to_str().unwrap());
                } else if path.is_dir() && recursive {
                    let _ = search_in_path(&regex, path.to_str().unwrap(), recursive);
                }
                tx.send(()).unwrap();
            });
        }

        for _ in std::fs::read_dir(&path)? {
            rx.recv().unwrap();
        }
    } else {
        eprintln!("{}: Not a valid directory nor file", "Error".red().bold());
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("{}: {}", "--|Error|--".red().bold(), err);
        eprintln!(
            "{}\n\t {} <pattern> <path> [-r] [-i]",
            "Usage:".bold().blue(),
            args[0]
        );
        std::process::exit(1);
    });

    let pattern = if config.ignore_case {
        format!("(?i){}", config.pattern)
    } else {
        config.pattern
    };

    let regex = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("{}: {}", "--|Error|--".red().bold(), err);
            std::process::exit(1);
        }
    };

    if let Err(err) = search_in_path(&regex, &config.path, config.recursive) {
        eprintln!("{} Searching in path: {}", "Error".red().bold(), err);
    }

    Ok(())
}
