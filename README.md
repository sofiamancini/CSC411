# CSC411: Machine Organization

Repository for course assignments and labs
University of Rhode Island
Term: Spring 2025

## Course Description

CSC411 – Machine Organization focuses on understanding and unpacking machine code, exploring how high-level abstractions map to low-level operations, and applying that knowledge to optimize performance and memory usage.

## Technologies Used

* **Primary Languages:** Rust, C
* **Tools & Concepts:** Cargo, Unix shell pipelines, GDB, profiling (Hyperfine, sampling), DCT, bit-packing, image formats (PGM/PPM), iterators, generics, exit codes

## How to Approach This Repository

* Each assignment lives in its own top-level folder (`A1/`, `A2/`, …).
* Inside, you’ll find one or more projects (Rust crates or C labs) with `README.md`, source files, and build scripts.
* Use the provided `Makefile` or `cargo build --release` to compile; see each assignment’s README for usage examples.

## Assignment Overview

| Assignment | Description                                                                                                                                                                             | Technologies & Concepts                                            |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| **A1**     | **brightness**: average-brightness calculator for PGM images<br>**fgroups**: group names by fingerprint from stdin                                                                      | Rust, `csc411_image` crate, I/O, `HashMap`                         |
| **A2**     | **array2** (lib): polymorphic 2D array with row-/column-major iterators<br>**sudoku**: validate a solved Sudoku in a 9×9 PGM (exit codes)<br>**technique**: write-up of coding practice | Rust generics, iterators, PGM parsing, exit codes                  |
| **A3**     | **ppmtrans**: image rotations, flips, transpose with Array2<br>row- vs. column-major storage performance analysis, cache locality study                                                 | Rust, `clap`, Array2, benchmarking, cache-locality analysis        |
| **A4**     | **rpeg**: simplified JPEG-style compressor for RGB images<br>color-space conversion, DCT, quantization, bit-packing                                                                     | Rust, DCT, YCbCr, bit-packing, custom compression format           |
| **A5**     | **Binary Bomb Lab**: reverse-engineer x86\_64 phases in C/assembly using GDB, crack input strings and numbers to “defuse” the bomb                                                      | C, x86\_64 assembly, GDB, disassembly                              |
| **A6**     | **Universal Machine Optimizations**: profile the VM hot loop, apply assembly-level tuning (invariant hoisting, dispatch refactoring), measure speedups                                  | Rust, profiling (flamegraphs), LTO, assembly optimization          |
| **A7**     | **Performance Tuning for UM**: iterative profiling and code-tuning of the Universal Machine using `samply`/`flamegraph`, detailed lab notes, assembly analysis, and benchmark results   | Profiling, Git-based iteration, cargo profiling, lab documentation |

## Getting Started

```bash
# Clone the repo
git clone https://github.com/sofiamancini/CSC411.git
cd CSC411

# Build all Rust projects
cargo build --release

# Example usage:
# Assignment 1 – brightness
./target/release/brightness assignment1/brightness/test.pgm

# Assignment 1 – fgroups
cat assignment1/fgroups/fingerprints.txt | ./target/release/fgroups

# Assignment 2 – sudoku
djpeg -grayscale assignment2/sudoku/puzzle.pgm | ./target/release/sudoku

# Assignment 3 – ppmtrans
ppmtrans --rotate 90 --row-major < assignment3/ppmtrans/input.ppm > out.ppm

# Assignment 4 – rpeg
rpeg -c input.ppm > compressed.rpeg
rpeg -d compressed.rpeg > output.ppm

# Assignment 5 – bomb
# Compile C lab and use GDB as described in assignment5/README.md

# Assignment 6 – UM optimizations
# Run benchmarks as described in assignment6/README.md

# Assignment 7 – UM performance tuning
# Follow instructions in assignment7/README.md for profiling, lab notes, and final benchmarks
```

---

*Explore each assignment folder for detailed build instructions, test data, and usage examples.*
