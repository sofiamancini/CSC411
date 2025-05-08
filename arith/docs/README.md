
# RPEG Image Compression System

## Overview

RPEG is a lossy image compression system that implements a simplified version of JPEG-style compression for RGB images. The system performs color space conversion, discrete cosine transform (DCT), quantization, and bit-packing to achieve compression.

## Project Structure

### Main Components
    rpeg/                    # Main compression application
    ├── src/
    │   ├── main.rs          # CLI interface
    │   ├── lib.rs           # Library exports
    │   ├── codec.rs         # Compression/decompression implementation
    │   ├── color_space.rs   # RGB ↔ YCbCr conversion
    │   └── dct.rs           # Discrete Cosine Transform operations

    bitpack/                 # Bit-packing utilities
    ├── src/
    │   ├── bitpack.rs       # Bit manipulation functions
    │   └── lib.rs           # Library exports

    array2/                  # 2D array implementation
    ├── src/
    │   ├── array2.rs        # 2D array operations
    │   └── lib.rs           # Library exports

## Implmentation Status

### Fully Implemented Features
- #### **Color Space Conversation**
    - RGB to YCbCr and back
    - Chroma subsampling (averaging for 2x2 blocks)

- #### **DCT Operations**
    - Forward and inverse DCT for 2x2 blocks
    - Coefficient quantization/dequantization

- #### **Bit Packing**
    - Packing/unpacking signed/unsigned values
    - Width and position validation

- #### **Codec**
    - Complete compression pipeline
    - Complete decompression pipeline
    - Handling of odd dimensions (trimming to even)

- #### **Testing**
    - Comprehensive unit tests for each component
    - Round-trip verification tests

### Not Implemented

- Unit tests for entire program
- Variable quantization tables
- Full JPEG file format support (using custom RPEG format)

## Architecture

### Compression Pipeline

1. **Color Space Conversion**
    - Convert RGB to YCbCr color space
    - Subsample chroma (Cb, Cr) components for 2x2 blocks

2. **DCT Transformation**
    - Apply 2x2 DCT to each block of luma (Y) components
    - Quantize DCT coefficients (9 bits for DC, 5 bits for AC)

3. **Bit Packing**
    - Pack quantized coefficients into 32-bit words
    - Include chroma indices in packed output

4. **Output**
    - Write compressed data in custom RPEG format

### Decompression Pipeline

1. **Bit Unpacking**
    - Extract coefficients and chroma indices
    - Validate bit widths and positions

2. **Inverse Quantization**
    - Reconstruct DCT coefficients from packed values
    - Dequantize using fixed scaling factors

3. **Inverse DCT**
    - Transform coefficients back to spatial domain
    - Clamp values to valid range [0.0, 1.0]

4. **Color Space Conversion**
    - Convert YCbCr back to RGB
    - Upsample chroma components

## Development Metrics

### Time Breakdown
- Initial Research/Practice: ~15 hours
    - Learning about JPEG/Lossy image compression
    - Planning the architecture/ general function design
    - Learning how to write and verify test cases
    - Understanding DCT & quantization
- Implementation: ~40 hours
    - Modifying array2: ~1 hours
    - Bitpack core and helper functions: ~8hours
    - color_space.rs: ~5 hours
    - dct.rs: ~2 hours
    - codec.rs: ~10 hours
    - I/O handling and main.rs: ~1 hours
    - Creating tests: ~5 hours
    - Debugging files & tests: ~8 hours
- Documentation: ~5 hours
    - README.md: ~1 hours
    - Cargo doc compatible comments: ~4 hours

## Building and Testing

### Requirements
- Rust 1.60+ (edition 2021)
- Cargo package manager

### Build Instructions

    # Build all components
    cargo build --release

    # Build documentation
    cargo doc --open

### Test Instructions

    # Run all tests
    cargo test

    # Run specific test suite
    cargo test --test test_test_dct.rs

### Usage

    # Compress an image
    rpeg -c input.ppm > output.rpeg

    # Decompress an image
    rpeg -d output.rpeg > reconstructed.ppm

## Documentation
Additional design documentation available in [Design Documentation](./docs/design.md)