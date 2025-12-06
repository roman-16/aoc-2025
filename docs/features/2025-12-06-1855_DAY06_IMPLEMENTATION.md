# Day 06: Trash Compactor

## Status
Complete

## Answers
- **Part 1**: 6100348226985
- **Part 2**: 12377473011151

## Problem Summary
- **Input**: A "math worksheet" with problems arranged horizontally in columns
- **Format**: Numbers are arranged vertically in columns, separated by columns of only spaces
- **Operator**: Each column has an operator (+, *) at the bottom row
- **Task**: Parse each problem, apply the operator to all numbers in that column, sum all results

## Example
```
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
```

This contains 4 problems:
- Problem 1: 123 * 45 * 6 = 33210
- Problem 2: 328 + 64 + 98 = 490
- Problem 3: 51 * 387 * 215 = 4243455
- Problem 4: 64 + 23 + 314 = 401

**Grand Total: 33210 + 490 + 4243455 + 401 = 4277556**

## Plan

### Part 1 Implementation
1. **Parse input**: Read lines, identify the operator row (last row)
2. **Identify columns**: Find column boundaries using space-only separators
3. **Extract problems**: For each column group, extract numbers and operator
4. **Solve each problem**: Apply operator to all numbers in the column
5. **Sum results**: Return grand total of all problem results

### File Structure
- `src/bin/day06/main.rs` - Solution code with tests
- `src/bin/day06/input.txt` - Puzzle input

### Functions
- `main()` - Entry point, prints Part 1 (and Part 2 when available)
- `solve_part1(input: &str) -> u64` - Solve part 1
- `parse_problems(input: &str) -> Vec<(Vec<u64>, char)>` - Parse into (numbers, operator) tuples
- `solve_problem(numbers: &[u64], operator: char) -> u64` - Apply operator to numbers

### Parsing Strategy
1. Find separator columns (columns that are ALL spaces except possibly the operator row)
2. Group consecutive non-separator columns into problems
3. For each problem column group:
   - Extract numbers from all rows except last
   - Extract operator from last row

### Tests
- Example input test (grand total = 4277556)
- Single problem test
- Edge cases: single number per column, large numbers
- Main function execution test

## Clarifications
- Input file confirmed: 4 rows of numbers + 1 row of operators
- User provided input.txt file

## Part 2
Cephalopod math reads numbers differently:
- Each column within a problem is a separate number
- Digits are read top-to-bottom (most significant first)
- Columns are processed right-to-left

Example: "123" / "45" / "6" read column-wise becomes:
- Col 2: 3, 5, 6 → 356
- Col 1: 2, 4 → 24
- Col 0: 1 → 1
Result: 356 * 24 * 1 = 8544
