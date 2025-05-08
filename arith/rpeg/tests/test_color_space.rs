use rpeg::codec::{compress, decompress};
use std::fs::{self, File};
use std::io::{Write, Read};
use tempfile::NamedTempFile;

// Helper function to create a valid PPM file
fn create_ppm_file(content: &str, dimensions: (usize, usize)) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "P3\n{} {}\n255\n{}", dimensions.0, dimensions.1, content).unwrap();
    file
}

#[test]
fn test_compress_decompress() {
    // Create a valid PPM test image (2x2 pixels)
    let ppm_content = "255 0 0 0 255 0 0 0 255 255 255 255 ";
    let mut ppm_file = create_ppm_file(ppm_content, (2, 2));
    let input_path = ppm_file.path().to_str().unwrap();

    // Test compression
    compress(Some(input_path)).expect("Compression failed");

    // Verify compressed file was created
    let compressed_path = format!("{}.rpeg", input_path);
    assert!(fs::metadata(&compressed_path).is_ok(), "Compressed file not created");

    // Test decompression
    decompress(Some(&compressed_path)).expect("Decompression failed");

    // Verify decompressed output was created
    let decompressed_path = format!("{}.ppm", input_path); // Changed from .decompressed to .ppm
    assert!(fs::metadata(&decompressed_path).is_ok(), "Decompressed file not created");

    // Clean up
    fs::remove_file(compressed_path).unwrap();
    fs::remove_file(decompressed_path).unwrap();
}

#[test]
fn test_compress_empty_ppm() {
    // Create a valid but empty PPM file (header only)
    let ppm_content = "";
    let mut ppm_file = create_ppm_file(ppm_content, (0, 0));
    let input_path = ppm_file.path().to_str().unwrap();

    // Compression should fail for empty images
    let result = compress(Some(input_path));
    assert!(result.is_err(), "Compression should fail for empty PPM");
}

#[test]
fn test_decompress_invalid_data() {
    // Create a file with invalid data
    let mut invalid_file = NamedTempFile::new().unwrap();
    invalid_file.write_all(&[0, 1, 2, 3, 4, 5]).unwrap();
    let invalid_path = invalid_file.path().to_str().unwrap();

    let result = decompress(Some(invalid_path));
    assert!(result.is_err(), "Decompression of invalid data should return an error");
}

#[test]
fn test_pixel_compression_roundtrip() {
    // Create a 3x3 PPM test image
    let ppm_content = "255 0 0 0 255 0 0 0 255 \
                      128 128 0 0 128 128 128 0 128 \
                      64 64 64 192 192 192 32 32 32 ";
    let mut ppm_file = create_ppm_file(ppm_content, (3, 3));
    let input_path = ppm_file.path().to_str().unwrap();

    // Test compression
    compress(Some(input_path)).expect("Compression failed");
    
    // Verify compressed file
    let compressed_path = format!("{}.rpeg", input_path);
    assert!(fs::metadata(&compressed_path).is_ok(), "Compressed file not created");

    // Test decompression
    decompress(Some(&compressed_path)).expect("Decompression failed");

    // Verify decompressed output
    let decompressed_path = format!("{}.ppm", input_path);
    assert!(fs::metadata(&decompressed_path).is_ok(), "Decompressed file not created");

    // Read and verify the decompressed content
    let mut decompressed_file = File::open(&decompressed_path).unwrap();
    let mut decompressed_content = String::new();
    decompressed_file.read_to_string(&mut decompressed_content).unwrap();
    
    // Basic verification - should contain PPM header and pixel data
    assert!(decompressed_content.starts_with("P3"));
    assert!(decompressed_content.contains("255"));
    assert!(decompressed_content.contains("255 0 0")); // First pixel

    // Clean up
    fs::remove_file(compressed_path).unwrap();
    fs::remove_file(decompressed_path).unwrap();
}