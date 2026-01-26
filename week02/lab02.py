def factorial(n):
    """Calculate the factorial of a non-negative integer.

    Parameters
    ----------
    n : int
        The non-negative integer to calculate the factorial of.

    Returns
    -------
    int
        The factorial of n. Returns 1 for n = 0.
    """
    if n == 0:
        return 1

    result = 1
    for i in range(1, n + 1):
        result *= i
    return result


def is_prime(number):
    """Check if a number is a prime number."""
    if number <= 1:
        return False

    for i in range(2, int(number ** 0.5) + 1):
        if number % i == 0:
            return False
    return True


def reverse_string(s):
    """Reverse a given string."""
    return s[::-1]
