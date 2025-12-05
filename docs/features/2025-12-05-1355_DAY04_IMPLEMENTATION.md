# Day 04: Printing Department

## Status
Complete

## Answers
- **Part 1**: 1549
- **Part 2**: 8887

## Problem Summary
- **Input**: Grid of paper rolls (`@`) and empty spaces (`.`)
- **Rule**: Forklift can access a roll if there are **fewer than 4** rolls in the 8 adjacent positions
- **Part 1**: Count how many rolls can be accessed by a forklift

## Example
```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
```

Accessible rolls (13 total) marked with `x`:
```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

## Plan

### Part 1 Implementation
1. **Parse grid**: Read input into a 2D structure (Vec<Vec<char>> or Vec<&[u8]>)
2. **Count neighbors**: For each cell with `@`, count adjacent `@` symbols in 8 directions
3. **Filter accessible**: Roll is accessible if neighbor count < 4
4. **Return count**: Sum all accessible rolls

### File Structure
- `src/bin/day04/main.rs` - Solution code with tests
- `src/bin/day04/input.txt` - Puzzle input

### Functions
- `main()` - Entry point, prints Part 1 (and Part 2 when available)
- `solve_part1(input: &str) -> usize` - Solve part 1
- `count_adjacent_rolls(grid: &[&[u8]], row: usize, col: usize) -> usize` - Count @ neighbors
- `parse_grid(input: &str) -> Vec<&[u8]>` - Parse input into grid

### Tests
- Example input test (13 accessible rolls)
- Edge cases: single roll, corner rolls, edge rolls
- Main function execution test

## Clarifications
- Input received: 140x140 grid
- Rules are unambiguous:
  - `@` = paper roll, `.` = empty space
  - Check 8 adjacent positions (including diagonals)
  - "Fewer than 4" means 0, 1, 2, or 3 neighboring rolls
  - Edge/corner positions: only count neighbors within grid bounds

## Part 2
Iteratively remove accessible rolls until none remain. Count total removed.

### Part 2 Implementation
1. Parse grid into mutable `Vec<Vec<u8>>`
2. Loop: find all accessible rolls, remove them, add to total
3. Stop when no accessible rolls remain
4. Return total removed
