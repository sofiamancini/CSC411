/// Represents Discrete Cosine Transform (DCT) coefficients for a 2x2 block.
/// 
/// These coefficients represent the frequency components of the block:
/// - `a`: DC coefficient (average brightness), typically in range [0.0, 1.0]
/// - `b`, `c`, `d`: AC coefficients representing vertical, horizontal, and diagonal frequencies
#[derive(Debug, Clone, Copy)]
pub struct DctCoefficients {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

/// Clamps a value to the specified range.
/// 
/// # Arguments
/// - `value`: The value to clamp
/// - `min`: Minimum bound (inclusive)
/// - `max`: Maximum bound (inclusive)
/// 
/// # Returns
/// - `min` if `value < min`
/// - `max` if `value > max`
/// - `value` otherwise
#[inline]
fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Performs forward Discrete Cosine Transform (DCT) on a 2x2 block of pixels.
/// 
/// # Arguments
/// - `block`: A 2x2 pixel block in row-major order [top-left, top-right, bottom-left, bottom-right]
/// 
/// # Returns
/// `DctCoefficients` containing:
/// - `a`: DC component (average brightness)
/// - `b`: Vertical frequency component
/// - `c`: Horizontal frequency component
/// - `d`: Diagonal frequency component
/// 
/// # Notes
/// All input pixel values should ideally be in range [0.0, 1.0] for proper DCT interpretation.
pub fn dct(block: &[f32; 4]) -> DctCoefficients {
    let y1 = block[0];
    let y2 = block[1];
    let y3 = block[2];
    let y4 = block[3];

    DctCoefficients {
        a: (y4 + y3 + y2 + y1) * 0.25_f32,
        b: (y4 + y3 - y2 - y1) * 0.25_f32,
        c: (y4 - y3 + y2 - y1) * 0.25_f32,
        d: (y4 - y3 - y2 + y1) * 0.25_f32,
    }
}

/// Performs inverse Discrete Cosine Transform (IDCT) with clamped output.
/// 
/// # Arguments
/// - `coeffs`: DCT coefficients to transform back to pixel space
/// 
/// # Returns
/// A 2x2 pixel block in row-major order with all values clamped to [0.0, 1.0]
/// 
/// # Notes
/// The clamping ensures valid pixel values but may cause some loss of precision
/// compared to the original input before forward DCT.
pub fn inverse_dct(coeffs: &DctCoefficients) -> [f32; 4] {
    let DctCoefficients { a, b, c, d } = *coeffs;
    [
        clamp(a - b - c + d, 0.0, 1.0), // top-left
        clamp(a - b + c - d, 0.0, 1.0), // top-right
        clamp(a + b - c - d, 0.0, 1.0), // bottom-left
        clamp(a + b + c + d, 0.0, 1.0), // bottom-right
    ]
}

/// Returns the block size used in DCT calculations.
/// 
/// # Notes
/// This is equivalent to 512 (2^9), which is used for scaling DC coefficients
/// during quantization/dequantization.
const fn block_size() -> f32 {
    (1 << 9) as f32
}

/// Quantizes DCT coefficients for compact storage.
/// 
/// # Arguments
/// - `coeffs`: DCT coefficients to quantize
/// 
/// # Returns
/// A tuple of quantized coefficients:
/// - First element: DC coefficient as 9-bit unsigned (0 - block_size())
/// - Remaining elements: AC coefficients as 5-bit signed (-15 to 15)
/// 
/// # Notes
/// - DC coefficient (`a`) is clamped to [0.0, 1.0] before quantization
/// - AC coefficients (`b`, `c`, `d`) are clamped to [-0.3, 0.3] before quantization
/// - Uses proper rounding during quantization
pub fn quantize(coeffs: &DctCoefficients) -> (u16, i16, i16, i16) {
    let a_quantized = (coeffs.a.clamp(0.0, 1.0) * (block_size()-1.0)).round() as u16;

    let quantize_component = |x: f32| {
        let clamped = x.clamp(-0.3, 0.3);
        (clamped * (15.0 / 0.3)).round() as i16
    };

    (
        a_quantized,
        quantize_component(coeffs.b),
        quantize_component(coeffs.c),
        quantize_component(coeffs.d),
    )
}


/// Dequantizes DCT coefficients from a compressed representation.  
///  
/// This function reverses a quantization process, converting packed integer values  
/// back into floating-point coefficients for further processing in a DCT-based  
/// compression pipeline.  
///  
/// # Arguments  
/// - `a`: A unsigned 16-bit value representing the DC coefficient (scaled by 511).  
/// - `b`, `c`, `d`: Signed 16-bit values representing AC coefficients (scaled by 15).  
///  
/// # Returns  
/// A `DctCoefficients` struct with floating-point values:  
/// - `a`: DC coefficient, scaled back to the range `[0.0, 1.0]`.  
/// - `b`, `c`, `d`: AC coefficients, scaled by `0.3 / 15.0` (a normalization factor).  
///  
/// # Notes  
/// - The DC coefficient (`a`) is scaled by `511.0` and then normalized by `block_size()`,  
///   which ensures it stays within the expected range.  
/// - The AC coefficients (`b`, `c`, `d`) are scaled by a fixed factor (`0.3 / 15.0`),  
///   typical in some DCT quantization schemes. 
pub fn dequantize(a: u16, b: i16, c: i16, d: i16) -> DctCoefficients {
    DctCoefficients {
        a: (a as f32 / 511.0 * block_size()) / block_size(),
        b: b as f32 * (0.3 / 15.0),
        c: c as f32 * (0.3 / 15.0),
        d: d as f32 * (0.3 / 15.0),
    }
}