use csc411_image::{GrayImage, Read};
use std::env;

fn main() { 
    let input = env::args().nth(1); 
    assert!(env::args().len() == 2); 
    let img = GrayImage::read(input.as_deref()).unwrap(); 
    let denom =img.denominator as u32;
    let mut sum = 0;
    for pixel in img.pixels {
        sum += pixel.value as u32;
    }
    // print to three decimals
    println!("{:.3}", sum as f64 / (denom * img.width * img.height) as f64); // f64: 1 signed bit, 11 exponent bits, 52 mantissa bits, can represent larger numbers to greater precision
                                                                            // f32: 1 signed bit, 8 exponent bits, 23 mantissa bits
}


/*
Formatting output:
println!("{:.3}", value); // prints value to 3 decimal places
*/
