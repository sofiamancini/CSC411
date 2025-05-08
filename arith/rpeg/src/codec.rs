use crate::dct::{dct, quantize};
use crate::color_space::{RgbToYcbcr, YcbcrToRgb, YCbCrPixel, average_chroma};
use bitpack::bitpack::{newu, news, gets, getu};
use csc411_rpegio::input_rpeg_data;
use csc411_arith::{index_of_chroma, chroma_of_index};
use csc411_image::{RgbImage, Read as ImageRead};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;

/// Compresses a PPM image to RPEG format.
///
/// # Arguments
/// * `filename` - An optional path to the input PPM file. If `None`, reads from stdin.
///
/// # Returns
/// * `Ok(())` if compression succeeds
/// * `Err` containing an error message if compression fails
///
/// # Errors
/// Returns an error if:
/// * The input file doesn't exist or can't be read
/// * The image dimensions are invalid (0 or odd after adjustment)
/// * Any compression step fails
///
/// # Behavior
/// 1. Reads PPM image from file or stdin
/// 2. Converts RGB to YCbCr color space
/// 3. Processes image in 2x2 blocks:
///    - Computes average chroma values
///    - Applies DCT to luma components
///    - Quantizes coefficients
///    - Packs data into 32-bit words
/// 4. Writes compressed data to output file or stdout
///
/// # Notes
/// - Input dimensions are adjusted to even numbers if needed
/// - Output will be written to `{input}.rpeg` if input is a file
/// - Uses 9 bits for DC coefficient, 5 bits for AC coefficients
/// - Chroma components are quantized to 4 bits each

pub fn compress(filename: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let img = match RgbImage::read(filename) {
        Ok(img) => img,
        Err(e) if filename.is_some() => {
            let path = Path::new(filename.unwrap());
            if !path.exists() {
                return Err("File not found".into());
            }
            return Err(e.into());
        }
        Err(e) => return Err(e.into()),
    };

    let mut width = img.width as usize;
    let mut height = img.height as usize;

    if width % 2 != 0 { width -= 1; }
    if height % 2 != 0 { height -= 1; }

    if width < 2 || height < 2 {
        return Err("Image too small after trimming".into());
    }

    let ycbcr_image = RgbToYcbcr::convert(&img);
    let mut output_data = Vec::with_capacity((width * height) / 4);

    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            let mut y_values = [0.0; 4];
            let mut block = [YCbCrPixel { y: 0.0, pb: 0.0, pr: 0.0 }; 4];
            
            for dy in 0..2 {
                for dx in 0..2 {
                    let pixel = ycbcr_image.get(x + dx, y + dy).unwrap();
                    block[dy * 2 + dx] = *pixel;
                    y_values[dy * 2 + dx] = pixel.y;
                }
            }

            let (pb_avg, pr_avg) = average_chroma(&block);
            let pb_index = index_of_chroma(pb_avg);
            let pr_index = index_of_chroma(pr_avg);

            let coeffs = dct(&y_values);
            let (a, b, c, d) = quantize(&coeffs);

            let mut codeword = 0u64;
            
            codeword = newu(codeword, 9, 23, a as u64).ok_or("Failed to pack 'a'")?;
            codeword = news(codeword, 5, 18, b as i64).ok_or("Failed to pack 'b'")?;
            codeword = news(codeword, 5, 13, c as i64).ok_or("Failed to pack 'c'")?;
            codeword = news(codeword, 5, 8, d as i64).ok_or("Failed to pack 'd'")?;
            codeword = newu(codeword, 4, 4, pb_index as u64).ok_or("Failed to pack PB")?;
            codeword = newu(codeword, 4, 0, pr_index as u64).ok_or("Failed to pack PR")?;

            output_data.push((codeword as u32).to_be_bytes());
        }
    }

    if let Some(input_path) = filename {
        let output_path = Path::new(input_path)
            .with_extension("rpeg")
            .to_string_lossy()
            .into_owned();
        
        let mut output_file = File::create(&output_path)?;
        
        writeln!(output_file, "Compressed image format 2")?;
        writeln!(output_file, "{} {}", width, height)?;
        
        output_file.write_all(&output_data.concat())?;
        output_file.flush()?;
    } else {
        println!("Compressed image format 2\n{} {}", width, height);
        io::stdout().write_all(&output_data.concat())?;
    }

    Ok(())
}

/// Decompresses an RPEG file to PPM format.
///
/// # Arguments
/// * `filename` - An optional path to the input RPEG file. If `None`, reads from stdin.
///
/// # Returns
/// * `Ok(())` if decompression succeeds
/// * `Err` containing an error message if decompression fails
///
/// # Errors
/// Returns an error if:
/// * The input file doesn't exist or can't be read
/// * The header format is invalid
/// * Any decompression step fails
///
/// # Behavior
/// 1. Reads compressed data from file or stdin
/// 2. Unpacks 32-bit codewords into components
/// 3. Reconstructs YCbCr values:
///    - Dequantizes DCT coefficients
///    - Applies inverse DCT
///    - Reconstructs chroma values
/// 4. Converts YCbCr back to RGB
/// 5. Writes PPM image to output file or stdout
///
/// # Notes
/// - When reading from file, output will be written to `{input}` without `.rpeg` extension
/// - Maintains original image dimensions
/// - Output PPM uses 255 as max color value

pub fn decompress(filename: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let (input_data, width, height) = if let Some(f) = filename {
        let path = Path::new(f);
        if !path.exists() {
            return Err("File not found".into());
        }
        input_rpeg_data(Some(f))?
    } else {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        
        let mut header = String::new();
        io::stdin().read_line(&mut header)?;
        let parts: Vec<&str> = header.trim().split_whitespace().collect();
        
        if parts.len() < 6 || parts[0] != "Compressed" || parts[1] != "image" || 
           parts[2] != "format" || parts[3] != "2" {
            return Err("Invalid header format".into());
        }
        
        let width = parts[4].parse()?;
        let height = parts[5].parse()?;
        
        let data = buffer.chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
            .collect();
        
        (data, width, height)
    };

    let mut pixels = vec![(0u16, 0u16, 0u16); width * height];

    for (i, chunk) in input_data.iter().enumerate() {
        let y = (i / (width / 2)) * 2;
        let x = (i % (width / 2)) * 2;

        let codeword = u32::from_be_bytes(*chunk) as u64;

        let a = getu(codeword, 9, 23).ok_or("Failed to unpack 'a'")? as u16;
        let b = gets(codeword, 5, 18).ok_or("Failed to unpack 'b'")? as i16;
        let c = gets(codeword, 5, 13).ok_or("Failed to unpack 'c'")? as i16;
        let d = gets(codeword, 5, 8).ok_or("Failed to unpack 'd'")? as i16;
        let pb_index = getu(codeword, 4, 4).ok_or("Failed to unpack PB")? as usize;
        let pr_index = getu(codeword, 4, 0).ok_or("Failed to unpack PR")? as usize;

        let pb = chroma_of_index(pb_index);
        let pr = chroma_of_index(pr_index);

        let coeffs = crate::dct::dequantize(a, b, c, d);
        let y_values = crate::dct::inverse_dct(&coeffs);

        for dy in 0..2 {
            for dx in 0..2 {
                if y + dy < height && x + dx < width {
                    let idx = (y + dy) * width + (x + dx);
                    let y_val = y_values[dy * 2 + dx];
                    let rgb = YcbcrToRgb::convert_pixel(YCbCrPixel { y: y_val, pb, pr }, 255);
                    pixels[idx] = (rgb.red, rgb.green, rgb.blue);
                }
            }
        }
    }

    let output_pixels: Vec<[u8; 3]> = pixels.iter()
        .map(|&(r, g, b)| [r as u8, g as u8, b as u8])
        .collect();

    if filename.is_none() {
        println!("P3\n{} {}\n255", width, height);
        for chunk in output_pixels.chunks(5) {
            for pixel in chunk {
                print!("{} {} {} ", pixel[0], pixel[1], pixel[2]);
            }
            println!();
        }
    } else {
        let output_path = Path::new(filename.unwrap())
            .with_extension("ppm")
            .to_string_lossy()
            .into_owned();
            
        let mut output_file = File::create(&output_path)?;
        writeln!(output_file, "P3\n{} {}\n255", width, height)?;
        
        for chunk in output_pixels.chunks(5) {
            for pixel in chunk {
                write!(output_file, "{} {} {} ", pixel[0], pixel[1], pixel[2])?;
            }
            writeln!(output_file)?;
        }
        output_file.flush()?;
    }

    Ok(())
}