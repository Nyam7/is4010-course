# Unit Converter CLI

A simple Python command-line unit converter that supports common conversions between miles and kilometers, pounds and kilograms, Celsius and Fahrenheit, inches and centimeters, and gallons and liters.

## What this tool does

This project provides:

- `converter.py`: core conversion logic with reusable functions
- `main.py`: command-line entry point using `argparse`
- `test_converter.py`: a pytest test suite for conversion functions
- `.github/workflows/ci.yml`: GitHub Actions workflow to run tests automatically

## Installation

1. Install Python 3.10 or newer.
2. Clone the repository.
3. (Optional) Create and activate a virtual environment:

```bash
python -m venv venv
source venv/bin/activate
```

4. Install test dependencies:

```bash
python -m pip install pytest
```

## Usage

Run the converter from the command line:

```bash
python main.py <from_unit> <to_unit> <value>
```

### Supported units

- `miles`, `km`
- `lbs`, `kg`
- `celsius`, `fahrenheit`
- `inches`, `cm`
- `gallons`, `liters`

Synonyms are also accepted, such as `mile`, `mi`, `pound`, `lb`, `liter`, `litre`, `c`, and `f`.

## Examples

Convert miles to kilometers:

```bash
python main.py miles km 5
```

Convert pounds to kilograms:

```bash
python main.py lbs kg 150
```

Convert Fahrenheit to Celsius:

```bash
python main.py fahrenheit celsius 100
```

## Running tests

Run the test suite with pytest:

```bash
python -m pytest
```

## Notes

- The CLI provides helpful error messages if a unit is unsupported or the value is not numeric.
- No third-party library is required except `pytest` for testing.
