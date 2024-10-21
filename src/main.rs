use clap::{Arg, Command};
use std::fs;
use std::path::{Path};
use std::error::Error;

fn organize_files(dir: &Path) -> Result<(), Box<dyn Error>> {
    // Iterate over entries in the specified directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            // Get the file extension
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy();
                let dest_dir = dir.join(ext_str.as_ref());

                // Create a directory for the extension if it doesn't exist
                if !dest_dir.exists() {
                    fs::create_dir(&dest_dir)?;
                }

                // Move the file into the corresponding directory
                let new_path = dest_dir.join(path.file_name().unwrap());
                fs::rename(&path, new_path)?;
                println!("Moved: {:?}", path.file_name().unwrap());
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("File Organizer")
        .version("1.0")
        .about("Organizes files in a directory by their extensions")
        .arg(
            Arg::new("directory")
                .help("The directory to organize")
                .required(true)
                .index(1),
        )
        .get_matches();

    let dir = matches.get_one::<String>("directory").unwrap();
    let path = Path::new(dir);

    if path.exists() && path.is_dir() {
        organize_files(&path)?;
        println!("Files organized successfully.");
    } else {
        eprintln!("Provided path is not a valid directory.");
    }

    Ok(())
}
