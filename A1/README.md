# CSC411 Assignment 1: Interfaces in Rust

- **brightness**  
  Computes the average brightness of a PGM (portable graymap) image, printing a decimal value with one digit before and three after the decimal point.

- **fgroups**  
  Reads lines of “fingerprint <whitespace> name” pairs from standard input and prints groups of names sharing the same fingerprint (only fingerprints appearing two or more times).

## Requirements

- Rust (1.60 or later) and Cargo
- (For `brightness`) the [`csc411_image`](https://crates.io/crates/csc411_image) crate
- Unix-like shell (macOS, Linux, or WSL) for piping and PGM utilities

## Building

    # From the root of this repo:
    cargo build --release --bin brightness
    cargo build --release --bin fgroups

## Usage

### brightness
    # From file:
    ./target/release/brightness path/to/image.pgm

    # From stdin (e.g., converting a JPEG to PGM):
    djpeg -grayscale bear.jpg | ./target/release/brightness

Outputs a single line like:
0.972

### fgroups

    # Read from stdin:
    cat fingerprints.txt | ./target/release/fgroups
Prints each group of names (one per line), separated by a blank line.