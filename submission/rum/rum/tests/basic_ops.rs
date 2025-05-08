use anyhow::Result;
use rum::vm::registers::RegisterFile;

#[test]
fn test_register_read_write() -> Result<()> {
    let mut reg = RegisterFile::default();
    reg.set(2, 42)?;
    assert_eq!(reg.get(2)?, 42);
    Ok(())
}

#[test]
fn test_default_zeroed() -> Result<()> {
    let reg = RegisterFile::default();
    for i in 0..8 {
        assert_eq!(reg.get(i)?, 0);
    }
    Ok(())
}
