use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use clap::Parser;
use csc411_image::{RgbImage, Read, Rgb};
use array2::Array2;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_file: String,  // Input file path
    #[arg(long)]
    output_file: String, // Output file path
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the command line arguments
    let args = Args::parse();

    // Step 1: Read the PPM image
    let image = RgbImage::read(Some(&args.input_file))?;

    // Step 2: Trim the image to even dimensions
    let trimmed_image = trim_to_even_dimensions(image);

    // Step 3: Convert from scaled integers (0-255) to floating-point numbers (0.0-1.0)
    let float_image = convert_to_floats(&trimmed_image);

    // Step 4: Convert back from floating-point numbers (0.0-1.0) to scaled integers (0-255)
    let int_image = convert_to_ints(&float_image);

    // Step 5: Write the transformed image to a PPM file
    write_ppm(&args.output_file, &int_image)?;

    Ok(())
}

// Function to trim the image to even dimensions
fn trim_to_even_dimensions(image: RgbImage) -> RgbImage {
    let new_width = if image.width % 2 == 0 { image.width } else { image.width - 1 };
    let new_height = if image.height % 2 == 0 { image.height } else { image.height - 1 };

    // Create a new vector to hold the trimmed pixels
    let mut trimmed_pixels = Vec::new();

    // Iterate over the image and keep only the pixels within the new dimensions
    for y in 0..new_height {
        for x in 0..new_width {
            let index = (y * image.width + x) as usize;
            trimmed_pixels.push(image.pixels[index].clone());
        }
    }

    RgbImage {
        width: new_width,
        height: new_height,
        pixels: trimmed_pixels,
        denominator: image.denominator,
    }
}

// Function to convert from scaled integers (0-255) to floating-point numbers (0.0-1.0)
fn convert_to_floats(image: &RgbImage) -> Array2<(f64, f64, f64)> {
    let mut float_pixels = Vec::new();

    for pixel in &image.pixels {
        let r = pixel.red as f64 / image.denominator as f64;
        let g = pixel.green as f64 / image.denominator as f64;
        let b = pixel.blue as f64 / image.denominator as f64;
        float_pixels.push((r, g, b));
    }

    Array2::from_row_major(float_pixels, image.width as usize, image.height as usize).unwrap()
}

// Function to convert back from floating-point numbers (0.0-1.0) to scaled integers (0-255)
fn convert_to_ints(image: &Array2<(f64, f64, f64)>) -> RgbImage {
    let mut int_pixels = Vec::new();

    for (_, _, &(r, g, b)) in image.iter_row_major() {
        let red = (r * 255.0).round() as u16;
        let green = (g * 255.0).round() as u16;
        let blue = (b * 255.0).round() as u16;
        int_pixels.push(Rgb { red, green, blue });
    }

    RgbImage {
        width: image.width as u32,
        height: image.height as u32,
        pixels: int_pixels,
        denominator: 255,
    }
}

// Function to write the transformed image to a PPM file
fn write_ppm(output_file: &str, image: &RgbImage) -> Result<(), Box<dyn Error>> {
    // Open the file in write mode
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write the PPM header
    writeln!(writer, "P6")?;
    writeln!(writer, "{} {}", image.width, image.height)?;
    writeln!(writer, "{}", image.denominator)?;

    // Write the pixel data
    for pixel in &image.pixels {
        writer.write_all(&[
            pixel.red.try_into().unwrap(), // Convert u16 to u8
            pixel.green.try_intox().unwrap(), // Convert u16 to u8
            pixel.blue.try_into().unwrap(), // Convert u16 to u8
        ])?;
    }

    Ok(())
}