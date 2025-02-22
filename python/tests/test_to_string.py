from __future__ import annotations

import pytest

from roman_numerals import (
    RomanNumeral,
)

from tests.utils import TEST_NUMERALS_LOWER, TEST_NUMERALS_UPPER


@pytest.mark.parametrize(('n', 'roman_str'), enumerate(TEST_NUMERALS_UPPER, start=1))
def test_str(n: int, roman_str: str) -> None:
    num = RomanNumeral(n)
    assert str(num) == roman_str
    assert f'{num}' == roman_str


@pytest.mark.parametrize(('n', 'roman_str'), enumerate(TEST_NUMERALS_UPPER, start=1))
def test_uppercase(n: int, roman_str: str) -> None:
    num = RomanNumeral(n)
    assert num.to_uppercase() == roman_str


@pytest.mark.parametrize(('n', 'roman_str'), enumerate(TEST_NUMERALS_LOWER, start=1))
def test_lowercase(n: int, roman_str: str) -> None:
    num = RomanNumeral(n)
    assert num.to_lowercase() == roman_str


def test_minitrue() -> None:
    # IGNORANCE IS STRENGTH
    num = RomanNumeral(1984)
    assert f'{num}' == 'MCMLXXXIV'
    assert num.to_uppercase() == 'MCMLXXXIV'
    assert num.to_lowercase() == 'mcmlxxxiv'
