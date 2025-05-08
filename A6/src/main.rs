mod vm;
mod loader;

use std::env;
use vm::VirtualMachine;
use loader::load_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <program.um>", args[0]);
        std::process::exit(1);
    }
    let program = load_program(&args[1]).expect("Failed to load program");
    let mut vm = VirtualMachine::new(program);
    vm.run();
}
