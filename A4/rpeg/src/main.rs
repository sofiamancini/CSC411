//! RPEG Image Compression Command Line Interface
//!
//! Provides command line access to the RPEG image compression system.
//! Supports both compression and decompression of images via stdin/stdout or files.

use std::env;
use std::process;
use rpeg::codec::{compress, decompress};
use std::path::Path;

/// Prints usage instructions to stderr
///
/// # Notes
/// Shows the two supported modes:
/// - `-c` for compression
/// - `-d` for decompression
/// Both modes can work with either files or stdin/stdout
fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  rpeg -c [input] [output]  # Compress image");
    eprintln!("  rpeg -d [input] [output]  # Decompress image");
    eprintln!("Omitting output uses stdout");
}

/// Main entry point for the RPEG compression tool
///
/// # Behavior
/// 1. Parses command line arguments
/// 2. Determines operation mode (compress/decompress)
/// 3. Handles file vs stdin/stdout operation
/// 4. Executes the requested operation
///
/// # Exit Codes
/// - 0: Success
/// - 1: Invalid arguments or operation failed
///
/// # Examples
/// ```bash
/// # Compress a file
/// rpeg -c input.ppm > compressed.rpeg
///
/// # Decompress from stdin
/// cat compressed.rpeg | rpeg -d > output.ppm
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let input = args.get(2).map(|s| s.as_str());
    let output = args.get(3).map(|s| s.as_str());

    let result = match command.as_str() {
        "-c" => {
            if let Some(output_path) = output {
                compress(input).map(|_| {
                    let input_path = input.unwrap_or("");
                    let binding = Path::new(input_path).with_extension("rpeg");
                    let default_path = binding.to_str().unwrap_or("output.rpeg");
                    
                    if output_path.is_empty() {
                        std::fs::rename(default_path, "output.rpeg").unwrap();
                    } else {
                        std::fs::rename(default_path, output_path).unwrap();
                    }
                })
            } else {
                compress(input)
            }
        },
        "-d" => {
            if let Some(output_path) = output {
                decompress(input).map(|_| {
                    let input_path = input.unwrap_or("");
                    let binding = Path::new(input_path).with_extension("ppm");
                    let default_path = binding.to_str().unwrap_or("output.ppm");
                    
                    if output_path.is_empty() {
                        std::fs::rename(default_path, "output.ppm").unwrap();
                    } else {
                        std::fs::rename(default_path, output_path).unwrap();
                    }
                })
            } else {
                decompress(input)
            }
        },
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}