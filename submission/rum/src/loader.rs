use anyhow::Result;
use std::io::{Cursor, Read};

/// Loads a UM program from bytes (big-endian format)
pub fn load_program(bytes: &[u8]) -> Result<Vec<u32>> {
    // Check if the input length is a multiple of 4
    if bytes.len() % 4 != 0 {
        anyhow::bail!("Input length must be a multiple of 4 bytes");
    }

    let mut program = Vec::with_capacity(bytes.len() / 4);
    let mut cursor = Cursor::new(bytes);

    // Manually read 4 bytes at a time and convert to u32
    while cursor.position() < bytes.len() as u64 {
        let mut word_bytes = [0u8; 4];
        cursor.read_exact(&mut word_bytes)?;
        
        // Convert big-endian bytes to u32
        let word = u32::from_be_bytes(word_bytes);
        program.push(word);
    }

    if program.is_empty() {
        anyhow::bail!("Empty or invalid UM program");
    }

    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_program() {
        // Big-endian representation of [0x00000001, 0x00000002]
        let bytes = vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let program = load_program(&bytes).unwrap();
        assert_eq!(program, vec![1, 2]);
    }

    #[test]
    fn test_invalid_length() {
        let bytes = vec![0x00, 0x00, 0x01]; // Not multiple of 4
        assert!(load_program(&bytes).is_err());
    }
}