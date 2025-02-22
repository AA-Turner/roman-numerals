use core::str::FromStr;
use roman_numerals_rs::InvalidRomanNumeralError;
use roman_numerals_rs::RomanNumeral;

fn main() {
    divan::main();
}

#[divan::bench(args = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"])]
fn from_str_one_to_ten_upper(input: &str) -> Result<RomanNumeral, InvalidRomanNumeralError> {
    RomanNumeral::from_str(input)
}

#[divan::bench(args = ["i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix", "x"])]
fn from_str_one_to_ten_lower(input: &str) -> Result<RomanNumeral, InvalidRomanNumeralError> {
    RomanNumeral::from_str(input)
}

#[divan::bench(args = ["MMMCMXCIX", "mmmcmxcix"])]
fn from_str_max(input: &str) -> Result<RomanNumeral, InvalidRomanNumeralError> {
    RomanNumeral::from_str(input)
}

#[divan::bench(args = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
fn to_string_one_to_ten(input: u16) -> String {
    RomanNumeral::new(input).unwrap().to_string()
}
