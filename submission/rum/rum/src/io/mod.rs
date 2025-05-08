
use anyhow::Result;

pub trait IoHandler {
    fn write_byte(&mut self, byte: u8) -> Result<()>;
    fn read_byte(&mut self) -> Result<Option<u8>>;
}

mod terminal;
mod mock;

// Re-export implementations
pub use terminal::TerminalIo;
pub use mock::MockIo;