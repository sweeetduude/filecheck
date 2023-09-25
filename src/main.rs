use std::fs;
use std::io::{stdin, stdout, Error, Write};
use std::path::PathBuf;

fn write_to_file(
    output_file: &str,
    name: &str,
    path: &str,
    format: u32,
    file_type: &str,
) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)?;

    let path = if path.starts_with(".") {
        &path[1..]
    } else {
        path
    };

    match format {
        1 => writeln!(file, "{};{};{}", name, path, file_type)?,
        2 => writeln!(file, "{}\t{}\t{}", name, path, file_type)?,
        _ => print!("Invalid format"),
    }
    Ok(())
}

fn list_files(
    path: &PathBuf,
    output_file: &str,
    write_files: bool,
    write_folders: bool,
    list_all: bool,
    format: u32,
) -> Result<(), Error> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_file() && write_files {
                write_to_file(
                    output_file,
                    &entry.file_name().to_string_lossy(),
                    &entry.path().display().to_string(),
                    format,
                    "File",
                )?;
            } else if entry.path().is_dir() {
                if write_folders {
                    write_to_file(
                        output_file,
                        &entry.file_name().to_string_lossy(),
                        &entry.path().display().to_string(),
                        format,
                        "Folder",
                    )?;
                }
                if list_all {
                    list_files(
                        &entry.path(),
                        output_file,
                        write_files,
                        write_folders,
                        true,
                        format,
                    )?;
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
    let user_input = choise.trim().parse::<u32>().unwrap_or(5);

    println!("Select output format:");
    println!("1. CSV (default)");
    println!("2. Text file");
    print!("Enter your format choice: ");
    stdout().flush()?;

    let mut format_choice = String::new();
    stdin().read_line(&mut format_choice)?;
    let user_format = format_choice.trim().parse::<u32>().unwrap_or(1);

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

    let output_file = match user_format {
        1 => "results.csv",
        _ => "results.txt",
    };

    if write_files | write_folders {
        list_files(
            &PathBuf::from("."),
            output_file,
            write_files,
            write_folders,
            recursive,
            user_format,
        )?;
    }

    Ok(())
}
