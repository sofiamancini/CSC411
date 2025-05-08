use rum::{
    TerminalIo,
    loader::load_program,
    vm::VirtualMachine,
};
use anyhow::{Context, Result};
use std::{env, fs, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        anyhow::bail!("Usage: {} <um_file>", args[0]);
    }

    // Load and validate program
    let bytes = fs::read(&args[1])
        .with_context(|| format!("Failed to read file: {}", args[1]))?;
    let program = load_program(&bytes)
        .with_context(|| "Invalid UM program format")?;

    // Initialize VM with terminal I/O
    let io = TerminalIo;
    let mut vm = VirtualMachine::new(program, io);

    // Main execution loop
    while vm.running {
        vm.step()
            .with_context(|| format!("Execution failed at PC={}", vm.pc))?;
    }

    Ok(())
}