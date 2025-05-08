//! RPEG Image Compression Library
//!
//! Provides lossy compression for RGB images using:
//! - Color space conversion (RGB ↔ YCbCr)
//! - Discrete Cosine Transform (DCT) on 2x2 blocks
//! - Chroma subsampling and coefficient quantization
//!
//! # Key Modules
//! - `codec`: Compression/decompression interface
//! - `color_space`: RGB/YCbCr conversion utilities
//! - `dct`: Discrete Cosine Transform operations
//!
//! # Example Usage
//! ```no_run
//! use rpeg::{compress, decompress};
//!
//! // Compress an image file
//! compress(Some("input.ppm")).unwrap();
//!
//! // Decompress from stdin to stdout
//! decompress(None).unwrap();
//! ```

pub mod codec;
pub mod color_space;
pub mod dct;

/// Re-exports of commonly used types and functions
pub mod prelude {
    pub use super::color_space::{YCbCrPixel, RgbToYcbcr, YcbcrToRgb};
    pub use super::dct::{dct, inverse_dct};
}

// Re-export key public APIs at crate root
pub use color_space::{YCbCrPixel, RgbToYcbcr, YcbcrToRgb, approx_equal, average_chroma, trim_image};
pub use dct::{dct, inverse_dct};
pub use codec::{compress, decompress};

#[cfg(test)]
mod tests {
    //! Unit tests for core library functionality
}

#[cfg(test)]
#[path = "tests/test_dct.rs"]
mod test_dct;

#[cfg(test)]
#[path = "tests/test_codec.rs"]
mod test_codec;

#[cfg(test)]
#[path = "tests/test_color_space.rs"]
mod test_color_space;