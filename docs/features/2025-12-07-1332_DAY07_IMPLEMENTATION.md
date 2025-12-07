# Day 07: Laboratories (Tachyon Manifold)

## Status
Complete

## Answers
- **Part 1**: 1690
- **Part 2**: 221371496188107

## Problem Summary
- **Input**: A grid representing a tachyon manifold diagram
- **Characters**: `S` (start), `.` (empty space), `^` (splitter)
- **Mechanics**:
  - Tachyon beam enters at `S` and travels downward
  - Beams pass freely through empty space (`.`)
  - When a beam hits a splitter (`^`), it stops and two new beams emit from immediate left and right
  - Multiple beams can merge into the same location (no duplication)
- **Task**: Count total number of times beams are split

## Example
```
.......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
...etc...
```

In the example, beams split 21 times total.

## Plan

### Part 1 Implementation

1. **Parse input**: Read grid, find `S` position and all `^` positions
2. **Simulate beam propagation**:
   - Track active beam positions (columns with downward-moving beams)
   - Process row by row from top to bottom
   - When a beam encounters a splitter, count it and spawn two new beams (left and right)
   - Use a Set to avoid duplicate beams at same column
3. **Count splits**: Each time a beam hits a `^`, increment the split counter
4. **Return total splits**

### Data Structures
- `HashSet<usize>` for tracking active beam columns (handles merging automatically)
- Grid as `Vec<Vec<char>>` or direct line-by-line processing

### Algorithm
```
1. Find column of S
2. Initialize beams = {S_column}
3. For each row below S:
   - For each beam position:
     - If splitter at position: count split, add left and right to new_beams
     - Else: add current position to new_beams
   - beams = new_beams (deduped via Set)
4. Return split count
```

### File Structure
- `src/bin/day07/main.rs` - Solution code with tests
- `src/bin/day07/input.txt` - Puzzle input (already exists)

### Functions
- `main()` - Entry point, prints Part 1 (and Part 2 when available)
- `solve_part1(input: &str) -> usize` - Solve part 1
- `parse_grid(input: &str) -> (Vec<&str>, usize)` - Parse grid, return (lines, start_column)
- `simulate_beams(lines: &[&str], start_col: usize) -> usize` - Simulate and count splits

### Tests
- Example input test (21 splits)
- Single splitter test
- Edge cases: beam exits grid, multiple beams merge
- Main function execution test

## Clarifications
- Problem mechanics were well-specified, no clarifications needed
- Beam movement: always downward
- Split mechanics: beam stops at splitter, two new beams start at left/right columns
- Counting: each beam hitting a splitter = 1 split
- Merging: beams at same column tracked as one (HashSet)

## Part 2

### Quantum Tachyon Manifold (Many-Worlds Interpretation)
- Single particle takes BOTH paths at each splitter
- Each split creates a new timeline (no merging)
- Count total number of active timelines at the end

### Algorithm Change
- Use `HashMap<column, timeline_count>` instead of `HashSet<column>`
- When splitting: add timeline count to both left and right positions
- Timelines at same position accumulate (sum counts)
- Return sum of all timeline counts at end

### Key Insight
- Part 1: Beams merge (deduplicated with HashSet)
- Part 2: Timelines don't merge (tracked with HashMap counts)
