use std::env;
use std::fs;
use std::io::{stdin, Read, Result};
use std::path::{Path, PathBuf};

pub fn get_current_dir() -> String {
    let current_dir = env::current_dir().unwrap();
    return current_dir.to_str().unwrap().to_string();
}

pub fn get_current_file_name() -> String {
    let current_file_name = env::current_exe().unwrap();
    return current_file_name
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn get_file_size(file_path: &PathBuf) -> Result<u64> {
    let metadata = file_path.metadata()?;
    if metadata.is_file() {
        return Ok(metadata.len());
    }

    let mut size = 0;
    for entry in fs::read_dir(file_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        size += metadata.len();
        if metadata.is_dir() {
            size += get_file_size(&entry.path())?
        }
    }
    Ok(size)
}

pub fn move_file(file_path: &PathBuf, dest_dir: &PathBuf) {
    if !dest_dir.exists() {
        fs::create_dir(dest_dir).unwrap();
    }
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let dest_path = dest_dir.clone().join(file_name);
    fs::rename(file_path, Path::new(&dest_path)).unwrap();
}

pub fn size_converter(size: &u64) -> String {
    let step_unit = 1024_f64;
    let mut num = size.clone() as f64;
    let units = ["bytes", "KB", "MB", "GB", "TB", "PB"];

    for unit in &units {
        if num < step_unit {
            return format!("{:.3} {}", num, unit);
        }
        num /= step_unit;
    }
    format!("{:.3} {}", num, "PB")
}

pub fn wait_for_enter() {
    let mut buffer = [0];
    println!("Press enter to close...");
    let _ = stdin().read_exact(&mut buffer);
}
