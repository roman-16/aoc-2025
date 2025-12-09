# Day 08: Playground (Junction Box Circuits)

## Status
Complete

## Answers
- **Part 1**: 115885
- **Part 2**: 274150525

## Problem Summary
- **Input**: List of 3D coordinates (X,Y,Z) representing junction box positions
- **Task**: Connect junction boxes to form circuits
- **Connection Rule**: Connect pairs of junction boxes closest together by straight-line (Euclidean) distance
- **Circuit Mechanics**:
  - Connected junction boxes form a circuit
  - Electricity flows between connected boxes
  - Connecting two boxes already in the same circuit does nothing (skip)
- **Goal**: After making 1000 shortest connections, multiply the sizes of the 3 largest circuits

## Example
```
162,817,812
57,618,57
906,360,560
...
```
- 20 junction boxes
- Closest pair: 162,817,812 and 425,690,689
- After 10 shortest connections: circuits of sizes 5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1
- Product of 3 largest (5 * 4 * 2) = 40

## Plan

### Algorithm: Union-Find (Disjoint Set Union)
This is a classic minimum spanning tree construction problem. Use Union-Find data structure to:
1. Track which junction boxes belong to which circuit
2. Efficiently check if two boxes are already connected
3. Track circuit sizes

### Part 1 Implementation

1. **Parse input**: Extract 3D coordinates into a vector of (x, y, z) tuples
2. **Generate all pairs**: Calculate distances for all pairs of junction boxes
3. **Sort by distance**: Order pairs from closest to farthest
4. **Union-Find operations**:
   - Initialize each box in its own circuit
   - Process pairs in distance order
   - For each pair: if not in same circuit, connect them (count as a connection)
   - Stop after 1000 connections
5. **Find largest circuits**: Get sizes of all circuits, take top 3, multiply

### Data Structures
- `Vec<(i64, i64, i64)>` - Junction box coordinates
- Union-Find with path compression and union by rank:
  - `parent: Vec<usize>` - Parent pointer for each node
  - `rank: Vec<usize>` - Rank for union by rank
  - `size: Vec<usize>` - Size of each circuit (tree)

### Distance Calculation
- Euclidean distance: sqrt((x2-x1)^2 + (y2-y1)^2 + (z2-z1)^2)
- For comparison purposes, use squared distance (avoid sqrt for efficiency)

### File Structure
- `src/bin/day08/main.rs` - Solution code with tests
- `src/bin/day08/input.txt` - Puzzle input (already exists)

### Functions
- `main()` - Entry point, prints Part 1 (and Part 2 when available)
- `solve_part1(input: &str) -> u64` - Solve part 1
- `parse_coordinates(input: &str) -> Vec<(i64, i64, i64)>` - Parse input coordinates
- `distance_squared(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64` - Calculate squared distance
- Union-Find struct with:
  - `new(n: usize)` - Initialize with n elements
  - `find(&mut self, x: usize) -> usize` - Find root with path compression
  - `union(&mut self, x: usize, y: usize) -> bool` - Union by rank, returns true if merged
  - `size(&mut self, x: usize) -> usize` - Get size of circuit containing x

### Tests
- Example input test (40 for 10 connections)
- Single pair test
- All boxes in same circuit test
- Edge cases: boxes at same position, empty input

## Part 2

### Problem
- Continue connecting until all boxes are in ONE circuit
- Find the last connection that merges two circuits
- Return product of X coordinates of those two boxes

### Algorithm Change
- Remove the 1000 connection limit
- Track which pair made the final merge (when circuit_count reaches 1)
- Return `coords[last_i].0 * coords[last_j].0`

### Example
- Last connection: 216,146,977 and 117,168,530
- Product: 216 * 117 = 25272

## Clarifications

### Q1: How to interpret "1000 connections"?
**Answer**: Process the 1000 closest pairs in distance order, even if some are redundant (already in same circuit). The example explicitly includes a redundant pair as one of the "ten shortest connections" - it counts toward the total but "nothing happens" to the circuits.
