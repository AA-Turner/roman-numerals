from __future__ import annotations

import pytest

from roman_numerals import (
    MAX,
    MIN,
    OutOfRangeError,
    RomanNumeral,
)


def test_zero() -> None:
    with pytest.raises(OutOfRangeError) as ctx:
        RomanNumeral(0)
    msg = str(ctx.value)
    assert msg == 'Number out of range (must be between 1 and 3,999). Got 0.'


def test_one() -> None:
    assert int(RomanNumeral(1)) == 1


def test_MIN() -> None:  # NoQA: N802
    assert int(RomanNumeral(MIN)) == MIN


def test_forty_two() -> None:
    assert int(RomanNumeral(42)) == 42  # NoQA: PLR2004


def test_three_thousand_nine_hundred_and_ninety_nine() -> None:
    assert int(RomanNumeral(3_999)) == 3_999  # NoQA: PLR2004


def test_MAX() -> None:  # NoQA: N802
    assert int(RomanNumeral(MAX)) == MAX


def test_four_thousand() -> None:
    with pytest.raises(OutOfRangeError) as ctx:
        RomanNumeral(4_000)
    msg = str(ctx.value)
    assert msg == 'Number out of range (must be between 1 and 3,999). Got 4000.'


def test_minus_one() -> None:
    with pytest.raises(OutOfRangeError) as ctx:
        RomanNumeral(-1)
    msg = str(ctx.value)
    assert msg == 'Number out of range (must be between 1 and 3,999). Got -1.'


def test_float() -> None:
    with pytest.raises(TypeError) as ctx:
        RomanNumeral(4.2)  # type: ignore[arg-type]
    msg = str(ctx.value)
    assert msg == "RomanNumeral: an integer is required, not 'float'"
