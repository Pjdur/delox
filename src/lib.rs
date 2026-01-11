use std::fs;
use std::path::Path;

pub fn delete_path(target: &str) {
    let path = Path::new(target);

    if path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => println!("Deleted file: {}", target),
            Err(e) => eprintln!("Error deleting file: {}", e),
        }
    } else if path.is_dir() {
        match fs::remove_dir(path) {
            Ok(_) => println!("Deleted empty folder: {}", target),
            Err(_) => {
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
