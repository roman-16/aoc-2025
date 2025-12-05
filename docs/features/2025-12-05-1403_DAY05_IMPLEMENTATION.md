# Day 05: Cafeteria

## Status
Complete

## Answers
- **Part 1**: 694
- **Part 2**: 352716206375547

## Problem Summary
- **Input**: Two sections separated by blank line:
  1. Fresh ingredient ID ranges (e.g., "3-5" means IDs 3, 4, 5 are fresh)
  2. Available ingredient IDs (one per line)
- **Rule**: Ranges are inclusive and can overlap
- **Part 1**: Count how many available ingredient IDs are fresh (fall within any range)

## Example
```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

Results:
- ID 1: spoiled (not in any range)
- ID 5: fresh (in 3-5)
- ID 8: spoiled
- ID 11: fresh (in 10-14)
- ID 17: fresh (in 16-20 and 12-18)
- ID 32: spoiled

**Answer: 3 fresh IDs**

## Plan

### Part 1 Implementation
1. **Parse input**: Split by blank line into ranges section and IDs section
2. **Parse ranges**: Convert each "start-end" line into a range tuple
3. **Parse available IDs**: Convert each line to an integer
4. **Check freshness**: For each available ID, check if it falls within any range
5. **Count fresh**: Return count of fresh available IDs

### File Structure
- `src/bin/day05/main.rs` - Solution code with tests
- `src/bin/day05/input.txt` - Puzzle input

### Functions
- `main()` - Entry point, prints Part 1 (and Part 2 when available)
- `solve_part1(input: &str) -> usize` - Solve part 1
- `parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>)` - Parse ranges and IDs
- `is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool` - Check if ID is in any range

### Tests
- Example input test (3 fresh IDs)
- Edge cases: ID at range boundary, overlapping ranges, single range
- Main function execution test

## Clarifications
- Rules are unambiguous:
  - Ranges are inclusive (start and end both included)
  - ID is fresh if it falls within ANY range (overlaps don't matter)
  - Input has two sections separated by blank line
  - Parse ranges as "start-end" format

## Part 2
Count total unique IDs considered fresh by all ranges (ignoring available IDs section).

### Part 2 Implementation
1. Parse ranges only (ignore available IDs)
2. Merge overlapping/adjacent ranges
3. Sum the size of each merged range (end - start + 1)
