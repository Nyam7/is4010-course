# Lab 02: Prompt Engineering Solutions

## Problem 1: Debugging

**My Prompt:**
> You are a senior Python developer.  
> Context: I have a Python function that is supposed to calculate the sum of all even numbers in a list, but it returns incorrect results.  
> Task: Identify the logical bug and provide a corrected version of the function.  
> Format: Return the fixed code in a Python code block and briefly explain the issue.

**AI's Corrected Code:**
```python
def sum_of_evens(numbers):
    total = 0
    for num in numbers:
        if num % 2 == 0:
            total += num
    return total
