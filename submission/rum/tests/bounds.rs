use anyhow::Result;
use rum::vm::registers::RegisterFile;

#[test]
fn test_invalid_register_read() {
    let reg = RegisterFile::default();
    assert!(reg.get(8).is_err());
    assert!(reg.get(255).is_err());
}

#[test]
fn test_invalid_register_write() {
    let mut reg = RegisterFile::default();
    assert!(reg.set(8, 42).is_err());
    assert!(reg.set(255, 42).is_err());
}

#[test]
fn test_edge_register_access() -> Result<()> {
    let mut reg = RegisterFile::default();
    // First and last valid registers
    reg.set(0, 100)?;
    reg.set(7, 200)?;
    assert_eq!(reg.get(0)?, 100);
    assert_eq!(reg.get(7)?, 200);
    Ok(())
}
