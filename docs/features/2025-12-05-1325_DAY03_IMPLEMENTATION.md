# Day 03 Implementation

## Summary

Implement Advent of Code 2025 Day 3 solution: Find maximum joltage from battery banks by selecting exactly two batteries per bank and summing all maximum joltages.

## Problem Understanding

- **Input**: Multiple lines, each line is a bank of batteries (digits 1-9)
- **Selection**: For each bank, select exactly 2 batteries (by position, not rearranged)
- **Joltage**: The 2-digit number formed by the selected digits in their original order
- **Goal**: Find the maximum joltage each bank can produce, then sum all maximums

### Examples from puzzle

| Bank | Maximum Joltage | Explanation |
|------|-----------------|-------------|
| `987654321111111` | 98 | First two batteries (positions 0,1) |
| `811111111111119` | 89 | Batteries at positions 0 and 15 |
| `234234234234278` | 78 | Last two batteries (positions 13,14) |
| `818181911112111` | 92 | Position 6 (9) and position 10 (2) |

**Expected sum**: 98 + 89 + 78 + 92 = **357**

## Algorithm

To maximize a 2-digit number XY:
1. Maximize X (tens digit) first
2. Then maximize Y (units digit) among digits that come after X

### Efficient approach (O(n) per bank)
1. Find the maximum digit that has at least one digit after it
2. Among all positions with that max digit, find the one followed by the highest possible digit
3. The result is `max_first_digit * 10 + max_digit_after`

### Simpler O(n^2) approach
1. For each pair of positions (i, j) where i < j:
   - Calculate `digit[i] * 10 + digit[j]`
2. Return maximum

Given typical puzzle input sizes, O(n^2) is acceptable and easier to verify.

## Implementation Plan

### Step 1: Create directory structure
- Create `src/bin/day03/` directory
- Create `src/bin/day03/main.rs`
- Create `src/bin/day03/input.txt` (empty placeholder)

### Step 2: Implement core logic
- `max_joltage(bank: &str) -> u32` - find maximum 2-digit joltage for a single bank
- `solve_part1(input: &str) -> u32` - parse input, compute max joltage per bank, sum results
- `solve_part2(input: &str) -> u32` - placeholder returning 0
- `main()` - read input and print answers

### Step 3: Write tests
- Test `max_joltage` with all 4 example banks
- Test `solve_part1` with example input (expected: 357)
- Test edge cases:
  - Minimum bank (2 digits): "12" -> 12, "91" -> 91
  - All same digits: "9999" -> 99
  - Descending: "987654321" -> 98
  - Ascending: "123456789" -> 89
- Panic tests for invalid input

### Step 4: Add puzzle input
- Copy actual puzzle input to `input.txt`

### Step 5: Validate
- Run all quality gates (`just check`)
- Ensure 100% test coverage

## Clarifications

**Q: Do you have the puzzle input ready?**
A: Yes - 200 lines of digit strings provided

**Q: How should I handle Part 2?**
A: Implement placeholder returning 0 - will provide Part 2 requirements later

## Answers

- **Part 1**: 16812
- **Part 2**: 166345822896410

## Files Created

- `src/bin/day03/main.rs`
- `src/bin/day03/input.txt`
