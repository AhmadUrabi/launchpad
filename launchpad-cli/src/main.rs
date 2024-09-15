use include_dir::{include_dir, Dir};
use std::path::Path;
use std::{fs, process};

use clap::Parser;

static SRC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../launchpad-src");

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    destination: String,
}

fn main() {
    let matches = Args::parse();
    let destination = matches.destination;
    let dest_dir = Path::new(&destination);

    // Create the destination directory if it doesn't exist
    if let Err(err) = fs::create_dir_all(dest_dir) {
        eprintln!("Error creating destination directory: {}", err);
        process::exit(1);
    }

    // Copy the template files to the destination directory
    for entry in SRC_DIR.entries() {
        let relative_path = entry.path();
        let dest_path = dest_dir.join(relative_path);

        match entry {
            include_dir::DirEntry::File(_) => {
                if let Some(parent) = dest_path.parent() {
                    if let Err(err) = fs::create_dir_all(parent) {
                        eprintln!("Error creating directory {}: {}", parent.display(), err);
                        process::exit(1);
                    }
                }

                if let Err(err) = fs::write(&dest_path, entry.as_file().unwrap().contents()) {
                    eprintln!("Error writing file {}: {}", dest_path.display(), err);
                    process::exit(1);
                }
            }
            include_dir::DirEntry::Dir(_) => {
                if let Err(err) = fs::create_dir_all(&dest_path) {
                    eprintln!("Error creating directory {}: {}", dest_path.display(), err);
                    process::exit(1);
                }
            }
        }
    }

    println!("Destination directory created successfully!")
}
