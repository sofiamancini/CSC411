
use anyhow::Result;

pub struct BitField {
    pub offset: u32,
    pub width: u32,
}

impl BitField {
    pub fn new(offset: u32, width: u32) -> Self {
        Self { offset, width }
    }
}

pub trait Bitpack {
    fn get_field(&self, field: BitField) -> Result<u32>;
}

impl Bitpack for u32 {
    fn get_field(&self, field: BitField) -> Result<u32> {
        if field.offset + field.width > 32 {
            return Err(anyhow::anyhow!("Bit field out of range"));
        }
        Ok((self >> field.offset) & ((1 << field.width) - 1))
    }
}
