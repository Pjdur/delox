use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};

fn recreate_test_file(path: &str) {
    fs::write(path, "hello world").unwrap();
}

fn recreate_test_dir(path: &str, files: usize) {
    fs::create_dir_all(path).unwrap();
    for i in 0..files {
        fs::write(format!("{}/file{}.txt", path, i), "hello").unwrap();
    }
}

fn run_many<F: Fn()>(label: &str, runs: usize, f: F) {
    let mut total = Duration::new(0, 0);
    for _ in 0..runs {
        let start = Instant::now();
        f();
        total += start.elapsed();
    }
    let avg = total / runs as u32;
    println!("{label} average over {runs} runs: {:?}", avg);
}

fn main() {
    let file = "benchfile.txt";
    run_many("Delox file delete", 10, || {
        recreate_test_file(file);
        Command::new("target/debug/delox").arg(file).status().unwrap();
    });

    run_many("del file delete", 10, || {
        recreate_test_file(file);
        Command::new("cmd").args(["/C", "del", file]).status().unwrap();
    });

    let dir = "benchdir";
    run_many("Delox dir delete", 10, || {
        recreate_test_dir(dir, 100);
        Command::new("target/debug/delox").arg(dir).status().unwrap();
    });

    run_many("rmdir dir delete", 10, || {
        recreate_test_dir(dir, 100);
        Command::new("cmd").args(["/C", "rmdir", "/S", "/Q", dir]).status().unwrap();
    });
}
