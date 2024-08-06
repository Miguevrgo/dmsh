use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use text_colorizer::Colorize;

const EXCLUDED_DIRS: &[&str] = &["/proc", "/sys", "/dev", "/run", "/tmp", "/var/run"];

#[derive(Clone)]
struct Options {
    hidden_folders: bool,
    max_depth: Option<usize>,
}

impl Options {
    fn new(hidden_folders: bool, max_depth: Option<usize>) -> Options {
        Options {
            hidden_folders,
            max_depth,
        }
    }
}

fn visit_dirs(
    dir: &Path,
    file_name: &str,
    tx: mpsc::Sender<String>,
    options: Options,
    depth: usize,
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let tx = tx.clone();

            if EXCLUDED_DIRS
                .iter()
                .any(|&excluded| path.starts_with(excluded))
            {
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
                if options.hidden_folders
                    || !dir_name.starts_with('.')
                        && options.max_depth.map_or(true, |max| depth < max)
                {
                    let file_name = file_name.to_string();
                    let options = options.clone();
                    thread::spawn(move || {
                        visit_dirs(&path, &file_name, tx, options, depth + 1).unwrap();
                    });
                }
            } else if path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains(file_name)
            {
                tx.send(path.display().to_string()).unwrap();
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "{} usage: {} <file_name> [start_dir] [-H] [-max-depth X]",
            "Error:".red().bold(),
            args[0]
        );
        std::process::exit(1);
    }

    let mut hidden_folders = false;
    let mut max_depth = None;
    let mut file_name = "";
    let mut start_dir = ".";

    for i in 1..args.len() {
        match args[i].as_str() {
            "-H" => hidden_folders = true,
            "-max-depth" => {
                if i + 1 < args.len() {
                    max_depth = Some(args[i + 1].parse().expect("Invalid max depth"));
                } else {
                    eprintln!(
                        "{} usage: {} <file_name> [start_dir] [-H] [-max-depth X]",
                        "Error:".red().bold(),
                        args[0]
                    );
                    std::process::exit(1);
                }
            }
            _ => {
                if file_name.is_empty() {
                    file_name = &args[i];
                } else {
                    start_dir = &args[i];
                }
            }
        }
    }

    let options = Options::new(hidden_folders, max_depth);
    let path = Path::new(start_dir);
    let (tx, rx) = mpsc::channel();

    visit_dirs(path, file_name, tx, options, 0)?;

    for found_path in rx {
        let found_path_buf = PathBuf::from(&found_path);
        let parent_dir = found_path_buf.parent().unwrap().display().to_string();
        let file_name = found_path_buf.file_name().unwrap().to_string_lossy();

        println!(
            "{}{}",
            parent_dir.blue().bold(),
            format!("/{}", file_name).green().bold()
        );
    }

    Ok(())
}
