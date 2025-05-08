pub mod bitfield;
pub mod bitpack;

pub use bitfield::{BitField, Bitpack};

/// Re-export of all public bitpack operations for convenient access
pub use crate::bitpack::{
    fitss,  // Check if signed value fits in bit width
    fitsu,  // Check if unsigned value fits in bit width
    gets,   // Extract signed value from word
    getu,   // Extract unsigned value from word
    news,   // Pack signed value into word
    newu,   // Pack unsigned value into word
};