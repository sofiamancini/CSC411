//! A 2D array implementation with flexible storage and iteration patterns.
//!
//! This crate provides a generic 2D array structure [`Array2<T>`] with:
//! - Row-major and column-major construction
//! - Bounds-checked element access
//! - Flexible iteration patterns
//! - Mutable and immutable views
//!
//! # Examples
//! ```
//! use array2::Array2;
//!
//! // Create a 2x2 array from row-major data
//! let arr = Array2::from_row_major(vec![1, 2, 3, 4], 2, 2).unwrap();
//! assert_eq!(*arr.get(1, 0).unwrap(), 2);
//!
//! // Iterate in row-major order
//! for (x, y, val) in arr.iter_row_major() {
//!     println!("({}, {}): {}", x, y, val);
//! }
//! ```

mod array2;

/// Re-export of the main [`Array2`] type for convenient access
pub use array2::Array2;