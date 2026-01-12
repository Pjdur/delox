# Delox

Delox is a command-line tool for deleting files and folders.  
Unlike Windows `del` (files only) and `rmdir` (folders only), Delox provides a unified interface that works for both.

## Installation

Build from source with Cargo:

```bash
cargo build --release
```

The compiled binary will be available in `target/release/delox`.

## Usage

Delete a file or folder with a single command:

```bash
delox <file-or-folder>
```

Examples:

```bash
delox notes.txt
delox projects/
```

## Performance

Benchmarks were run on Windows using a custom harness.  
Each test was repeated 10 times and averaged.

| Operation              | Delox (avg) | Native command (avg) |
|------------------------|-------------|-----------------------|
| File deletion          | 33.63 ms    | `del`: 156.19 ms      |
| Directory deletion (100 files) | 596.18 ms   | `rmdir /S /Q`: 750.40 ms |

### Notes
- Results vary depending on filesystem caching and system load.  
- Delox shows competitive performance, and in directory deletion workloads it outperforms `rmdir`.  
- Future improvements (e.g. multithreading) may further reduce deletion times.

## License

MIT License
