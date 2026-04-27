"""Command-line interface for the unit converter app."""
import argparse
import sys

from converter import convert, get_supported_units


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description='Convert between common units such as miles and kilometers.'
    )
    parser.add_argument(
        'from_unit',
        help='Unit to convert from, for example miles, km, lbs, kg, celsius, fahrenheit, inches, cm, gallons, liters',
    )
    parser.add_argument(
        'to_unit',
        help='Unit to convert to, for example km, miles, kg, lbs, fahrenheit, celsius, cm, inches, liters, gallons',
    )
    parser.add_argument(
        'value',
        type=float,
        help='Numeric value to convert',
    )
    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()

    try:
        result = convert(args.value, args.from_unit, args.to_unit)
        print(f"{args.value} {args.from_unit} = {result:.4f} {args.to_unit}")
        return 0
    except ValueError as error:
        print(f"Error: {error}", file=sys.stderr)
        print("Supported units:", ', '.join(get_supported_units()), file=sys.stderr)
        return 2
    except TypeError as error:
        print(f"Error: {error}", file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
