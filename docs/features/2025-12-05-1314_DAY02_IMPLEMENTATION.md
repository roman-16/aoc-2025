# Day 02 Implementation

## Summary

Implement Advent of Code 2025 Day 2 solution: Find invalid product IDs (numbers made of a digit sequence repeated twice) within given ranges and sum them.

## Problem Understanding

- **Input**: Comma-separated ranges on a single line (e.g., `11-22,95-115,998-1012,...`)
- **Invalid ID**: A number whose string representation is some digit sequence repeated twice
  - `55` = "5" + "5" (invalid)
  - `6464` = "64" + "64" (invalid)
  - `123123` = "123" + "123" (invalid)
  - `101` = odd length, cannot be doubled (valid)
- **Output**: Sum of all invalid IDs across all ranges

## Algorithm

1. Parse input: split by `,`, then each range by `-` to get start/end
2. For each number in each range:
   - Convert to string
   - If length is odd, skip (cannot be doubled)
   - If length is even, check if first half equals second half
   - If doubled, add to sum
3. Return total sum

## Test Cases (from puzzle)

| Range | Invalid IDs | Notes |
|-------|-------------|-------|
| 11-22 | 11, 22 | 2 IDs |
| 95-115 | 99 | 1 ID |
| 998-1012 | 1010 | 1 ID |
| 1188511880-1188511890 | 1188511885 | 1 ID |
| 222220-222224 | 222222 | 1 ID |
| 1698522-1698528 | (none) | 0 IDs |
| 446443-446449 | 446446 | 1 ID |
| 38593856-38593862 | 38593859 | 1 ID |
| 565653-565659 | (none) | 0 IDs |
| 824824821-824824827 | (none) | 0 IDs |
| 2121212118-2121212124 | (none) | 0 IDs |

**Expected sum**: 11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446 + 38593859 = **1227775554**

## Implementation Plan

### Step 1: Create directory structure
- Create `src/bin/day02/` directory
- Create `src/bin/day02/main.rs`
- Create `src/bin/day02/input.txt` (empty placeholder)

### Step 2: Implement core logic
- `is_doubled(n: u64) -> bool` - check if number is a doubled sequence
- `solve_part1(input: &str) -> u64` - parse ranges and sum invalid IDs
- `main()` - read input and print answer

### Step 3: Write tests
- Test `is_doubled` with known examples (11, 22, 99, 1010, 123123)
- Test `is_doubled` with valid IDs (101, 123, 1234)
- Test `solve_part1` with example input
- Test edge cases (single digit ranges, large numbers)
- Panic tests for invalid input

### Step 4: Add puzzle input
- Copy actual puzzle input to `input.txt`

### Step 5: Validate
- Run all quality gates (`just check`)
- Ensure 100% test coverage

## Clarifications

**Q: Puzzle input?**
A: Provided - 30 ranges with numbers up to ~8 billion (requires u64)

**Q: Part 2 handling?**
A: Implement Part 1 with placeholder `solve_part2` returning 0

## Answers

- **Part 1**: 56660955519
- **Part 2**: 79183223243

## Files Created

- `src/bin/day02/main.rs`
- `src/bin/day02/input.txt`
