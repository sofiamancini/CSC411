//! Bit-packing and unpacking utilities for 64-bit words.
//!
//! Provides functions for safely packing and unpacking signed and unsigned integers
//! of arbitrary bit widths (1-64 bits) within 64-bit words.

/// Safely performs left shift operation, returning 0 if shift >= 64.
#[inline]
fn safe_shl(value: u64, shift: u64) -> u64 {
    if shift >= 64 { 0 } else { value << shift }
}

/// Safely performs right shift operation, returning 0 if shift >= 64.
#[inline]
fn safe_shr(value: u64, shift: u64) -> u64 {
    if shift >= 64 { 0 } else { value >> shift }
}

#[inline]
#[allow(dead_code)]
fn safe_shra(value: u64, shift: u64) -> u64 {
    if shift >= 63 { 0 } else {
        let sign_bit = (value >> 63) & 1;
        let mask = !0u64 << (63 - shift);
        (value >> shift) | (sign_bit * mask)
    }
}

/// Checks if a signed integer can fit within a specified bit width.
///
/// # Arguments
/// * `n` - The signed integer to check
/// * `width` - The number of bits available (1-64)
///
/// # Returns
/// `true` if the value can be represented in `width` signed bits, `false` otherwise
///
/// # Examples
/// ```
/// use bitpack::fitss;
///
/// assert!(fitss(127, 8));  // Fits in 8 signed bits
/// assert!(!fitss(128, 8)); // Doesn't fit in 8 signed bits
/// ```
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 || width > 64 { return false; }
    if width == 64 { return true; }
    let min = -(safe_shl(1, width - 1) as i64);
    let max = (safe_shl(1, width - 1) as i64) - 1;
    n >= min && n <= max
}

/// Checks if an unsigned integer can fit within a specified bit width.
///
/// # Arguments
/// * `n` - The unsigned integer to check
/// * `width` - The number of bits available (1-64)
///
/// # Returns
/// `true` if the value can be represented in `width` unsigned bits, `false` otherwise
///
/// # Examples
/// ```
/// use bitpack::fitsu;
///
/// assert!(fitsu(255, 8));  // Fits in 8 unsigned bits
/// assert!(!fitsu(256, 8)); // Doesn't fit in 8 unsigned bits
/// ```
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 || width > 64 { return false; }
    n <= safe_shl(1, width).wrapping_sub(1)
}

/// Extracts a signed integer from a packed 64-bit word.
///
/// # Arguments
/// * `word` - The packed 64-bit word
/// * `width` - The number of bits to extract (1-64)
/// * `lsb` - The least significant bit position (0-63)
///
/// # Returns
/// `Some(i64)` containing the extracted value if successful, or `None` if:
/// - `width` is 0 or > 64
/// - `lsb + width` > 64
///
/// # Examples
/// ```
/// use bitpack::gets;
///
/// let word = 0b11110000u64;
/// assert_eq!(gets(word, 4, 4), Some(-1));
/// ```
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    if width == 0 || width > 64 || lsb + width > 64 {
        return Some(0);
    }
    
    let value = safe_shr(word, lsb) & safe_shr(!0u64, 64 - width);
    let sign_bit = safe_shl(1, width - 1);

    if value & sign_bit != 0 {
        let sign_ext = safe_shl(!0u64, width);
        Some((value | sign_ext) as i64)
    } else {
        Some(value as i64)
    }
}

/// Extracts an unsigned integer from a packed 64-bit word.
///
/// # Arguments
/// * `word` - The packed 64-bit word
/// * `width` - The number of bits to extract (1-64)
/// * `lsb` - The least significant bit position (0-63)
///
/// # Returns
/// `Some(u64)` containing the extracted value if successful, or `None` if:
/// - `width` is 0 or > 64
/// - `lsb + width` > 64
///
/// # Examples
/// ```
/// use bitpack::getu;
///
/// let word = 0b11110000u64;
/// assert_eq!(getu(word, 4, 4), Some(0b1111));
/// ```
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    if width == 0 || width > 64 || lsb + width > 64 {
        return Some(0);
    }
    Some(safe_shr(word, lsb) & safe_shr(!0u64, 64 - width))
}

/// Packs an unsigned integer into a 64-bit word.
///
/// # Arguments
/// * `word` - The original 64-bit word
/// * `width` - The number of bits to pack (1-64)
/// * `lsb` - The least significant bit position (0-63)
/// * `value` - The unsigned value to pack
///
/// # Returns
/// `Some(u64)` containing the modified word if successful, or `None` if:
/// - `value` doesn't fit in `width` bits
/// - `width` is 0 or > 64
/// - `lsb + width` > 64
///
/// # Examples
/// ```
/// use bitpack::newu;
///
/// let word = 0u64;
/// assert_eq!(newu(word, 4, 4, 0b1010), Some(0b10100000));
/// ```
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(value, width) || lsb + width > 64 {
        return None;
    }
    
    let mask = if width == 64 {
        !0u64
    } else {
        safe_shl(safe_shl(1, width).wrapping_sub(1), lsb)
    };

    Some((word & !mask) | safe_shl(value, lsb) & mask)
}

/// Packs a signed integer into a 64-bit word.
///
/// # Arguments
/// * `word` - The original 64-bit word
/// * `width` - The number of bits to pack (1-64)
/// * `lsb` - The least significant bit position (0-63)
/// * `value` - The signed value to pack
///
/// # Returns
/// `Some(u64)` containing the modified word if successful, or `None` if:
/// - `value` doesn't fit in `width` bits
/// - `width` is 0 or > 64
/// - `lsb + width` > 64
///
/// # Examples
/// ```
/// use bitpack::news;
///
/// let word = 0u64;
/// assert_eq!(news(word, 4, 4, -1), Some(0b11110000));
/// ```
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width) || lsb + width > 64 {
        return None;
    }
    let mask = if width == 64 {
        !0u64
    } else {
        safe_shl(safe_shl(1, width).wrapping_sub(1), lsb)
    };

    let unsign_val = value as u64;
    Some((word & !mask) | safe_shl(unsign_val, lsb) & mask)
}