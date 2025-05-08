
use anyhow::{Result, anyhow};

#[derive(Debug, Default)]
pub struct RegisterFile {
    r: [u32; 8],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, idx: u8) -> Result<u32> {
        self.r.get(idx as usize)
            .copied()
            .ok_or_else(|| anyhow!("Invalid register index: {}", idx))
    }

    pub fn set(&mut self, idx: u8, val: u32) -> Result<()> {
        if idx >= 8 {
            return Err(anyhow!("Invalid register index: {}", idx));
        }
        self.r[idx as usize] = val;
        Ok(())
    }
}