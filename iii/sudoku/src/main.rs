use std::env;
use std::process;
use csc411_image::{GrayImage, Read};
use array2::Array2;

fn is_valid_sudoku(image: &GrayImage) -> bool {
    // Invariant: The image dimensions must be 9x9
    // This ensures that the image represents a valid sudoku puzzle
    if image.width != 9 || image.height != 9 {
        eprintln!("Image dimensions incorrect: expected 9x9, but got {}x{}", image.width, image.height);
        return false;
    }
    // Invariant: The image denominator must be 9
    if image.denominator != 9 {
        eprintln!("Image denominator incorrect: expected 9, but got {}", image.denominator);
        return false;
    }
    // Convert the image pixels to a vector so Array2 can process the values
    // Invariant: the 'pixels' vector must contain 81 elements
    let pixels: Vec<u16> = image.pixels.iter().map(|p| p.value).collect();
    let grid = match Array2::from_row_major(pixels, 9, 9) {
        // Invariant: The array2::from_row_major function must return a valid 2D array
        Ok(grid) => grid,
        Err(e) => {
            eprintln!("Error creating grid: {}", e);
            process::exit(1);
        }
    };
    for y in 0..9 {
        // Invariant: Each row must contain unique values from 1 to 9
        let mut seen = [false; 10];
        for x in 0..9 {
            let value = match grid.get(x, y) {
                Ok(&value) => value as usize,
                Err(e) => {
                    eprintln!("Error getting value at ({}, {}): {}", x, y, e);
                    process::exit(1);
                }
            };
            // Invariant: The value must be between 1 and 9, and have not been seen before
            if value == 0 || seen[value] {
                eprintln!("Invalid value at ({}, {}): {}", x, y, value);
                return false;
            }
            seen[value] = true;
        }
    }

    for x in 0..9 {
        // Invariant: Each column must contain unique values from 1 to 9
        let mut seen = [false; 10];
        for y in 0..9 {
            let value = *grid.get(x, y).unwrap() as usize;
            if value == 0 || seen[value] {
                // Invariant: The value must be between 1 and 9, and have not been seen before
                eprintln!("Invalid value at ({}, {}): {}", x, y, value);
                return false;
            }
            seen[value] = true;
        }
    }

    for sub_x in 0..3 {
        // Invariant: Each 3x3 subgrid must contain unique values from 1 to 9
        for sub_y in 0..3 {
            let mut seen = [false; 10];
            for dx in 0..3 {
                for dy in 0..3 {
                    let x = sub_x * 3 + dx;
                    let y = sub_y * 3 + dy;
                    let value = *grid.get(x, y).unwrap() as usize;
                    if value == 0 || seen[value] {
                        // Invariant: The value must be between 1 and 9, and have not been seen before
                        eprintln!("Invalid value at ({}, {}): {}", x, y, value);
                        return false;
                    }
                    seen[value] = true;
                }
            }
        }
    }

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).map(String::as_str);

    let image_result = GrayImage::read(filename);

    match image_result {
        Ok(image) => {
            if is_valid_sudoku(&image) {
                // println!("valid");
                process::exit(0);
            } else {
                // eprintln!("invalid.");
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error reading image: {}", e);
            process::exit(1);
        }
    }
}
