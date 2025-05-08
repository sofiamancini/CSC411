// //! Comprehensive test suite for bitpack operations.
// //!
// //! Tests cover:
// //! - Basic value fitting checks
// //! - Roundtrip packing/unpacking
// //! - Edge cases and error conditions
// //! - Randomized fuzz testing

// use bitpack::{fitss, fitsu, gets, getu, news, newu};
// use rand::Rng;

// /// Tests for the `fitss` (signed value fitting check) function
// #[test]
// fn test_fitss() {
//     // Test signed fitting
//     assert!(fitss(0, 1));      // 0 fits in 1 bit
//     assert!(!fitss(1, 1));     // 1 doesn't fit in 1 bit (range is -1 to 0)
//     assert!(fitss(-1, 1));     // -1 fits in 1 bit
//     assert!(fitss(127, 8));    // 127 fits in 8 bits
//     assert!(!fitss(128, 8));   // 128 doesn't fit
//     assert!(fitss(-128, 8));   // -128 fits
//     assert!(!fitss(-129, 8));  // -129 doesn't fit
// }

// /// Tests for the `fitsu` (unsigned value fitting check) function
// #[test]
// fn test_fitsu() {
//     // Test unsigned fitting
//     assert!(fitsu(0, 1));      // 0 fits in 1 bit
//     assert!(fitsu(1, 1));      // 1 fits in 1 bit
//     assert!(!fitsu(2, 1));     // 2 doesn't fit
//     assert!(fitsu(255, 8));    // 255 fits in 8 bits
//     assert!(!fitsu(256, 8));   // 256 doesn't fit
// }

// /// Tests roundtrip packing/unpacking of unsigned values
// #[test]
// fn test_roundtrip_unsigned() {
//     // Test newu/getu roundtrips
//     let mut word = 0;
//     word = newu(word, 4, 8, 0b1010).unwrap();
//     assert_eq!(getu(word, 4, 8).unwrap(), 0b1010);

//     word = newu(word, 8, 16, 0b11001100).unwrap();
//     assert_eq!(getu(word, 8, 16).unwrap(), 0b11001100);
// }

// /// Tests roundtrip packing/unpacking of signed values
// #[test]
// fn test_roundtrip_signed() {
//     // Test news/gets roundtrips
//     let mut word = 0;
//     word = news(word, 5, 8, -3).unwrap();
//     assert_eq!(gets(word, 5, 8).unwrap(), -3);

//     word = news(word, 8, 16, -42).unwrap();
//     assert_eq!(gets(word, 8, 16).unwrap(), -42);
// }

// /// Tests invalid input handling and edge cases
// #[test]
// fn test_invalid_ranges() {
//     // Test edge cases
//     assert!(getu(0, 0, 0).is_none());     // 0 width
//     assert!(getu(0, 65, 0).is_none());    // width > 64
//     assert!(getu(0, 64, 1).is_none());    // lsb + width > 64
//     assert!(newu(0, 4, 61, 0b1010).is_none()); // would overflow
// }

// /// Randomized fuzz testing for unsigned packing/unpacking
// #[test]
// fn test_random_roundtrips() {
//     // Fuzz testing
//     let mut rng = rand::thread_rng();
//     for _ in 0..1000 {
//         let width = rng.gen_range(1..=64);
//         let lsb = rng.gen_range(0..=64 - width);
//         let value = rng.gen::<u64>() >> (64 - width);
        
//         let packed = newu(0, width, lsb, value).unwrap();
//         let unpacked = getu(packed, width, lsb).unwrap();
//         assert_eq!(value, unpacked);
//     }
// }

// /// Tests boundary conditions and extreme values
// #[test]
// fn test_edge_cases() {
//     // Test full-width values
//     assert_eq!(newu(0, 64, 0, u64::MAX).unwrap(), u64::MAX);
//     assert_eq!(getu(u64::MAX, 64, 0).unwrap(), u64::MAX);
    
//     // Test boundary shifts
//     assert_eq!(newu(0, 1, 63, 1).unwrap(), 1 << 63);
//     let word = news(0, 1, 63, -1).unwrap();
//     assert_eq!(gets(word, 1, 63).unwrap(), -1);
    
//     // Test maximum values
//     assert!(fitss(i64::MIN, 64));
//     assert!(fitss(i64::MAX, 64));
//     assert!(fitsu(u64::MAX, 64));
// }