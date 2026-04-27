"""Core conversion functions for the unit converter app."""

CONVERSION_MAP = {
    ('miles', 'km'): lambda value: value * 1.60934,
    ('km', 'miles'): lambda value: value / 1.60934,
    ('lbs', 'kg'): lambda value: value * 0.453592,
    ('kg', 'lbs'): lambda value: value / 0.453592,
    ('celsius', 'fahrenheit'): lambda value: value * 9 / 5 + 32,
    ('fahrenheit', 'celsius'): lambda value: (value - 32) * 5 / 9,
    ('inches', 'cm'): lambda value: value * 2.54,
    ('cm', 'inches'): lambda value: value / 2.54,
    ('gallons', 'liters'): lambda value: value * 3.78541,
    ('liters', 'gallons'): lambda value: value / 3.78541,
}

UNIT_ALIASES = {
    'mile': 'miles',
    'mi': 'miles',
    'kilometer': 'km',
    'kilometers': 'km',
    'kilometre': 'km',
    'kilometres': 'km',
    'pound': 'lbs',
    'pounds': 'lbs',
    'kilogram': 'kg',
    'kilograms': 'kg',
    'c': 'celsius',
    'f': 'fahrenheit',
    'inch': 'inches',
    'in': 'inches',
    'centimeter': 'cm',
    'centimetre': 'cm',
    'centimeters': 'cm',
    'centimetres': 'cm',
    'gallon': 'gallons',
    'gal': 'gallons',
    'liter': 'liters',
    'litre': 'liters',
    'liters': 'liters',
    'litres': 'liters',
}


def normalize_unit(unit: str) -> str:
    """Return a normalized unit name for supported conversions."""
    unit = unit.strip().lower()
    return UNIT_ALIASES.get(unit, unit)


def get_supported_units() -> list[str]:
    """Return a sorted list of supported unit names."""
    units = set(unit for pair in CONVERSION_MAP for unit in pair)
    return sorted(units)


def convert(value: float, from_unit: str, to_unit: str) -> float:
    """Convert a numeric value from one unit to another."""
    if not isinstance(value, (int, float)):
        raise TypeError('Value must be a number.')

    from_unit_normalized = normalize_unit(from_unit)
    to_unit_normalized = normalize_unit(to_unit)
    conversion_key = (from_unit_normalized, to_unit_normalized)

    if conversion_key not in CONVERSION_MAP:
        supported = ', '.join(get_supported_units())
        raise ValueError(
            f"Conversion from '{from_unit}' to '{to_unit}' is not supported. "
            f"Supported units: {supported}"
        )

    return CONVERSION_MAP[conversion_key](float(value))


def miles_to_km(value: float) -> float:
    return convert(value, 'miles', 'km')


def km_to_miles(value: float) -> float:
    return convert(value, 'km', 'miles')


def lbs_to_kg(value: float) -> float:
    return convert(value, 'lbs', 'kg')


def kg_to_lbs(value: float) -> float:
    return convert(value, 'kg', 'lbs')


def celsius_to_fahrenheit(value: float) -> float:
    return convert(value, 'celsius', 'fahrenheit')


def fahrenheit_to_celsius(value: float) -> float:
    return convert(value, 'fahrenheit', 'celsius')


def inches_to_cm(value: float) -> float:
    return convert(value, 'inches', 'cm')


def cm_to_inches(value: float) -> float:
    return convert(value, 'cm', 'inches')


def gallons_to_liters(value: float) -> float:
    return convert(value, 'gallons', 'liters')


def liters_to_gallons(value: float) -> float:
    return convert(value, 'liters', 'gallons')
