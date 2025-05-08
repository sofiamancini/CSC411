
use std::error::Error;
use std::io::{self, Write as IoWrite};
use std::time::Instant;
use clap::{Parser, ValueEnum};
use csc411_image::{Read, Rgb, RgbImage};
use array2::Array2;

// Use clap to define command line arguments
#[derive(Parser, Debug)]
#[clap(name = "ppmtrans", author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'r', long = "rotate", value_enum)]
    rotate: Option<Rotation>,

    #[clap(short = 'f', long = "flip", value_enum)]
    flip: Option<Flip>,

    #[clap(short = 't', long = "transpose")]
    transpose: bool,

    #[clap(long = "row-major")]
    row_major: bool,

    #[clap(long = "col-major")]
    col_major: bool,
}

// Define the possible values for the rotation options
#[derive(Clone, Debug, ValueEnum)]
enum Rotation {
    #[clap(alias = "90")]
    Rotate90,
    #[clap(alias = "180")]
    Rotate180,
    #[clap(alias = "270")]
    Rotate270,
    #[clap(alias = "0")]
    Rotate0,
}

// Define the possible values for the flip options
#[derive(Clone, Debug, ValueEnum)]
enum Flip {
    Horizontal,
    Vertical,
}

// Transform the image according to the specified option
fn transform_image(
    image: Array2<Rgb>,
    width: usize,
    height: usize,
    rotation: Option<Rotation>,
    flip: Option<Flip>,
    transpose: bool,
    use_row_major: bool,
) -> Result<(Array2<Rgb>, usize, usize), Box<dyn Error>> {
    // Calculate the new width and height of the image after rotation
    let (new_width, new_height) = match rotation {
        Some(Rotation::Rotate90) | Some(Rotation::Rotate270) => (height, width), // Swap dimensions for 90 and 270 degree rotations
        _ => (width, height), // Keep dimensions the same otherwise
    };

    let mut new_image = Array2::fill(Rgb { red: 0, green: 0, blue: 0 }, new_width, new_height);

    // Start timing the transformation
    let now = Instant::now();

    // Apply the transformation based on the specified traversal order
    if use_row_major {
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get(x, y)?;
                let (new_x, new_y) = map_coords(x, y, width, height, &rotation, &flip, transpose);
                new_image.set(new_x, new_y, pixel.clone())?;
            }
        }
    } else {
        for x in 0..width {
            for y in 0..height {
                let pixel = image.get(x, y)?;
                let (new_x, new_y) = map_coords(x, y, width, height, &rotation, &flip, transpose);
                new_image.set(new_x, new_y, pixel.clone())?;
            }
        }
    }

    let elapsed = now.elapsed();
    eprintln!("Elapsed: {:?}", elapsed);

    Ok((new_image, new_width, new_height))
}

// Map the coordinates of a pixel in the input image to the coordinates of the pixel in the output image
fn map_coords(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    rotation: &Option<Rotation>,
    flip: &Option<Flip>,
    transpose: bool,
) -> (usize, usize) {
    let (mut new_x, mut new_y) = (x, y);

    // Apply rotation
    if let Some(rotation) = rotation {
        match rotation {
            Rotation::Rotate90 => (new_x, new_y) = (height - y - 1, x),
            Rotation::Rotate180 => (new_x, new_y) = (width - x - 1, height - y - 1),
            Rotation::Rotate270 => (new_x, new_y) = (height - y - 1, x),
            Rotation::Rotate0 => {}
        }
    }

    // Apply flip
    if let Some(flip) = flip {
        match flip {
            Flip::Horizontal => new_x = width - new_x - 1,
            Flip::Vertical => new_y = height - new_y - 1,
        }
    }

    // Apply transpose
    if transpose {
        (new_x, new_y) = (new_y, new_x);
    }

    (new_x, new_y)
}

// Custom function to write PPM in P6 format
fn write_ppm_p6(output_img: &RgbImage) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    // Write the PPM header
    writeln!(stdout, "P6")?; // Magic number for binary PPM
    writeln!(stdout, "{} {}", output_img.width, output_img.height)?; // Width and height
    writeln!(stdout, "{}", output_img.denominator)?; // Maximum pixel value (255) on a new line

    // Write the pixel data in binary format
    for pixel in &output_img.pixels {
        let rgb = [pixel.red as u8, pixel.green as u8, pixel.blue as u8];
        stdout.write_all(&rgb)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    if args.row_major && args.col_major {
        eprintln!("Cannot specify both row-major and col-major");
        return Err("Cannot specify both row-major and col-major".into());
    }

    // Read image from stdin
    let img = RgbImage::read(None)?;

    let rotation = args.rotate;
    let flip = args.flip;
    let transpose = args.transpose;
    let use_row_major = args.row_major;

    if rotation.is_none() && flip.is_none() && !transpose {
        eprintln!("No operation specified");
        return Err("No operation specified".into());
    }

    let width = img.width as usize;
    let height = img.height as usize;

    if width == 0 || height == 0 {
        eprintln!("Invalid image dimensions: {} x {}", width, height);
        return Err("Invalid image dimensions".into());
    }

    // Convert image pixels to Array2 for easier manipulation
    let array2_img = match Array2::from_row_major(img.pixels.clone(), width, height) {
        Ok(array2_img) => array2_img,
        Err(e) => {
            eprintln!("Error converting image to Array2: {}", e);
            return Err("Error converting image to Array2".into());
        }
    };

    // Transform the image
    let (new_array2_img, new_width, new_height) =
        transform_image(array2_img, width, height, rotation, flip, transpose, use_row_major)?;

    // Convert the transformed Array2 back into a Vec<Rgb> for RgbImage
    let transformed_pixels = new_array2_img.iter_row_major()
        .map(|(_, _, pixel)| pixel.clone()) // Clone the pixel
        .collect::<Vec<_>>();

    // Create a new RgbImage with the transformed pixels
    let output_img = RgbImage {
        pixels: transformed_pixels,
        width: new_width as u32,
        height: new_height as u32,
        denominator: img.denominator,
    };

    // Write the transformed image to stdout using the custom function
    write_ppm_p6(&output_img)?;

    eprintln!("Image processed successfully with width={} and height={}", new_width, new_height);

    Ok(())
}