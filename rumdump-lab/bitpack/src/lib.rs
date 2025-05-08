//! A library for efficient bit-level packing and unpacking operations.
//!
//! Provides utilities for working with packed integer values within 64-bit words,
//! including signed and unsigned packing/unpacking with arbitrary bit widths.
//!
//! # Key Features
//! - Safe packing/unpacking of signed and unsigned integers
//! - Bit-width validation checks
//! - Flexible bit positioning within words
//! - Comprehensive error handling
//!
//! # Examples
//! ```
//! use bitpack::{news, gets};
//!
//! // Pack a signed value
//! let packed = news(0, 4, 4, -1).unwrap();
//! assert_eq!(packed, 0b11110000);
//!
//! // Unpack the value
//! assert_eq!(gets(packed, 4, 4), Some(-1));
//! ```

pub mod bitpack;

/// Re-export of all public bitpack operations for convenient access
pub use crate::bitpack::{
    fitss,  // Check if signed value fits in bit width
    fitsu,  // Check if unsigned value fits in bit width
    gets,   // Extract signed value from word
    getu,   // Extract unsigned value from word
    news,   // Pack signed value into word
    newu,   // Pack unsigned value into word
};