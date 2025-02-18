from __future__ import annotations

from roman_numerals import (
    MAX,
    MIN,
    RomanNumeral,
)


def test_round_trip() -> None:
    for n in range(MIN, MAX + 1):
        num = RomanNumeral(n)
        parsed = RomanNumeral.from_string(str(num))
        assert num == parsed
