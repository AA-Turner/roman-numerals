from __future__ import annotations

import pytest

from roman_numerals import (
    InvalidRomanNumeralError,
    RomanNumeral,
)

from tests.utils import TEST_NUMERALS_LOWER, TEST_NUMERALS_UPPER


@pytest.mark.parametrize(('n', 'roman_str'), enumerate(TEST_NUMERALS_UPPER, start=1))
def test_uppercase(n: int, roman_str: str) -> None:
    expected = RomanNumeral(n)
    parsed = RomanNumeral.from_string(roman_str)
    assert expected == parsed


@pytest.mark.parametrize(('n', 'roman_str'), enumerate(TEST_NUMERALS_LOWER, start=1))
def test_lowercase(n: int, roman_str: str) -> None:
    expected = RomanNumeral(n)
    parsed = RomanNumeral.from_string(roman_str)
    assert expected == parsed


def test_special() -> None:
    parsed = RomanNumeral.from_string('MDLXXXIII')
    assert RomanNumeral(1583) == parsed

    parsed = RomanNumeral.from_string('mdlxxxiii')
    assert RomanNumeral(1583) == parsed

    parsed = RomanNumeral.from_string('MCMLXXXIV')
    assert RomanNumeral(1984) == parsed

    parsed = RomanNumeral.from_string('mcmlxxxiv')
    assert RomanNumeral(1984) == parsed

    parsed = RomanNumeral.from_string('MM')
    assert RomanNumeral(2000) == parsed

    parsed = RomanNumeral.from_string('mm')
    assert RomanNumeral(2000) == parsed

    parsed = RomanNumeral.from_string('MMMCMXCIX')
    assert RomanNumeral(3_999) == parsed

    parsed = RomanNumeral.from_string('mmmcmxcix')
    assert RomanNumeral(3_999) == parsed


def test_invalid() -> None:
    with pytest.raises(InvalidRomanNumeralError) as ctx:
        RomanNumeral.from_string('Not a Roman numeral!')
    msg = str(ctx.value)
    assert msg == 'Invalid Roman numeral: Not a Roman numeral!'


def test_mixed_case() -> None:
    with pytest.raises(InvalidRomanNumeralError) as ctx:
        RomanNumeral.from_string('McMlXxXiV')
    msg = str(ctx.value)
    assert msg == 'Invalid Roman numeral: McMlXxXiV'
