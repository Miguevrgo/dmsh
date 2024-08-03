use std::env;
use std::fs;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&Path)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            cb(&path);
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let start_dir = if args.len() > 1 { &args[1] } else { "." };

    let path = Path::new(start_dir);

    visit_dirs(&path, &|path| {
        println!("{}", path.display());
    })?;

    Ok(())
}
