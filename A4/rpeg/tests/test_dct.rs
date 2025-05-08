//! Comprehensive tests for Discrete Cosine Transform (DCT) operations.
//!
//! Tests cover:
//! - Forward and inverse DCT transformations
//! - Quantization and dequantization
//! - Special case patterns
//! - Round-trip accuracy

use rpeg::dct::{dct, inverse_dct, quantize, dequantize, DctCoefficients};

/// Tests basic DCT transformation with known input values
///
/// # Test Steps
/// 1. Creates a simple 2x2 block with known values
/// 2. Computes DCT coefficients
/// 3. Verifies each coefficient against expected values
///
/// # Notes
/// - Uses exact calculations for expected values
/// - Tests all four DCT coefficients (DC, vertical, horizontal, diagonal)
#[test]
fn test_dct_basic() {
    // Block mapping:
    // [0.1, 0.2]  [Y1, Y2]
    // [0.3, 0.4]  [Y3, Y4]
    let block = [0.1, 0.2, 0.3, 0.4];
    println!("Testing basic DCT with block: {:?}", block);
    
    let coeffs = dct(&block);
    println!("DCT coefficients: a={:.5}, b={:.5}, c={:.5}, d={:.5}", 
             coeffs.a, coeffs.b, coeffs.c, coeffs.d);
    
    // Expected values calculated from equations:
    // a = (0.4 + 0.3 + 0.2 + 0.1)/4 = 1.0/4 = 0.25
    // b = (0.4 + 0.3 - 0.2 - 0.1)/4 = 0.4/4 = 0.1
    // c = (0.4 - 0.3 + 0.2 - 0.1)/4 = 0.2/4 = 0.05
    // d = (0.4 - 0.3 - 0.2 + 0.1)/4 = 0.0/4 = 0.0
    
    println!("Checking DC component (a)");
    assert!((coeffs.a - 0.25).abs() < 1e-5, "DC component incorrect. Expected ~0.25, got {:.5}", coeffs.a);
    
    println!("Checking vertical frequency (b)");
    assert!((coeffs.b - 0.1).abs() < 1e-5, "Vertical frequency incorrect. Expected ~0.1, got {:.5}", coeffs.b);
    
    println!("Checking horizontal frequency (c)");
    assert!((coeffs.c - 0.05).abs() < 1e-5, "Horizontal frequency incorrect. Expected ~0.05, got {:.5}", coeffs.c);
    
    println!("Checking diagonal frequency (d)");
    assert!((coeffs.d - 0.0).abs() < 1e-5, "Diagonal frequency should be 0. Got {:.5}", coeffs.d);
}

/// Tests quantization of DCT coefficients
///
/// # Test Steps
/// 1. Creates DCT coefficients with known values
/// 2. Quantizes the coefficients
/// 3. Verifies quantized values against expected results
///
/// # Notes
/// - DC coefficient uses 9-bit unsigned quantization
/// - AC coefficients use 5-bit signed quantization
#[test]
fn test_quantization() {
    let coeffs = DctCoefficients {
        a: 0.8,
        b: 0.25,
        c: -0.35,
        d: 0.4
    };
    println!("Testing quantization with coefficients: {:?}", coeffs);
    
    let (a, b, c, d) = quantize(&coeffs);
    println!("Quantized values: a={}, b={}, c={}, d={}", a, b, c, d);
    
    // Corrected expected values
    assert_eq!(a, 409, "DC quantization failed. Expected 409, got {}", a);
    assert_eq!(b, 12, "Vertical frequency quantization failed. Expected 12, got {}", b);
    assert_eq!(c, -15, "Horizontal frequency quantization failed. Expected -15, got {}", c);
    assert_eq!(d, 15, "Diagonal frequency quantization failed. Expected 15, got {}", d);
}

/// Tests dequantization of packed values
///
/// # Test Steps
/// 1. Creates quantized values
/// 2. Dequantizes back to floating-point coefficients
/// 3. Verifies reconstructed coefficients
///
/// # Notes
/// - Tests reverse of quantization process
/// - Uses relaxed tolerances due to quantization error
#[test]
fn test_dequantization() {
    let quantized = (255, 10, -15, 5);
    println!("Testing dequantization with quantized values: {:?}", quantized);
    
    let coeffs = dequantize(quantized.0, quantized.1, quantized.2, quantized.3);
    println!("Dequantized coefficients: a={:.5}, b={:.5}, c={:.5}, d={:.5}", 
             coeffs.a, coeffs.b, coeffs.c, coeffs.d);
    
    // Adjusted tolerances
    assert!((coeffs.a - 0.5).abs() < 0.001, "DC dequantization failed. Expected ~0.5, got {:.5}", coeffs.a);
    assert!((coeffs.b - 0.2).abs() < 1e-4, "Vertical frequency dequantization failed. Expected ~0.2, got {:.5}", coeffs.b);
    assert!((coeffs.c + 0.3).abs() < 1e-4, "Horizontal frequency dequantization failed. Expected ~-0.3, got {:.5}", coeffs.c);
    assert!((coeffs.d - 0.1).abs() < 1e-4, "Diagonal frequency dequantization failed. Expected ~0.1, got {:.5}", coeffs.d);
}

/// Tests full reconstruction via inverse DCT
///
/// # Test Steps
/// 1. Creates original pixel block
/// 2. Computes DCT coefficients
/// 3. Reconstructs block using inverse DCT
/// 4. Verifies pixel values match original
///
/// # Notes
/// - Uses tight tolerances for exact reconstruction
#[test]
fn test_inverse_dct() {
    let original = [0.5, 0.3, 0.8, 0.1];
    println!("Testing inverse DCT with block: {:?}", original);
    
    let coeffs = dct(&original);
    println!("DCT coefficients: {:?}", coeffs);
    
    let reconstructed = inverse_dct(&coeffs);
    println!("Reconstructed block: {:?}", reconstructed);
    
    for (i, (orig, rec)) in original.iter().zip(reconstructed.iter()).enumerate() {
        let diff = (orig - rec).abs();
        println!("Pixel {}: original={}, reconstructed={}, diff={:.5}", i, orig, rec, diff);
        assert!(diff < 1e-5, "Pixel {} reconstruction error too large: {:.5}", i, diff);
    }
}

/// Tests uniform block (all identical values)
///
/// # Test Steps
/// 1. Creates block with uniform values
/// 2. Computes DCT coefficients
/// 3. Verifies only DC component is non-zero
///
/// # Notes
/// - Tests edge case where only DC component should exist
#[test]
fn test_uniform_block() {
    let block = [0.5, 0.5, 0.5, 0.5];
    println!("Testing uniform block: {:?}", block);
    
    let coeffs = dct(&block);
    println!("DCT coefficients: {:?}", coeffs);
    
    assert!((coeffs.a - 0.5).abs() < 1e-5, "DC component incorrect. Expected 0.5, got {:.5}", coeffs.a);
    assert!(coeffs.b.abs() < 1e-5, "Vertical frequency should be 0. Got {:.5}", coeffs.b);
    assert!(coeffs.c.abs() < 1e-5, "Horizontal frequency should be 0. Got {:.5}", coeffs.c);
    assert!(coeffs.d.abs() < 1e-5, "Diagonal frequency should be 0. Got {:.5}", coeffs.d);
}

/// Tests vertical gradient pattern
///
/// # Test Steps
/// 1. Creates block with vertical gradient
/// 2. Computes DCT coefficients
/// 3. Verifies expected frequency components
///
/// # Notes
/// - Should only activate DC and vertical frequency components
#[test]
fn test_vertical_gradient() {
    let block = [0.0, 0.0, 1.0, 1.0];
    println!("Testing vertical gradient block: {:?}", block);
    
    let coeffs = dct(&block);
    println!("DCT coefficients: {:?}", coeffs);
    
    assert!((coeffs.a - 0.5).abs() < 1e-5, "DC component incorrect. Expected 0.5, got {:.5}", coeffs.a);
    assert!((coeffs.b - 0.5).abs() < 1e-5, "Vertical frequency incorrect. Expected 0.5, got {:.5}", coeffs.b);
    assert!(coeffs.c.abs() < 1e-5, "Horizontal frequency should be 0. Got {:.5}", coeffs.c);
    assert!(coeffs.d.abs() < 1e-5, "Diagonal frequency should be 0. Got {:.5}", coeffs.d);
}

/// Tests full compression/decompression round-trip
///
/// # Test Steps
/// 1. Creates original pixel block
/// 2. Performs DCT + quantization
/// 3. Performs dequantization + inverse DCT
/// 4. Verifies reconstructed values
///
/// # Notes
/// - Uses relaxed tolerance to account for quantization error
/// - Tests complete pipeline from pixels to coefficients and back
#[test]
fn test_round_trip() {
    let original = [0.1, 0.2, 0.3, 0.4];
    println!("Testing round trip with block: {:?}", original);
    
    let coeffs = dct(&original);
    println!("DCT coefficients: {:?}", coeffs);
    
    let quantized = quantize(&coeffs);
    println!("Quantized values: {:?}", quantized);
    
    let dequantized = dequantize(quantized.0, quantized.1, quantized.2, quantized.3);
    println!("Dequantized coefficients: {:?}", dequantized);
    
    let reconstructed = inverse_dct(&dequantized);
    println!("Reconstructed block: {:?}", reconstructed);
    
    for (i, (orig, rec)) in original.iter().zip(reconstructed.iter()).enumerate() {
        let diff = (orig - rec).abs();
        println!("Pixel {}: original={}, reconstructed={}, diff={:.5}", i, orig, rec, diff);
        assert!(diff < 0.015, "Pixel {} reconstruction error too large: {:.5}", i, diff);
    }
}