use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn write_to_file(output_file: &str, name: &str, path: &str) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
        .unwrap();

    writeln!(file, "{}\t{}", name, path).unwrap();
}

fn list_files(
    path: &PathBuf,
    output_file: &str,
    write_files: bool,
    write_folders: bool,
    list_all: bool,
) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_file() && write_files {
                write_to_file(
                    output_file,
                    &entry.file_name().to_string_lossy(),
                    &entry.path().display().to_string(),
                );
            } else if entry.path().is_dir() {
                if write_folders {
                    write_to_file(
                        output_file,
                        &entry.file_name().to_string_lossy(),
                        &entry.path().display().to_string(),
                    );
                }
                if list_all {
                    list_files(&entry.path(), output_file, write_files, write_folders, true);
                }
            }
        }
    }
}

fn main() {
    println!("Choose an option:");
    println!("1. List all folders in the current directory");
    println!("2. List all folders in all underlying directories");
    println!("3. List all files in the current directory");
    println!("4. List all files in all underlying directories");
    println!("5. List both folders and files in the current directory");
    println!("6. List folders and files in all underlying directories");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<u32>().unwrap();

    let write_files = match choice {
        3 | 4 | 5 | 6 => true,
        _ => false,
    };
    let write_folders = match choice {
        1 | 2 | 5 | 6 => true,
        _ => false,
    };
    let recursive = match choice {
        2 | 4 | 6 => true,
        _ => false,
    };

    if write_files | write_folders {
        list_files(
            &PathBuf::from("."),
            "results.txt",
            write_files,
            write_folders,
            recursive,
        );
    }

    if write_files && write_folders {
        println!("Folders and files written to results.txt");
    } else if write_files {
        println!("Files written to results.txt");
    } else if write_folders {
        println!("Folders written to results.txt");
    }
}
