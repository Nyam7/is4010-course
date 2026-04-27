#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn safe_sqrt(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n.sqrt())
    }
}

pub fn parse_number(s: &str) -> Option<i32> {
    s.trim().parse().ok()
}

pub fn divide_numbers(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match safe_sqrt(4.0) {
        Ok(result) => println!("sqrt(4.0) = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match safe_sqrt(-1.0) {
        Ok(result) => println!("sqrt(-1.0) = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    if let Some(num) = parse_number("42") {
        println!("Parsed: {}", num);
    }

    if let Some(result) = divide_numbers(20, 4) {
        println!("20 / 4 = {}", result);
    }

    if divide_numbers(20, 0).is_none() {
        println!("Cannot divide by zero");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_safe_sqrt_success() {
        assert_eq!(safe_sqrt(4.0), Ok(2.0));
    }

    #[test]
    fn test_safe_sqrt_negative() {
        assert_eq!(safe_sqrt(-1.0), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_parse_number_success() {
        assert_eq!(parse_number("42"), Some(42));
    }

    #[test]
    fn test_parse_number_failure() {
        assert_eq!(parse_number("not a number"), None);
    }

    #[test]
    fn test_divide_numbers_success() {
        assert_eq!(divide_numbers(20, 4), Some(5));
    }

    #[test]
    fn test_divide_numbers_by_zero() {
        assert_eq!(divide_numbers(20, 0), None);
    }
}
