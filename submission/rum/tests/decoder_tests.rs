use anyhow::Result;
use rum::disasm::decoder::{Instruction, Opcode};

#[test]
fn test_decode_add() -> Result<()> {
    let inst = Instruction::decode(0x3000_0000)?;
    assert_eq!(inst.opcode()?, Opcode::Add);
    Ok(())
}

#[test]
fn test_decode_all_ones() -> Result<()> {
    let inst = Instruction::decode(0xFFFF_FFFF)?;
    assert_eq!(inst.opcode()?, Opcode::Invalid);  // Check that 15 maps to Invalid
    Ok(())
}


#[test]
fn test_decode_load() -> Result<()> {
    let inst = Instruction::decode(0x1000_0000)?;
    assert_eq!(inst.opcode()?, Opcode::Load);
    Ok(())
}

#[test]
fn test_get_a_register() -> Result<()> {
    let inst = Instruction::decode(0x3000_0000)?;
    assert_eq!(inst.get_a()?, 0);  // Based on the value of 'a' in 0x3000_0000
    Ok(())
}

