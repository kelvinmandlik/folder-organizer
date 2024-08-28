use std::collections::HashMap;
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;

use crate::configs::{
    FOLDERS, FOLDER_DIR_NAME, OTHER_DIR_NAME, PROGRESS_BAR_LENGTH,
};
use crate::utils::{
    get_current_dir, get_current_file_name, get_file_size, move_file,
    size_converter,
};

pub fn organize() {
    let ext_dirs: &[&str] =
        &FOLDERS.iter().map(|&(a, _)| a).collect::<Vec<&str>>();
    let dirs: &[&str] = &ext_dirs
        .iter()
        .copied()
        .chain([FOLDER_DIR_NAME, OTHER_DIR_NAME])
        .collect::<Vec<&str>>();

    let current_dir = get_current_dir();
    let current_file_name = get_current_file_name();

    println!("\n* Scanning the folder - {}\n", current_dir);

    let files = fs::read_dir(&current_dir).unwrap();
    let total_files = fs::read_dir(&current_dir).unwrap().count();

    let mut item_moved: HashMap<&str, (u32, u64)> = HashMap::new();

    print!(
        "* Cleaning in process [{:-<pbl$}] 0%",
        "",
        pbl = PROGRESS_BAR_LENGTH
    );
    stdout().flush().unwrap();

    for (count, entry) in files.into_iter().enumerate() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();

        if file_name == current_file_name {
            continue;
        }

        let mut folder_to_move: Option<&str> = None;

        if entry_path.is_dir() {
            if !dirs.contains(&file_name.as_str()) {
                folder_to_move = Some(&FOLDER_DIR_NAME);
            }
        } else {
            folder_to_move = Some(&OTHER_DIR_NAME);
            if let Some(extension) = entry_path.extension() {
                let extension = extension.to_str().unwrap().to_lowercase();
                for &(folder_name, extensions) in FOLDERS {
                    if extensions.contains(&extension.as_str()) {
                        folder_to_move = Some(&folder_name);
                        break;
                    }
                }
            }
        }

        if let Some(folder_to_move) = folder_to_move {
            let dest_dir = Path::new(&current_dir).join(folder_to_move);
            let file_size = get_file_size(&entry_path).unwrap();
            move_file(&entry_path, &dest_dir);
            let item_moved_value =
                item_moved.entry(folder_to_move).or_insert((0, 0));
            item_moved_value.0 += 1;
            item_moved_value.1 += file_size;
        }

        let progress_bar_complete = (((count + 1) * PROGRESS_BAR_LENGTH
            / total_files) as f32)
            .floor() as usize;
        print!(
            "\r* Cleaning in process [{:#<pbc$}{:-<pbl$}] {}%",
            "",
            "",
            (((count + 1) * 100 / total_files) as f32).floor(),
            pbc = progress_bar_complete,
            pbl = PROGRESS_BAR_LENGTH - progress_bar_complete
        );
        stdout().flush().unwrap();
    }

    println!("\n");

    let mut item_moved_count: u32 = 0;
    let mut data_moved_size: u64 = 0;

    for (folder_name, (files_count, data_size)) in &item_moved {
        println!(
            "* {:<15} : {:>5} [{:>10} | {:>15} bytes]",
            folder_name,
            files_count,
            size_converter(data_size),
            data_size
        );
        item_moved_count += files_count;
        data_moved_size += data_size;
    }

    if item_moved_count == 0 {
        println!("* Evenrything is Cleaned");
    } else {
        println!(
            "\n* {:<15} : {:>5} [{:>10} | {:>15} bytes]",
            "TOTAL",
            item_moved_count,
            size_converter(&data_moved_size),
            &data_moved_size
        );
    }

    println!("\n* Cleaning complete\n\n")
}
