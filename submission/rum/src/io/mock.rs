
use crate::io::IoHandler;
use anyhow::Result;

pub struct MockIo {
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

impl IoHandler for MockIo {
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.output.push(byte);
        Ok(())
    }

    fn read_byte(&mut self) -> Result<Option<u8>> {
        if self.input.is_empty() {
            return Ok(None);
        }
        Ok(Some(self.input.remove(0)))
    }
}
impl MockIo {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            output: Vec::new(),
        }
    }

    pub fn get_output(&self) -> Vec<u8> {
        self.output.clone()
    }
}