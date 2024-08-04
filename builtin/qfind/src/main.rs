use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use text_colorizer::Colorize;

const EXCLUDED_DIRS: &[&str] = &["/proc", "/sys", "/dev", "/run", "/tmp", "/var/run"];

fn visit_dirs(dir: &Path, file_name: &str, tx: mpsc::Sender<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let tx = tx.clone();

            if EXCLUDED_DIRS.iter().any(|&excluded| path.starts_with(excluded)) {
                continue;
            }

            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.permissions().readonly() {
                    continue;
                }
            } else {
                continue;
            }

            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_string_lossy();
                if !dir_name.starts_with('.') {
                    let file_name = file_name.to_string();
                    thread::spawn(move || {
                        visit_dirs(&path, &file_name, tx).unwrap();
                    });
                }
            } else if path.file_name().unwrap().to_string_lossy().contains(file_name) {
                tx.send(path.display().to_string()).unwrap();
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{} usage: {} <file_name> [start_dir]", "Error:".red().bold(), args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    let start_dir = if args.len() > 2 { &args[2] } else { "." };

    let path = Path::new(start_dir);
    let (tx, rx) = mpsc::channel();

    visit_dirs(&path, file_name, tx)?;

    for found_path in rx {
        let found_path_buf = PathBuf::from(&found_path);
        let parent_dir = found_path_buf.parent().unwrap().display().to_string();
        let file_name = found_path_buf.file_name().unwrap().to_string_lossy();

        println!("{}{}", parent_dir.blue().bold(), format!("/{}", file_name).green().bold());
    }

    Ok(())
}