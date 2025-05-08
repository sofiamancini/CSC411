
pub mod registers;
pub mod memory;
pub mod executor;

pub use executor::ExecutionUnit;
pub use registers::RegisterFile;
pub use memory::SegmentTable;

use crate::io::IoHandler;
use crate::disasm::decoder::Instruction;


// Make VirtualMachine generic over any type implementing IoHandler
pub struct VirtualMachine<IO: IoHandler> {
    pub registers: RegisterFile,
    pub memory: SegmentTable,
    pub io: IO, 
    pub pc: u32,
    pub running: bool,
}

impl<IO: IoHandler> VirtualMachine<IO> {
    pub fn new(program: Vec<u32>, io: IO) -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: SegmentTable::new(program),
            io,
            pc: 0,
            running: true,
        }
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        let word = self.memory.fetch(self.pc)?;
        let inst = Instruction::decode(word)?;
        ExecutionUnit::execute(self, inst)?;
        self.pc = self.pc.wrapping_add(1);
        Ok(())
    }
}