use std::fs::{self, File};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_hello_program() {
    let program = include_bytes!("../umbin/hello.um");
    let output = run_um_program(program);
    assert!(output.contains("Hello"), "Hello program failed. Output: '{}'", output);
}

#[test]
fn test_midmark_program() {
    let program = include_bytes!("../umbin/midmark.um");
    let output = run_um_program(program);
    assert!(!output.is_empty(), "Midmark program produced no output");
}

#[test]
fn test_cat_program() {
    let program = include_bytes!("../umbin/cat.um");
    let input = "Test input\n";
    let output = run_um_program_with_input(program, input);
    assert_eq!(output, input, "Cat program failed. Output: '{}'", output);
}

// Helper function to run a UM program
fn run_um_program(program: &[u8]) -> String {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp directory");
    let um_path = dir.path().join("program.um");
    
    // Write the program to a temporary file
    fs::write(&um_path, program).expect("Failed to write program file");
    
    // Run the UM executable
    let output = Command::new("./target/release/rum")
        .arg(um_path)
        .output()
        .expect("Failed to execute UM program");
    
    // Convert output to string
    String::from_utf8(output.stdout).unwrap_or_else(|_| String::new())
}

// Helper function to run a UM program with input
fn run_um_program_with_input(program: &[u8], input: &str) -> String {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp directory");
    let um_path = dir.path().join("program.um");
    let input_path = dir.path().join("input.txt");
    
    // Write the program and input to temporary files
    fs::write(&um_path, program).expect("Failed to write program file");
    fs::write(&input_path, input).expect("Failed to write input file");
    
    // Run the UM executable with input redirected
    let output = Command::new("./target/release/rum")
        .arg(um_path)
        .stdin(File::open(input_path).expect("Failed to open input file"))
        .output()
        .expect("Failed to execute UM program");
    
    // Convert output to string
    String::from_utf8(output.stdout).unwrap_or_else(|_| String::new())
}