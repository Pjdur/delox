use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: delox <file-or-folder>");
        return;
    }

    let target = &args[1];
    let path = Path::new(target);

    if path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => println!("Deleted file: {}", target),
            Err(e) => eprintln!("Error deleting file: {}", e),
        }
    } else if path.is_dir() {
        // remove_dir only works if the directory is empty
        match fs::remove_dir(path) {
            Ok(_) => println!("Deleted empty folder: {}", target),
            Err(_) => {
                // fallback: remove everything inside
                match fs::remove_dir_all(path) {
                    Ok(_) => println!("Deleted folder and contents: {}", target),
                    Err(e) => eprintln!("Error deleting folder: {}", e),
                }
            }
        }
    } else {
        eprintln!("Target not found: {}", target);
    }
}
