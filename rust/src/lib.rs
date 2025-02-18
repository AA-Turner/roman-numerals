//! # roman-numerals
//!
//! A library for manipulating well-formed Roman numerals.
//!
//! Integers between 1 and 3,999 (inclusive) are supported.
//! Numbers beyond this range will return an ``OutOfRangeError``.
//!
//! The classical system of roman numerals requires that
//! the same character may not appear more than thrice consecutively,
//! meaning that 'MMMCMXCIX' (3,999) is the largest well-formed Roman numeral.
//! The smallest is 'I' (1), as there is no symbol for zero in Roman numerals.
//!
//! Both upper- and lower-case formatting of roman numerals are supported,
//! and likewise for parsing strings,
//! although the entire string must be of the same case.
//! Numerals that do not adhere to the classical form are rejected
//! with an ``InvalidRomanNumeralError``.
//!
//! ## Example usage
//!
//! ### Create a roman numeral
//!
//! ```rust
//! use roman_numerals_rs::RomanNumeral;
//!
//! let num = RomanNumeral::new(16).unwrap();
//! assert_eq!(num.to_string(), "XVI");
//!
//! let num: RomanNumeral = "XVI".parse().unwrap();
//! assert_eq!(num.as_u16(), 16);
//!
//! let num: RomanNumeral = 3_999.try_into().unwrap();
//! println!("{}", num);  // MMMCMXCIX
//! ```
//!
//! ### Convert a roman numeral to a string
//!
//! ```rust
//! use roman_numerals_rs::RomanNumeral;
//!
//! let num = RomanNumeral::new(16).unwrap();
//! assert_eq!(num.to_string(), "XVI");
//! assert_eq!(num.to_uppercase(), "XVI");
//! assert_eq!(num.to_lowercase(), "xvi");
//! assert_eq!(format!("{:X}", num), "XVI");
//! assert_eq!(format!("{:x}", num), "xvi");
//! ```
//!
//! ### Extract the decimal value of a roman numeral
//!
//! ```rust
//! use roman_numerals_rs::RomanNumeral;
//!
//! let num = RomanNumeral::new(42).unwrap();
//! assert_eq!(num.as_u16(), 42);
//! ```
//!
//! ### Invalid input
//!
//! ```rust
//! use core::str::FromStr;
//! use roman_numerals_rs::{RomanNumeral, InvalidRomanNumeralError, OutOfRangeError};
//!
//! let res = RomanNumeral::from_str("Spam!");
//! assert!(matches!(res.unwrap_err(), InvalidRomanNumeralError));
//!
//! let res = "CLL".parse::<RomanNumeral>();
//! assert!(matches!(res.unwrap_err(), InvalidRomanNumeralError));
//!
//! let res = RomanNumeral::new(0);
//! assert!(matches!(res.unwrap_err(), OutOfRangeError));
//!
//! let res = RomanNumeral::new(4_000);
//! assert!(matches!(res.unwrap_err(), OutOfRangeError));
//! ```
//!
//! ## Licence
//!
//! This project is licenced under the terms of either
//! the Zero-Clause BSD licence or the CC0 1.0 Universal licence.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use core::fmt;
use core::num::NonZero;
use core::str::FromStr;

/// The value of the smallest well-formed roman numeral.
pub const MIN: u16 = 1;
/// The value of the largest well-formed roman numeral.
pub const MAX: u16 = 3_999;

/// Returned as an error if a numeral is constructed with an invalid input.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number out of range (must be between 1 and 3,999).")
    }
}

/// Returned as an error if a parsed string is not a roman numeral.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub struct InvalidRomanNumeralError;

impl fmt::Display for InvalidRomanNumeralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Roman numeral.")
    }
}

/// A Roman numeral.
///
/// Only values between 1 and 3,999 are valid.
/// Stores the value internally as a ``NonZero<u16>``.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RomanNumeral(NonZero<u16>);

impl RomanNumeral {
    /// Creates a ``RomanNumeral`` for any value that implements.
    /// Requires ``value`` to be greater than 0 and less than 4,000.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    //     let answer: RomanNumeral = RomanNumeral::new(42).unwrap();
    //     assert_eq!("XLII", answer.to_uppercase());
    ///
    pub const fn new(value: u16) -> Result<Self, OutOfRangeError> {
        if 0 != value && value < 4_000 {
            // SAFETY: 0 < value <= 3,999
            Ok(Self(unsafe { NonZero::new_unchecked(value) }))
        } else {
            Err(OutOfRangeError)
        }
    }

    /// Return the value of this ``RomanNumeral`` as an ``u16``.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    assert_eq!(answer.as_u16(), 42_u16);
    ///
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0.get()
    }

    /// Converts a ``RomanNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    assert_eq!("XLII", answer.to_uppercase());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_uppercase(self) -> String {
        let mut out = String::new();
        let mut n = self.0.get();
        for &(value, name, _) in ROMAN_NUMERAL_PREFIXES {
            while n >= value {
                n -= value;
                out.push_str(name);
            }
        }
        out
    }

    /// Converts a ``RomanNumeral`` to a lowercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    assert_eq!("xlii", answer.to_lowercase());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_lowercase(self) -> String {
        let mut out = String::new();
        let mut n = self.0.get();
        for &(value, _, name) in ROMAN_NUMERAL_PREFIXES {
            while n >= value {
                n -= value;
                out.push_str(name);
            }
        }
        out
    }
}

#[cfg(feature = "std")]
impl fmt::Display for RomanNumeral {
    /// Converts a ``RomanNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    assert_eq!("XLII", answer.to_string());
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_uppercase())
    }
}

#[cfg(feature = "std")]
impl fmt::UpperHex for RomanNumeral {
    /// Converts a ``RomanNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    println!("{:X}", answer);  // XLII
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_uppercase())
    }
}

#[cfg(feature = "std")]
impl fmt::LowerHex for RomanNumeral {
    /// Converts a ``RomanNumeral`` to a lowercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = RomanNumeral::new(42)?;
    ///    println!("{:x}", answer);  // xlii
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_lowercase())
    }
}

const PREFIXES_BYTES: [u8; 7] = [b'I', b'V', b'X', b'L', b'C', b'D', b'M'];

impl FromStr for RomanNumeral {
    type Err = InvalidRomanNumeralError;

    /// Creates a ``RomanNumeral`` from a well-formed string
    /// representation of the roman numeral.
    ///
    /// Returns ``RomanNumeral`` or ``InvalidRomanNumeralError``.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: RomanNumeral = "XLII".parse()?;
    ///    assert_eq!(42, answer.0);
    ///
    #[allow(clippy::too_many_lines)]
    fn from_str(s: &str) -> Result<Self, InvalidRomanNumeralError> {
        if s.is_empty() {
            return Err(InvalidRomanNumeralError);
        }

        // ASCII-only uppercase string.
        let chars = if s.chars().all(|c: char| char::is_ascii_uppercase(&c)) {
            s.as_bytes()
        } else if s.chars().all(|c: char| char::is_ascii_lowercase(&c)) {
            &s.as_bytes().to_ascii_uppercase()
        } else {
            // Either Non-ASCII or mixed-case ASCII.
            return Err(InvalidRomanNumeralError);
        };

        // ASCII-only uppercase string only containing I, V, X, L, C, D, M.
        if chars.iter().any(|c| !PREFIXES_BYTES.contains(c)) {
            return Err(InvalidRomanNumeralError);
        }

        let mut result: u16 = 0;
        let mut idx: usize = 0;

        // Thousands: between 0 and 4 "M" characters at the start
        for _ in 0..4 {
            let Some(x) = chars.get(idx..=idx) else {
                break;
            };
            if x == b"M" {
                result += 1_000;
                idx += 1;
            } else {
                break;
            }
        }
        if chars.len() == idx {
            // SAFETY: idx is only incremented after adding to result,
            //         and chars is not empty, hence ``idx > 1``.
            return Ok(Self(unsafe { NonZero::new_unchecked(result) }));
        }

        // Hundreds: 900 ("CM"), 400 ("CD"), 0-300 (0 to 3 "C" characters),
        // or 500-800 ("D", followed by 0 to 3 "C" characters)
        if chars[idx..].starts_with(b"CM") {
            result += 900;
            idx += 2;
        } else if chars[idx..].starts_with(b"CD") {
            result += 400;
            idx += 2;
        } else {
            if chars.get(idx..=idx).unwrap_or_default() == b"D" {
                result += 500;
                idx += 1;
            }
            for _ in 0..3 {
                let Some(x) = chars.get(idx..=idx) else {
                    break;
                };
                if x == b"C" {
                    result += 100;
                    idx += 1;
                } else {
                    break;
                }
            }
        }
        if chars.len() == idx {
            // SAFETY: idx is only incremented after adding to result,
            //         and chars is not empty, hence ``idx > 1``.
            return Ok(Self(unsafe { NonZero::new_unchecked(result) }));
        }

        // Tens: 90 ("XC"), 40 ("XL"), 0-30 (0 to 3 "X" characters),
        // or 50-80 ("L", followed by 0 to 3 "X" characters)
        if chars[idx..].starts_with(b"XC") {
            result += 90;
            idx += 2;
        } else if chars[idx..].starts_with(b"XL") {
            result += 40;
            idx += 2;
        } else {
            if chars.get(idx..=idx).unwrap_or_default() == b"L" {
                result += 50;
                idx += 1;
            }
            for _ in 0..3 {
                let Some(x) = chars.get(idx..=idx) else {
                    break;
                };
                if x == b"X" {
                    result += 10;
                    idx += 1;
                } else {
                    break;
                }
            }
        }
        if chars.len() == idx {
            // SAFETY: idx is only incremented after adding to result,
            //         and chars is not empty, hence ``idx > 1``.
            return Ok(Self(unsafe { NonZero::new_unchecked(result) }));
        }

        // Ones: 9 ("IX"), 4 ("IV"), 0-3 (0 to 3 "I" characters),
        // or 5-8 ("V", followed by 0 to 3 "I" characters)
        if chars[idx..].starts_with(b"IX") {
            result += 9;
            idx += 2;
        } else if chars[idx..].starts_with(b"IV") {
            result += 4;
            idx += 2;
        } else {
            if chars.get(idx..=idx).unwrap_or_default() == b"V" {
                result += 5;
                idx += 1;
            }
            for _ in 0..3 {
                let Some(x) = chars.get(idx..=idx) else {
                    break;
                };
                if x == b"I" {
                    result += 1;
                    idx += 1;
                } else {
                    break;
                }
            }
        }
        if chars.len() == idx {
            // SAFETY: idx is only incremented after adding to result,
            //         and chars is not empty, hence ``idx > 1``.
            Ok(Self(unsafe { NonZero::new_unchecked(result) }))
        } else {
            Err(InvalidRomanNumeralError)
        }
    }
}

/// Numeral value, uppercase character, and lowercase character.
#[cfg(feature = "std")]
const ROMAN_NUMERAL_PREFIXES: &[(u16, &str, &str)] = &[
    (1000, "M", "m"),
    (900, "CM", "cm"),
    (500, "D", "d"),
    (400, "CD", "cd"),
    (100, "C", "c"),
    (90, "XC", "xc"),
    (50, "L", "l"),
    (40, "XL", "xl"),
    (10, "X", "x"),
    (9, "IX", "ix"),
    (5, "V", "v"),
    (4, "IV", "iv"),
    (1, "I", "i"),
];

impl TryFrom<u8> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u8``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u8) -> Result<Self, OutOfRangeError> {
        Self::new(u16::from(value))
    }
}

impl TryFrom<u16> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u16``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u16) -> Result<Self, OutOfRangeError> {
        Self::new(value)
    }
}

impl TryFrom<u32> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u32``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u32) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<u64> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u64``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u64) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<u128> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``u128``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u128) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<usize> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``usize``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: usize) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i8> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i8``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i8) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i16> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i16``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i16) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i32> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i32``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i32) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i64> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i64``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i64) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i128> for RomanNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``RomanNumeral`` from an ``i128``.
    ///
    /// Returns ``RomanNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i128) -> Result<Self, OutOfRangeError> {
        u16::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roman_numeral_new() {
        assert_eq!(RomanNumeral::new(0), Err(OutOfRangeError));
        assert_eq!(
            RomanNumeral::new(1),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::new(1_u8.into()),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::new(1_u16),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::new(42),
            Ok(RomanNumeral(NonZero::new(42_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::new(3_999),
            Ok(RomanNumeral(NonZero::new(3_999_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::new(MAX),
            Ok(RomanNumeral(NonZero::new(3_999_u16).unwrap()))
        );
        assert!(matches!(RomanNumeral::new(4_000), Err(OutOfRangeError)));
        assert!(matches!(RomanNumeral::new(u16::MAX), Err(OutOfRangeError)));
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_roman_numeral_to_string() {
        let test_numerals = [
            "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII",
            "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX", "XXI", "XXII", "XXIII", "XXIV",
        ];
        for (i, roman_str) in test_numerals.iter().enumerate() {
            let n = u16::try_from(i + 1).unwrap();
            let r = RomanNumeral::new(n).unwrap().to_string();
            assert_eq!(&r, roman_str);
        }
        assert_eq!(RomanNumeral::new(1984).unwrap().to_string(), "MCMLXXXIV");
    }

    #[test]
    fn test_roman_numeral_parse_string() {
        let test_numerals = [
            "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII",
            "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX", "XXI", "XXII", "XXIII", "XXIV",
        ];
        for (i, roman_str) in test_numerals.iter().enumerate() {
            let n = u16::try_from(i + 1).unwrap();
            let expected = RomanNumeral::new(n).unwrap();
            let parsed: RomanNumeral = roman_str.parse().expect("parsing failed!");
            assert_eq!(parsed, expected);
        }

        let parsed: RomanNumeral = "xvi".parse().unwrap();
        assert_eq!(parsed, RomanNumeral::new(16).unwrap());

        let parsed: RomanNumeral = "MDLXXXIII".parse().unwrap();
        assert_eq!(parsed, RomanNumeral::new(1583).unwrap());

        let parsed: RomanNumeral = "MCMLXXXIV".parse().unwrap();
        assert_eq!(parsed, RomanNumeral::new(1984).unwrap());

        let parsed: RomanNumeral = "MM".parse().unwrap();
        assert_eq!(parsed, RomanNumeral::new(2000).unwrap());

        let parsed: RomanNumeral = "MMMCMXCIX".parse().unwrap();
        assert_eq!(parsed, RomanNumeral::new(3_999).unwrap());
    }

    #[test]
    fn test_try_from_one() {
        assert_eq!(
            RomanNumeral::try_from(1_u8),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_u16),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_u32),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_u64),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_u128),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_usize),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_i8),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_i16),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_i32),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_i64),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
        assert_eq!(
            RomanNumeral::try_from(1_i128),
            Ok(RomanNumeral(NonZero::new(1_u16).unwrap()))
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_roman_numeral_round_trip() {
        for i in 1..=3_999 {
            let r = RomanNumeral::new(i).unwrap().to_string();
            let parsed: RomanNumeral = r.parse().unwrap();
            let val = parsed.0.get();
            assert_eq!(val, i);
        }
    }
}
