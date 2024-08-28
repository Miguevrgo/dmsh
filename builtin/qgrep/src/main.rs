use regex::Regex;
use text_colorizer::*;

fn search_in_path(regex: &Regex, path: &str) -> Result<(), std::io::Error> {
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "{}\n\t {} {} <regex> <path>",
            "--|Error|--".red().bold(),
            "Usage:".bold().blue(),
            args[0],
        );
        std::process::exit(1);
    }

    let pattern = &args[1];
    let path = &args[2];

    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("{}: {}", "--|Error|--".red().bold(), err);
            std::process::exit(1);
        }
    };

    if let Err(err) = search_in_path(&regex, path) {
        eprintln!("{} searching in the path: {}", "Error".red().bold(), err)
    }

    Ok(())
}
