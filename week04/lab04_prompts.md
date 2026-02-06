# Lab 04: AI Prompts and Reasoning

## Problem 1: Finding Common Elements

### Prompt Used
"I have two large lists of product IDs from different suppliers, and I need to find which IDs appear in both lists. The order doesn't matter, but performance is important since the lists are very large. What Python data structure should I use to solve this efficiently?"

### AI Recommendation
Use **sets** with the intersection operator (`&`).

### Reasoning
- Sets provide O(1) average-case lookups
- The intersection operator (`set(list1) & set(list2)`) efficiently finds common elements
- Converting lists to sets is O(n), and intersection is O(n), making the overall approach O(n)
- This is much faster than nested loops which would be O(n²)
- The order of results doesn't matter, which makes sets ideal

### Implementation
```python
def find_common_elements(list1, list2):
    return list(set(list1) & set(list2))