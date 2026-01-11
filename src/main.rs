use std::env;
use delox::delete_path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: delox <file-or-folder>");
        return;
    }

    for target in &args[1..] {
        delete_path(target);
    }
}
