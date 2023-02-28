use std::fs;
use std::io::{stdin, stdout, Error, Write};
use std::path::PathBuf;

fn write_to_file(output_file: &str, name: &str, path: &str) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)?;

    writeln!(file, "{}\t{}", name, path)?;
    Ok(())
}

fn list_files(
    path: &PathBuf,
    output_file: &str,
    write_files: bool,
    write_folders: bool,
    list_all: bool,
) -> Result<(), Error> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_file() && write_files {
                write_to_file(
                    output_file,
                    &entry.file_name().to_string_lossy(),
                    &entry.path().display().to_string(),
                )?;
            } else if entry.path().is_dir() {
                if write_folders {
                    write_to_file(
                        output_file,
                        &entry.file_name().to_string_lossy(),
                        &entry.path().display().to_string(),
                    )?;
                }
                if list_all {
                    list_files(&entry.path(), output_file, write_files, write_folders, true)?;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    println!("Choose an option:");
    println!("1. List all folders in the current directory");
    println!("2. List all folders in all underlying directories");
    println!("3. List all files in the current directory");
    println!("4. List all files in all underlying directories");
    println!("5. List both folders and files in the current directory");
    println!("6. List folders and files in all underlying directories");
    print!("Enter your choice: ");
    stdout().flush()?;

    let mut choise = String::new();
    stdin().read_line(&mut choise)?;
    let user_input = choise.trim().parse::<u32>().unwrap_or(0);

    let write_files = match user_input {
        3 | 4 | 5 | 6 => true,
        _ => false,
    };
    let write_folders = match user_input {
        1 | 2 | 5 | 6 => true,
        _ => false,
    };
    let recursive = match user_input {
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
        )?;
    }

    Ok(())
}
