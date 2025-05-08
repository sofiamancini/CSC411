use anyhow::{anyhow, Result};
use bitpack::{BitField, Bitpack};

#[derive(Debug, PartialEq)]  // Derive PartialEq for comparison in tests
pub enum Opcode {
    CMov = 0,
    Load = 1,
    Store = 2,
    Add = 3,
    Mul = 4,
    Div = 5,
    Nand = 6,
    Halt = 7,
    MapSegment = 8,
    UnmapSegment = 9,
    Output = 10,
    Input = 11,
    LoadProgram = 12,
    LoadValue = 13,
    Invalid = 15,
}

impl Opcode {
    pub fn from(val: u32) -> Result<Self> {
        match val {
            0 => Ok(Self::CMov),
            1 => Ok(Self::Load),
            2 => Ok(Self::Store),
            3 => Ok(Self::Add),
            4 => Ok(Self::Mul),
            5 => Ok(Self::Div),
            6 => Ok(Self::Nand),
            7 => Ok(Self::Halt),
            8 => Ok(Self::MapSegment),
            9 => Ok(Self::UnmapSegment),
            10 => Ok(Self::Output),
            11 => Ok(Self::Input),
            12 => Ok(Self::LoadProgram),
            13 => Ok(Self::LoadValue),
            15 => Ok(Self::Invalid),
            _ => Err(anyhow!("Invalid opcode: {}", val)), // Handle invalid opcodes gracefully
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub word: u32,
}

impl Instruction {
    // Decode an instruction from a u32 word
    pub fn decode(word: u32) -> Result<Self> {
        Ok(Self { word })
    }

    // Extract and return the opcode of the instruction
    pub fn opcode(&self) -> Result<Opcode> {
        // Opcode occupies the highest 4 bits (bits 28 to 31)
        let opcode_value = self.word.get_field(BitField::new(28, 4))?;
        Opcode::from(opcode_value)  // Get the opcode and handle invalid opcodes here
    }

    // Extract and return the 'a' register (bits 25:23)
    pub fn get_a(&self) -> Result<u8> {
        // 'a' is bits 25-23 (3 bits)
        Ok(self.word.get_field(BitField::new(23, 3))? as u8)
    }

    // Extract and return the 'b' register (bits 22:20)
    pub fn get_b(&self) -> Result<u8> {
        // 'b' is bits 22-20 (3 bits)
        Ok(self.word.get_field(BitField::new(20, 3))? as u8)
    }

    // Extract and return the 'c' register (bits 19:17)
    pub fn get_c(&self) -> Result<u8> {
        // 'c' is bits 19-17 (3 bits)
        Ok(self.word.get_field(BitField::new(17, 3))? as u8)
    }

    // Extract and return the value (lower 25 bits)
    pub fn get_value(&self) -> Result<u32> {
        // Value is the lower 25 bits (bits 0 to 24)
        self.word.get_field(BitField::new(0, 25))
    }

    // Return registers as a tuple
    pub fn registers(&self) -> (u8, u8, u8) {
        // Here, unwrap_or(0) is used in case of an error (defaults to 0)
        (self.get_a().unwrap_or(0), self.get_b().unwrap_or(0), self.get_c().unwrap_or(0))
    }
}
