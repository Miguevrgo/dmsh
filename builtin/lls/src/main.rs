use std::{
    env,
    fs::{self, read_dir},
    io,
    path::PathBuf,
};
use text_colorizer::*;

#[derive(Default)]
struct Config {
    show_all: bool,
    long_format: bool,
    sort_by_time: bool,
    show_size: bool,
}

impl Config {
    fn new() -> Self {
        Config::default()
    }

    fn parse_args(&mut self) {
        let args: Vec<String> = env::args().skip(1).collect();
        for arg in &args {
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'a' => self.show_all = true,
                        'l' => self.long_format = true,
                        't' => self.sort_by_time = true,
                        's' => self.show_size = true,
                        _ => eprintln!("{}: unknown option", ch.to_string().red().bold()),
                    }
                }
            }
        }
    }
}

fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}
/// Modificar esto y el otro para ver si write! es mas eficiencite 
/// https://rust-cli.github.io/book/tutorial/output.html
fn print_long_format(metadata: &fs::Metadata, file_name: &std::ffi::OsString) {
    print!("hofd");
    todo!();
}

fn print_short_format(metadata: &fs::Metadata, file_name: &std::ffi::OsString, show_size: bool) {
    todo!();
}

fn list_files(path: &PathBuf, config: &Config) -> io::Result<()> {
    let mut entries: Vec<std::fs::DirEntry> = read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| config.show_all || !is_hidden(e))
        .collect();

    if config.sort_by_time {
        entries.sort_by_key(|e| {
            let metadata = e.metadata().unwrap();
            std::cmp::Reverse(metadata.modified().unwrap())
        });
    }

    for entry in entries {
        let metadata = entry.metadata()?;
        let file_name = entry.file_name();

        if config.long_format {
            print_long_format(&metadata, &file_name);
        } else {
            print_short_format(&metadata, &file_name, config.show_size);
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut config = Config::new();
    config.parse_args();
    let path = env::current_dir()?;
    if let Err(err) = list_files(&path, &config) {
        eprintln!("{}: {}", "Error".red().bold(), err);
    }
    Ok(())
}