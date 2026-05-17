use chrono::{DateTime, Datelike, Local, Timelike};
use libc::{c_char, getgrgid_r, getpwuid_r, group, passwd};
use std::{
    collections::HashMap,
    env,
    fs::{self, read_dir},
    io::{self, BufWriter, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::SystemTime,
};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const WHITE: &str = "\x1b[37m";
const GREY: &str = "\x1b[90m";

#[derive(Default)]
struct Config {
    directory: Option<PathBuf>,
    show_all: bool,
    long_format: bool,
    sort_by_time: bool,
    human_readable: bool,
    show_group: bool,
    group_directory: bool,
    show_icons: bool,
}

impl Config {
    fn parse_args(&mut self) {
        for arg in env::args().skip(1) {
            match arg {
                arg if arg.starts_with("--") => match arg.as_str() {
                    "--long" => self.long_format = true,
                    "--all" => self.show_all = true,
                    "--human-readable" => self.human_readable = true,
                    "--group-directories-first" => self.group_directory = true,
                    "--icons" => self.show_icons = true,
                    _ => eprintln!("{RED}{BOLD}{arg}{RESET}: unknown option"),
                },
                arg if arg.starts_with('-') => {
                    for ch in arg.chars().skip(1) {
                        match ch {
                            'a' => self.show_all = true,
                            'l' => self.long_format = true,
                            't' => self.sort_by_time = true,
                            'h' => self.human_readable = true,
                            'g' => self.show_group = true,
                            _ => eprintln!("{RED}{BOLD}{ch}{RESET}: unknown option"),
                        }
                    }
                }
                _ => self.directory = Some(PathBuf::from(arg)),
            }
        }
    }
}

fn ugo_mode(mode: u32) -> String {
    let bits = [
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

    bits.iter()
        .map(|&(set, ch)| {
            if set {
                let color = match ch {
                    'r' => RED,
                    'w' => YELLOW,
                    _ => GREEN,
                };
                format!("{color}{ch}{RESET}")
            } else {
                format!("{GREY}-{RESET}")
            }
        })
        .collect()
}

fn human_format(size: u64) -> String {
    const UNITS: [char; 8] = ['B', 'K', 'M', 'G', 'T', 'P', 'E', 'Z'];
    let mut idx = 0;
    let mut s = size * 10;
    // Whole internet is around 64 Zettabytes so no bound checking
    while s >= 10240 {
        s >>= 10;
        idx += 1;
    }
    format!("{:.1}{} ", s as f64 / 10.0, UNITS[idx])
}

fn get_user_name(uid: u32) -> String {
    let mut buf = [0u8; 512];
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
            result = std::ffi::CStr::from_ptr((*ptr).pw_name)
                .to_string_lossy()
                .into_owned();
        }
    }
    result
}

fn get_group_name(gid: u32) -> String {
    let mut buf = [0u8; 512];
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
            result = std::ffi::CStr::from_ptr((*ptr).gr_name)
                .to_string_lossy()
                .into_owned();
        }
    }
    result
}

fn format_date(metadata: &fs::Metadata) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let dt = DateTime::<Local>::from(metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH));
    format!(
        "{:02} {} {:02}:{:02}",
        dt.day(),
        MONTHS[(dt.month() - 1) as usize],
        dt.hour(),
        dt.minute()
    )
}

fn fmt_long(
    metadata: &fs::Metadata,
    file_name: &std::ffi::OsString,
    user_cache: &mut HashMap<u32, String>,
    group_cache: &mut HashMap<u32, String>,
    config: &Config,
) -> String {
    let is_dir = metadata.is_dir();
    let (name_colored, dir_char) = if is_dir {
        (
            format!("{BLUE}{BOLD}{}{RESET}", file_name.to_string_lossy()),
            format!("{BLUE}d{RESET}"),
        )
    } else {
        (
            format!("{WHITE}{BOLD}{}{RESET}", file_name.to_string_lossy()),
            format!("{WHITE}-{RESET}"),
        )
    };

    let name_colored = if config.show_icons && is_dir {
        format!(
            "{BLUE}{BOLD}\u{f07b} {}{RESET}",
            file_name.to_string_lossy()
        )
    } else {
        name_colored
    };

    let uid = metadata.uid();
    let gid = metadata.gid();
    let user = user_cache.entry(uid).or_insert_with(|| get_user_name(uid));
    let group = if config.show_group {
        format!(
            "{} ",
            group_cache
                .entry(gid)
                .or_insert_with(|| get_group_name(gid))
        )
    } else {
        String::new()
    };

    let size = if config.human_readable {
        human_format(metadata.len())
    } else {
        metadata.len().to_string()
    };

    format!(
        "{dir_char}{} {user} {group}{size:>8} {} {name_colored}",
        ugo_mode(metadata.mode()),
        format_date(metadata),
    )
}

fn write_short(
    out: &mut impl Write,
    metadata: &fs::Metadata,
    file_name: &std::ffi::OsString,
) -> io::Result<()> {
    if metadata.is_dir() {
        writeln!(out, "{BLUE}{BOLD}{}{RESET}", file_name.to_string_lossy())
    } else {
        writeln!(out, "{WHITE}{BOLD}{}{RESET}", file_name.to_string_lossy())
    }
}

struct Entry {
    dir_entry: std::fs::DirEntry,
    metadata: fs::Metadata,
}

fn main() -> io::Result<()> {
    let mut config = Config::default();
    config.parse_args();
    let path = config.directory.clone().unwrap_or(env::current_dir()?);

    let mut entries: Vec<Entry> = read_dir(&path)?
        .filter_map(Result::ok)
        .filter(|e| config.show_all || !e.file_name().to_string_lossy().starts_with('.'))
        .filter_map(|e| {
            e.metadata().ok().map(|m| Entry {
                dir_entry: e,
                metadata: m,
            })
        })
        .fold(Vec::with_capacity(8), |mut v, e| {
            v.push(e);
            v
        });

    entries.sort_unstable_by(|a, b| {
        if config.group_directory {
            let dir_ord = b.metadata.is_dir().cmp(&a.metadata.is_dir());
            if dir_ord != std::cmp::Ordering::Equal {
                return dir_ord;
            }
        }
        if config.sort_by_time {
            let ta = a.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let tb = b.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            return tb.cmp(&ta);
        }
        std::cmp::Ordering::Equal
    });

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut user_cache: HashMap<u32, String> = HashMap::new();
    let mut group_cache: HashMap<u32, String> = HashMap::new();

    for e in &entries {
        let name = e.dir_entry.file_name();
        let res = if config.long_format {
            let line = fmt_long(
                &e.metadata,
                &name,
                &mut user_cache,
                &mut group_cache,
                &config,
            );
            writeln!(out, "{line}")
        } else {
            write_short(&mut out, &e.metadata, &name)
        };
        if res.is_err() {
            break;
        }
    }

    let _ = out.flush();
    Ok(())
}
