"""Tests for the converter module."""

import pytest

from converter import (
    convert,
    celsius_to_fahrenheit,
    fahrenheit_to_celsius,
    gallons_to_liters,
    inches_to_cm,
    kg_to_lbs,
    lbs_to_kg,
    miles_to_km,
    cm_to_inches,
)


def test_miles_to_km_zero():
    assert miles_to_km(0) == 0


def test_km_to_miles():
    assert pytest.approx(convert(10, 'km', 'miles'), rel=1e-4) == 6.2137


def test_lbs_to_kg_and_back():
    assert pytest.approx(lbs_to_kg(10), rel=1e-4) == 4.5359
    assert pytest.approx(kg_to_lbs(4.5359), rel=1e-4) == 10.0


def test_temperature_conversion():
    assert pytest.approx(celsius_to_fahrenheit(100), rel=1e-4) == 212
    assert pytest.approx(fahrenheit_to_celsius(32), rel=1e-4) == 0


def test_inch_cm_round_trip():
    assert pytest.approx(inches_to_cm(10), rel=1e-4) == 25.4
    assert pytest.approx(cm_to_inches(25.4), rel=1e-4) == 10


def test_gallons_to_liters():
    assert pytest.approx(gallons_to_liters(5), rel=1e-4) == 18.92705


def test_invalid_conversion_units():
    with pytest.raises(ValueError):
        convert(1, 'miles', 'liters')


def test_invalid_value_type():
    with pytest.raises(TypeError):
        convert('five', 'miles', 'km')
