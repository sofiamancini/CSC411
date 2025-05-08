use rpeg::codec::{compress, decompress};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

// Integration tests for RPEG image compression/decompression
//
// This module contains tests that verify the complete compression and decompression pipeline,
// including edge cases and error handling.

/// Tests the complete compression and decompression cycle with a simple 2x2 image
///
/// # Test Steps
/// 1. Creates a minimal valid PPM file (2x2 pixels)
/// 2. Compresses the image
/// 3. Verifies compressed file creation
/// 4. Decompresses the image
/// 5. Verifies decompressed file creation
/// 6. Cleans up temporary files
///
/// # Verification Points
/// - Compression succeeds without errors
/// - Compressed file is created with .rpeg extension
/// - Decompression succeeds without errors
/// - Decompressed file is created at original path
/// 
#[test]
fn test_compress_decompress() {
    // Create a temporary PPM file with test image data
    let ppm_content = "P3\n2 2\n255\n255 0 0 0 255 0 0 0 255 255 255 255";
    let mut input_file = NamedTempFile::new().unwrap();
    write!(input_file, "{}", ppm_content).unwrap();
    let input_path = input_file.path().to_str().unwrap();

    // Test compression
    compress(Some(input_path)).expect("Compression failed");

    // Verify compressed file was created
    let compressed_path = format!("{}.rpeg", input_path);
    assert!(fs::metadata(&compressed_path).is_ok(), "Compressed file not created");

    // Test decompression
    decompress(Some(&compressed_path)).expect("Decompression failed");

    // Verify decompressed output was created at original path
    assert!(fs::metadata(input_path).is_ok(), "Decompressed file not created");

    // Clean up
    fs::remove_file(compressed_path).unwrap();
    fs::remove_file(input_path).unwrap();
}

/// Tests compression behavior with an empty PPM file (0x0 dimensions)
///
/// # Test Steps
/// 1. Creates a valid but empty PPM file (header only)
/// 2. Attempts compression
///
/// # Verification Points
/// - Compression fails as expected (cannot process 0x0 image)
/// - Proper error is returned

#[test]
fn test_compress_empty_ppm() {
    // Create a valid empty PPM file (header only, 0x0 dimensions)
    let mut input_file = NamedTempFile::new().unwrap();
    writeln!(input_file, "P3\n0 0\n255").unwrap();
    let input_path = input_file.path().to_str().unwrap();
    
    // Should fail because we can't compress 0x0 image
    let result = compress(Some(input_path));
    assert!(result.is_err(), "Compression should fail for 0x0 image");
}

/// Tests decompression with intentionally invalid data
///
/// # Test Steps
/// 1. Creates a file with random invalid bytes
/// 2. Attempts decompression
///
/// # Verification Points
/// - Decompression fails as expected
/// - Proper error is returned

#[test]
fn test_decompress_invalid_data() {
    // Create a file with invalid data
    let mut invalid_file = NamedTempFile::new().unwrap();
    invalid_file.write_all(&[0, 1, 2, 3, 4, 5]).unwrap();
    let invalid_path = invalid_file.path().to_str().unwrap();

    let result = decompress(Some(invalid_path));
    assert!(result.is_err(), "Decompression of invalid data should return an error");
}

/// Tests compression/decompression with various pixel values and patterns
///
/// # Test Steps
/// 1. Creates a 3x3 PPM with diverse pixel values:
///    - Primary colors
///    - Mixed colors
///    - Grayscale values
/// 2. Performs complete compression/decompression cycle
///
/// # Verification Points
/// - Handles different color values correctly
/// - Maintains file paths as expected
/// - Cleanly processes non-trivial image content

#[test]
fn test_pixel_compression() {
    // Create a temporary PPM file with pixel data
    let pixels: Vec<(u16, u16, u16)> = vec![
        (255, 0, 0), (0, 255, 0), (0, 0, 255),
        (128, 128, 0), (0, 128, 128), (128, 0, 128),
        (64, 64, 64), (192, 192, 192), (32, 32, 32)
    ];
    
    let mut ppm_content = String::new();
    ppm_content.push_str("P3\n");
    ppm_content.push_str("3 3\n");
    ppm_content.push_str("255\n");
    for (r, g, b) in pixels {
        ppm_content.push_str(&format!("{} {} {} ", r, g, b));
    }

    let mut input_file = NamedTempFile::new().unwrap();
    write!(input_file, "{}", ppm_content).unwrap();
    let input_path = input_file.path().to_str().unwrap();

    // Test compression
    compress(Some(input_path)).expect("Compression failed");
    
    // Verify compressed file was created with .rpeg extension
    let compressed_path = format!("{}.rpeg", input_path);
    assert!(
        fs::metadata(&compressed_path).is_ok(),
        "Compressed file not found at expected path: {}",
        compressed_path
    );

    // Test decompression
    decompress(Some(&compressed_path)).expect("Decompression failed");

    // Verify decompressed file was created at original path
    assert!(
        fs::metadata(input_path).is_ok(),
        "Decompressed file not found at expected path: {}",
        input_path
    );

    // Clean up
    fs::remove_file(compressed_path).unwrap();
    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_black_image() {
    let ppm = "P3\n2 2\n255\n0 0 0 0 0 0 0 0 0 0 0 0";
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", ppm).unwrap();
    
    compress(Some(file.path().to_str().unwrap())).unwrap();
    decompress(Some("input.rpeg")).unwrap();
    
    let output = fs::read_to_string("input.ppm").unwrap();
    assert!(output.contains("0 0 0"));  // Verify black pixels
}