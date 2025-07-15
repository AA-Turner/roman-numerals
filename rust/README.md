# roman-numerals

A library for manipulating well-formed Roman numerals.

Integers between 1 and 3,999 (inclusive) are supported.
Numbers beyond this range will return an ``OutOfRangeError``.

The classical system of roman numerals requires that
the same character may not appear more than thrice consecutively,
meaning that 'MMMCMXCIX' (3,999) is the largest well-formed Roman numeral.
The smallest is 'I' (1), as there is no symbol for zero in Roman numerals.

Both upper- and lower-case formatting of roman numerals are supported,
and likewise for parsing strings,
although the entire string must be of the same case.
Numerals that do not adhere to the classical form are rejected
with an ``InvalidRomanNumeralError``.

## Example usage

### Create a roman numeral

```rust
use roman_numerals_rs::RomanNumeral;

let num = RomanNumeral::new(16)?;
assert_eq!(num.to_string(), "XVI");

let num: RomanNumeral = "XVI".parse()?;
assert_eq!(num.as_u16(), 16);

let num: RomanNumeral = 3_999.try_into().unwrap();
println!("{}", num);  // MMMCMXCIX
```

### Convert a roman numeral to a string

```rust
use roman_numerals_rs::RomanNumeral;

let num = RomanNumeral::new(16)?;
assert_eq!(num.to_string(), "XVI");
assert_eq!(num.to_uppercase(), "XVI");
assert_eq!(num.to_lowercase(), "xvi");
assert_eq!(format!("{:X}", num), "XVI");
assert_eq!(format!("{:x}", num), "xvi");
```

### Extract the decimal value of a roman numeral

```rust
use roman_numerals_rs::RomanNumeral;

let num = RomanNumeral::new(42)?;
assert_eq!(num.as_u16(), 42);
```

### Invalid input

```rust
use core::str::FromStr;
use roman_numerals_rs::{RomanNumeral, InvalidRomanNumeralError, OutOfRangeError};

let res = RomanNumeral::from_str("Spam!");
assert!(matches!(res.unwrap_err(), InvalidRomanNumeralError));

let res = "CLL".parse::<RomanNumeral>();
assert!(matches!(res.unwrap_err(), InvalidRomanNumeralError));

let res = RomanNumeral::new(0);
assert!(matches!(res.unwrap_err(), OutOfRangeError));

let res = RomanNumeral::new(4_000);
assert!(matches!(res.unwrap_err(), OutOfRangeError));
```

## Benchmarks

Run the benchmarks with ``cargo bench``.

## Licence

This project is licenced under the terms of either the Zero-Clause BSD licence
or the CC0 1.0 Universal licence.
