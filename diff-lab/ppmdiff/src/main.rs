use std::error::Error;
use clap::Parser;
use csc411_image::{Read, RgbImage};
use std::io::{self, Write};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    file1: String,
    #[arg(long)]
    file2: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file1> <file2>", args[0]);
        eprintln!("One of the arguments can be '-' for standard input, but not both.");
        return Ok(());
    }

    let file1 = &args[1];
    let file2 = &args[2];

    // Check if both arguments are '-'
    if file1 == "-" && file2 == "-" {
        eprintln!("Error: Only one argument can be '-' (standard input).");
        return Ok(());
    }

    // Read image1
    let image1 = if file1 == "-" {
        RgbImage::read(None)?
    } else {
        RgbImage::read(Some(file1))?
    };

    // Read image2
    let image2 = if file2 == "-" {
        RgbImage::read(None)?
    } else {
        RgbImage::read(Some(file2))?
    };

    // Check if images are empty
    if image1.pixels.is_empty() || image2.pixels.is_empty() {
        eprintln!("Error: One of the images is empty or failed to load.");
        return Ok(());
    }

    // Check if image dimensions differ by more than 1
    if (image1.width as i32 - image2.width as i32).abs() > 1 || 
       (image1.height as i32 - image2.height as i32).abs() > 1 {
        eprintln!("Image dimensions differ by more than 1");
        println!("1.0");
        return Ok(());
    }

    // Compute the Root Mean Square Difference
    let rmsd = compute_rmsd(&image1, &image2);

    // Print the RMSD with 4 decimal places
    println!("{:.4}", rmsd);
    io::stdout().flush().unwrap();

    Ok(())
}

fn compute_rmsd(image1: &RgbImage, image2: &RgbImage) -> f64 {
    let w = std::cmp::min(image1.width, image2.width) as usize;
    let h = std::cmp::min(image1.height, image2.height) as usize;
    
    let mut sum = 0.0;

    for i in 0..h {
        for j in 0..w {
            let pixel1 = &image1.pixels[i * image1.width as usize + j];
            let pixel2 = &image2.pixels[i * image2.width as usize + j];

            let r_diff = (pixel1.red as f64 - pixel2.red as f64) / 255.0;
            let g_diff = (pixel1.green as f64 - pixel2.green as f64) / 255.0;
            let b_diff = (pixel1.blue as f64 - pixel2.blue as f64) / 255.0;

            sum += r_diff * r_diff + g_diff * g_diff + b_diff * b_diff;
        }
    }

    let denom = (3 * w * h) as f64;
    (sum / denom).sqrt()
}
