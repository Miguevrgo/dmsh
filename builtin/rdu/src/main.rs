use std::{
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

fn file_size(path: &Path) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn calculate_disk_usage(path: &Path, total_size: Arc<Mutex<u64>>) -> io::Result<()> {
    let mut handles = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let size = file_size(&path)?;
            let mut total = total_size.lock().unwrap();
            *total += size;
        } else if path.is_dir() {
            let total_size_clone = Arc::clone(&total_size);
            let path_clone = path.clone();
            let handle = thread::spawn(move || {
                if let Err(err) = calculate_disk_usage(&path_clone, total_size_clone) {
                    eprintln!(
                        "Error calculating size of directory {:?}: {}",
                        path_clone, err
                    );
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().expect("The thread failed");
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let dir = Path::new(&path);

    if !dir.exists() {
        eprintln!("Error: Directory {} doesn't exist", dir.display());
        std::process::exit(1);
    }

    let total_size = Arc::new(Mutex::new(0_u64));

    if let Err(err) = calculate_disk_usage(dir, Arc::clone(&total_size)) {
        eprintln!("Error calculating size: {err}");
    }

    let total_size = *total_size.lock().unwrap() as f64;

    let mut total_size = total_size;
    const UNITS: [char; 8] = ['B', 'K', 'M', 'G', 'T', 'P', 'E', 'Z'];
    let mut unit = 0;
    while total_size > 1024.0 {
        total_size /= 1024.0;
        unit += 1;
    }

    println!("Size: {:.2}{}", total_size, UNITS[unit]);

    Ok(())
}
