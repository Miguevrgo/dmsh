use chrono::{DateTime, Datelike, Local, Timelike};
use libc::{c_char, getgrgid_r, getpwuid_r, group, passwd};
use std::{
    collections::HashMap,
    env,
    fs::{self, read_dir},
    io,
    os::unix::fs::MetadataExt,
    path::PathBuf,
};
use text_colorizer::*;

#[derive(Default)]
struct Config {
    directory: Option<PathBuf>,
    show_all: bool,
    long_format: bool,
    sort_by_time: bool,
    human_readable: bool,
    show_group: bool,
    group_directory: bool,
}

impl Config {
    fn new() -> Self {
        Config::default()
    }

    fn parse_args(&mut self) {
        for arg in env::args().skip(1) {
            match arg {
                arg if arg.starts_with("--") => match arg.as_str() {
                    "--long" => self.long_format = true,
                    "--all" => self.show_all = true,
                    "--human-readable" => self.human_readable = true,
                    "--group-directories-first" => self.group_directory = true,
                    _ => eprintln!("{}: unknown option", arg.to_string().red().bold()),
                },
                arg if arg.starts_with('-') => {
                    for ch in arg.chars().skip(1) {
                        match ch {
                            'a' => self.show_all = true,
                            'l' => self.long_format = true,
                            't' => self.sort_by_time = true,
                            'h' => self.human_readable = true,
                            'g' => self.show_group = true,
                            _ => eprintln!("{}: unknown option", ch.to_string().red().bold()),
                        }
                    }
                }
                _ => {
                    self.directory = Some(PathBuf::from(arg));
                }
            }
        }
    }
}

fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

fn ugo_mode(mode: u32) -> String {
    let permissions = [
        (mode & 0o400 != 0, 'r'),
        (mode & 0o200 != 0, 'w'),
        (mode & 0o100 != 0, 'x'),
        (mode & 0o040 != 0, 'r'),
        (mode & 0o020 != 0, 'w'),
        (mode & 0o010 != 0, 'x'),
        (mode & 0o004 != 0, 'r'),
        (mode & 0o002 != 0, 'w'),
        (mode & 0o001 != 0, 'x'),
    ];

    permissions
        .iter()
        .map(|&(bit, ch)| {
            if bit {
                match ch {
                    'r' => ch.to_string().red().to_string(),
                    'w' => ch.to_string().yellow().to_string(),
                    'x' => ch.to_string().green().to_string(),
                    _ => ch.to_string(),
                }
            } else {
                "-".to_string()
            }
        })
        .collect()
}

fn human_format(size: f64) -> String {
    let units = ['B', 'K', 'M', 'G', 'T', 'P', 'E', 'Z'];
    let mut size = size;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        // Check bounds?
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1}{} ", size, units[unit_index])
}

fn get_user_name(uid: u32) -> String {
    let mut buf: [u8; 1024] = [0; 1024];
    let mut result = String::new();

    unsafe {
        let mut ptr: *mut passwd = std::ptr::null_mut();
        let ret = getpwuid_r(
            uid as libc::uid_t,
            &mut std::mem::zeroed(),
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            &mut ptr,
        );
        if ret == 0 && !ptr.is_null() {
            let cstr = std::ffi::CStr::from_ptr((*ptr).pw_name);
            result = cstr.to_string_lossy().to_string();
        }
    }

    result
}

fn get_group_name(gid: u32) -> String {
    let mut buf: [u8; 1024] = [0; 1024];
    let mut result = String::new();

    unsafe {
        let mut ptr: *mut group = std::ptr::null_mut();
        let ret = getgrgid_r(
            gid as libc::gid_t,
            &mut std::mem::zeroed(),
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            &mut ptr,
        );
        if ret == 0 && !ptr.is_null() {
            let cstr = std::ffi::CStr::from_ptr((*ptr).gr_name);
            result = cstr.to_string_lossy().to_string();
        }
    }

    result
}

fn long_format(
    metadata: &fs::Metadata,
    file_name: &std::ffi::OsString,
    user_cache: &mut HashMap<u32, String>,
    group_cache: &mut HashMap<u32, String>,
    config: &Config,
) -> String {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let datetime = DateTime::<Local>::from(metadata.modified().unwrap());
    let formatted_date = format!(
        "{:02} {:02} {:02}:{:02}",
        datetime.day(),
        months[(datetime.month() - 1) as usize],
        datetime.hour(),
        datetime.minute()
    );
    let mode = ugo_mode(metadata.mode());
    let (file_name, dir_char) = if metadata.is_dir() {
        (
            file_name.to_string_lossy().blue().bold(),
            "d".blue().to_string(),
        )
    } else {
        (
            file_name.to_string_lossy().white().bold(),
            "-".white().to_string(),
        )
    };

    let u_id = metadata.uid();
    let g_id = metadata.gid();

    let user_name = user_cache
        .entry(u_id)
        .or_insert_with(|| get_user_name(u_id));

    let group_name = if config.show_group {
        format!(
            "{} ",
            group_cache
                .entry(g_id)
                .or_insert_with(|| get_group_name(g_id))
        )
    } else {
        String::new()
    };

    let size = if config.human_readable {
        human_format(metadata.len() as f64)
    } else {
        metadata.len().to_string()
    };

    format!(
        "{}{} {} {}{:>8} {} {}",
        dir_char, mode, user_name, group_name, size, formatted_date, file_name,
    )
}

fn short_format(metadata: &fs::Metadata, file_name: &std::ffi::OsString) -> String {
    let file_name = if metadata.is_dir() {
        file_name.to_string_lossy().blue().bold().to_string()
    } else {
        file_name.to_string_lossy().white().bold().to_string()
    };

    file_name
}

fn list_files(path: &PathBuf, config: &Config) -> io::Result<Vec<String>> {
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

    if config.group_directory {
        entries.sort_by_key(|e| {
            let metadata = e.metadata().unwrap();
            std::cmp::Reverse(metadata.is_dir())
        });
    }

    let mut user_cache = HashMap::new();
    let mut group_cache = HashMap::new();

    entries
        .iter()
        .map(|entry| {
            let metadata = entry.metadata()?;
            let file_name = entry.file_name();

            if config.long_format {
                Ok(long_format(
                    &metadata,
                    &file_name,
                    &mut user_cache,
                    &mut group_cache,
                    config,
                ))
            } else {
                Ok(short_format(&metadata, &file_name))
            }
        })
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    let mut config = Config::new();
    config.parse_args();
    let path = config.directory.clone().unwrap_or(env::current_dir()?);
    match list_files(&path, &config) {
        Ok(results) => {
            for result in results {
                println!("{}", result);
            }
        }
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
        }
    }
    Ok(())
}
