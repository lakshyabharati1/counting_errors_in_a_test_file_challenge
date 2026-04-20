# Counting Errors in a Test File

A small Rust CLI that counts how many times the word `error` appears in `test.txt`.

The program parallelizes the work by splitting the file into chunks and processing those chunks across CPU threads.

## What It Does

- Reads `test.txt` from the project root.
- Splits the file into chunk ranges based on available CPU cores.
- Uses one thread per core (`num_cpus`) to process chunks concurrently.
- Handles chunk boundaries so lines are not double-counted.
- Counts matches case-insensitively (`ERROR`, `Error`, `error`, etc.).

## Project Structure

- `src/main.rs`: main logic for chunking, threading, and counting.
- `test.txt`: sample input log file.
- `Cargo.toml`: project manifest and dependencies.

## Requirements

- Rust toolchain installed (Cargo + rustc).

Install Rust if needed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run

From the project directory:

```bash
cargo run --release
```

Expected output format:

```text
Total Error Counter: <number>
```

For the provided `test.txt`, the expected count is:

```text
Total Error Counter: 2400
```

## How Chunk Processing Works

1. Get the total file size.
2. Divide size by number of threads to compute chunk sizes.
3. Each thread seeks to its chunk start offset.
4. If a chunk does not start at byte `0`, it discards the first partial line to align at a line boundary.
5. Read lines until the chunk end and count `error` matches per line.
6. Add each thread's count to a shared atomic counter.

## Notes

- Input path is currently hardcoded to `test.txt`.
- Search term is currently hardcoded to `error`.
- Counting is based on substring matches, not whole-word matching.

## Future Improvements

- Accept input file path and search term via CLI arguments.
- Add whole-word matching mode.
- Add tests and benchmarks for very large files.
- Improve chunk distribution for skewed line lengths.
