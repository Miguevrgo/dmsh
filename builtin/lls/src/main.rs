use std::{env, fs::read_dir, io, path::PathBuf};
use termcolor::*;

fn list_files(path: &PathBuf) -> io::Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        println!("{:?} ", entry.file_name());
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let path = env::current_dir()?;
    if let Err(err) = list_files(&path) {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        eprintln!("Error: {}", err.to_string());
    }
    Ok(())
}