# CSC411 Assignment 2: Interfaces, Implementations, and Images

- **array2** (library)  
  A polymorphic, two-dimensional array abstraction with:
  - `Array2<T>` supporting row-major and column-major iteration.
  - Constructors like `from_row_major`, `from_col_major`, and `with_default`.
  - Element access via `(x, y)` coordinates.

- **sudoku** (binary)  
  A command-line tool that reads a 9×9 grayscale PGM from a file or stdin and exits:
  - `exit(0)` if the image encodes a valid solved Sudoku (denominator = 9, no zero pixels, all rows/columns/3×3 blocks contain 1–9 exactly once).
  - `exit(1)` otherwise.

## Requirements

- Rust toolchain (1.60 or later) and Cargo  
- [`csc411_image`](https://crates.io/crates/csc411_image) crate  
- Unix-style shell for piping or reading from stdin

## Building


    # From the root of this repo:
    cargo build --release --package array2
    cargo build --release --package sudoku

## Usage

### sudoku

    # From a file:
    ./target/release/sudoku path/to/puzzle.pgm

    # From stdin:
    djpeg -grayscale puzzle.jpg | ./target/release/sudoku

- No output is printed.
- Check the exit code ($? in Bash) for success (0) or failure (1).