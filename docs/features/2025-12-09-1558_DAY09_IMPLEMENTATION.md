# Day 09: Movie Theater (Largest Rectangle from Red Tiles)

## Status
Complete

## Answers
- **Part 1**: 4737096935
- **Part 2**: 1644094530

## Problem Summary
- **Input**: List of 2D coordinates (X,Y) representing red tile positions on a grid
- **Task**: Find the largest rectangle area using two red tiles as opposite corners
- **Rectangle**: Any two red tiles define opposite corners of a rectangle
- **Area Calculation**: `|x2 - x1| * |y2 - y1|` for tiles at (x1,y1) and (x2,y2)
- **Goal**: Return the maximum possible rectangle area

## Example
```
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
```
- 8 red tiles
- Largest rectangle: between (2,5) and (11,1) = |11-2| * |5-1| = 9 * 4... wait
- Actually: between (2,5) and (11,1) means width = 11-2 = 9, height = 5-1 = 4, but example says 50
- Let me recalculate: (2,5) to (11,1) gives |11-2| * |5-1| = 9 * 4 = 36... not 50
- Looking at example: area 50 from (2,5) to (11,1) shown as 10 wide × 5 tall
- Ah! The formula should be `(|x2-x1| + 1) * (|y2-y1| + 1)` if we count tiles inclusively? No...
- Wait: width = 11-2+1 = 10, but height shown is 5 rows (from y=1 to y=5 inclusive)
- Area = (11-2) * (5-1) = 9 * 4 = 36? Still not 50.
- Re-reading: "between 2,5 and 11,1" - the rectangle spans from x=2 to x=11 (10 cols) and y=1 to y=5 (5 rows)
- That's (11-2+1) * (5-1+1) = 10 * 5 = 50 ✓
- **Formula**: `(|x2-x1| + 1) * (|y2-y1| + 1)` for inclusive tile count

## Plan

### Part 1 Implementation

1. **Parse input**: Extract 2D coordinates into a vector of (x, y) tuples
2. **Check all pairs**: For each pair of red tiles, calculate rectangle area
3. **Area formula**: `(|x2-x1| + 1) * (|y2-y1| + 1)` (inclusive grid counting)
4. **Return maximum**: Track and return the largest area found

### Algorithm
- O(n²) brute force - check all pairs
- For n red tiles, there are n*(n-1)/2 pairs
- Simple and efficient enough for expected input size

### Data Structures
- `Vec<(i64, i64)>` - Red tile coordinates

### File Structure
- `src/bin/day09/main.rs` - Solution code with tests
- `src/bin/day09/input.txt` - Puzzle input (already exists)

### Functions
- `main()` - Entry point, prints Part 1 (and Part 2 when available)
- `solve_part1(input: &str) -> i64` - Solve part 1
- `parse_coordinates(input: &str) -> Vec<(i64, i64)>` - Parse input coordinates
- `rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> i64` - Calculate rectangle area

### Tests
- Example input test (area 50)
- Two tiles on same row (area based on width only)
- Two tiles on same column (area based on height only)
- Two tiles at same position (area 1)
- Single tile (no rectangle possible - return 0)
- Empty input (return 0)

## Part 2

### Problem
- Red tiles form a loop connected by green tiles
- Interior of the loop is also green
- Rectangle must have red corners AND all tiles inside must be red or green

### Algorithm
- Use coordinate compression (496 red tiles with coords up to ~100k)
- Build boundary in compressed space
- Flood fill from outside to find interior
- Check each pair of red tiles for valid rectangle

### Key Optimization
- Coordinate compression reduces grid from ~100k×100k to ~496×496
- Makes flood fill and rectangle checking tractable

## Clarifications
- **Area formula**: Confirmed as `(|dx|+1) * (|dy|+1)` (inclusive grid counting)
- Verified against all example cases: 50, 35, 24, 6
