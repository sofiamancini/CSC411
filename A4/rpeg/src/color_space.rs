use array2::Array2;
use csc411_image::{Rgb, RgbImage};

/// Represents a pixel in YCbCr color space with floating-point components.
/// 
/// # Fields
/// - `y`: Luma (brightness) component, typically in range [0.0, 1.0]
/// - `pb`: Blue-difference chroma component, typically in range [-0.5, 0.5]
/// - `pr`: Red-difference chroma component, typically in range [-0.5, 0.5]
#[derive(Debug, Clone, Copy)]
pub struct YCbCrPixel {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

/// Provides conversion from RGB to YCbCr color space.
/// 
/// Uses standard conversion formulas for JPEG/MPEG color space conversion.
pub struct RgbToYcbcr;

impl RgbToYcbcr {
    /// Converts a single RGB pixel to YCbCr color space.
    ///
    /// # Arguments
    /// - `pixel`: The RGB pixel to convert
    /// - `denominator`: The maximum value of the RGB components (typically 255)
    ///
    /// # Returns
    /// A `YCbCrPixel` with components:
    /// - `y` clamped to [0.0, 1.0]
    /// - `pb` and `pr` in [-0.5, 0.5] range
    ///
    /// # Notes
    /// - Input RGB values are normalized using the denominator
    /// - Uses standard conversion coefficients:
    ///   - Y = 0.299R + 0.587G + 0.114B
    ///   - Pb = -0.168736R - 0.331264G + 0.5B
    ///   - Pr = 0.5R - 0.418688G - 0.081312B
    pub fn convert_pixel(pixel: &Rgb, denominator: u16) -> YCbCrPixel {
        let scale = denominator as f32;
        let r = (pixel.red as f32 / scale).clamp(0.0, 1.0);
        let g = (pixel.green as f32 / scale).clamp(0.0, 1.0);
        let b = (pixel.blue as f32 / scale).clamp(0.0, 1.0);

        YCbCrPixel {
            y: 0.299 * r + 0.587 * g + 0.114 * b,
            pb: -0.168736 * r - 0.331264 * g + 0.5 * b,
            pr: 0.5 * r - 0.418688 * g - 0.081312 * b,
        }
    }

    /// Converts an entire RGB image to YCbCr color space.
    ///
    /// # Arguments
    /// - `image`: The RgbImage to convert
    ///
    /// # Returns
    /// An `Array2<YCbCrPixel>` with the same dimensions as the input image
    ///
    /// # Panics
    /// - If the resulting array dimensions don't match the image dimensions
    pub fn convert(image: &RgbImage) -> Array2<YCbCrPixel> {
        let mut ycbcr_data = Vec::with_capacity((image.width * image.height) as usize);
        
        for pixel in &image.pixels {
            ycbcr_data.push(Self::convert_pixel(pixel, image.denominator));
        }

        Array2::from_row_major(ycbcr_data, image.width as usize, image.height as usize)
            .expect("Valid dimensions")
    }
}

/// Provides conversion from YCbCr to RGB color space.
/// 
/// Uses standard conversion formulas for JPEG/MPEG color space conversion.
pub struct YcbcrToRgb;

impl YcbcrToRgb {
    /// Converts a single YCbCr pixel to RGB color space.
    ///
    /// # Arguments
    /// - `pixel`: The YCbCr pixel to convert
    /// - `denominator`: The maximum value for output RGB components (typically 255)
    ///
    /// # Returns
    /// An `Rgb` pixel with components clamped to [0, denominator]
    ///
    /// # Notes
    /// - Uses standard conversion coefficients:
    ///   - R = Y + 1.402Pr
    ///   - G = Y - 0.344136Pb - 0.714136Pr
    ///   - B = Y + 1.772Pb
    /// - All components are clamped to valid ranges before conversion
    pub fn convert_pixel(pixel: YCbCrPixel, denominator: u16) -> Rgb {
        let y = pixel.y.clamp(0.0, 1.0);
        let pb = pixel.pb.clamp(-0.5, 0.5);
        let pr = pixel.pr.clamp(-0.5, 0.5);

        let r = (1.0 * y + 0.0 * pb + 1.402 * pr).clamp(0.0, 1.0);
        let g = (y - 0.344136 * pb - 0.714136 * pr).clamp(0.0, 1.0);
        let b = (y + 1.772 * pb).clamp(0.0, 1.0);

        let scale = denominator as f32;
        Rgb {
            red: (r * scale).round().clamp(0.0, scale) as u16,
            green: (g * scale).round().clamp(0.0, scale) as u16,
            blue: (b * scale).round().clamp(0.0, scale) as u16,
        }
    }

    /// Converts an entire YCbCr image to RGB color space.
    ///
    /// # Arguments
    /// - `image`: The YCbCr image to convert
    /// - `denominator`: The maximum value for output RGB components
    ///
    /// # Returns
    /// An `RgbImage` with the same dimensions as the input
    pub fn convert(image: &Array2<YCbCrPixel>, denominator: u16) -> RgbImage {
        let mut pixels = Vec::with_capacity(image.width * image.height);
        
        for (_, _, pixel) in image.iter_row_major() {
            pixels.push(Self::convert_pixel(*pixel, denominator));
        }

        RgbImage {
            pixels,
            width: image.width as u32,
            height: image.height as u32,
            denominator,
        }
    }
}

/// Calculates the average chroma values for a 2x2 block of pixels.
///
/// # Arguments
/// - `block`: A 2x2 array of YCbCr pixels
///
/// # Returns
/// A tuple `(pb_avg, pr_avg)` containing the average chroma values
///
/// # Notes
/// - Used during chroma subsampling in compression
/// - Simply averages the pb and pr components of all four pixels
pub fn average_chroma(block: &[YCbCrPixel; 4]) -> (f32, f32) {
    let mut sum_pb = 0.0;
    let mut sum_pr = 0.0;

    for pixel in block {
        sum_pb += pixel.pb;
        sum_pr += pixel.pr;
    }

    (sum_pb / 4.0, sum_pr / 4.0)
}

/// Trims image dimensions to be even numbers by removing edge pixels if needed.
///
/// # Arguments
/// - `image`: The image to trim
///
/// # Returns
/// A new Array2 with even dimensions
///
/// # Notes
/// - Preserves original image when dimensions are already even
/// - For odd dimensions, removes one row/column from the right/bottom
/// - Handles edge cases where dimensions would become zero
pub fn trim_image(image: Array2<YCbCrPixel>) -> Array2<YCbCrPixel> {
    let width = if image.width % 2 == 0 || image.width == 1 {
        image.width
    } else {
        image.width - 1
    };

    let height = if image.height % 2 == 0 || image.height == 1 {
        image.height
    } else {
        image.height - 1
    };

    let mut data = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            match image.get(x, y) {
                Ok(pixel) => data.push(*pixel),
                Err(e) => {
                    eprintln!("Warning: {}", e);
                    data.push(YCbCrPixel { y: 0.0, pb: 0.0, pr: 0.0 });
                }
            }
        }
    }
    Array2::from_row_major(data, width, height).unwrap()
}

/// Checks if two floating-point numbers are approximately equal.
///
/// # Arguments
/// - `a`: First value to compare
/// - `b`: Second value to compare
///
/// # Returns
/// `true` if the values are within a small epsilon of each other
///
/// # Notes
/// - Uses both absolute and relative comparison
/// - Epsilon value is 1e-5
pub fn approx_equal(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 1e-5;
    (a - b).abs() < EPSILON || (a - b).abs() / (a.abs() + b.abs()) < EPSILON
}