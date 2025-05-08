
use crate::io::IoHandler;
use anyhow::Result;
use std::io::{self, Write, Read};

pub struct TerminalIo;

impl IoHandler for TerminalIo {
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        io::stdout().write_all(&[byte])?;
        io::stdout().flush()?;
        Ok(())
    }

    fn read_byte(&mut self) -> Result<Option<u8>> {
        let mut buf = [0];
        match io::stdin().read_exact(&mut buf) {
            Ok(_) => Ok(Some(buf[0])),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}