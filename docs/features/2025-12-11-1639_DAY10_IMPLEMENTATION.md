# Day 10: Factory (Minimum Button Presses)

## Status
- Part 1: Complete ✅
- Part 2: Complete ✅

## Answers
- **Part 1**: 422
- **Part 2**: 16361

## Problem Summary
- **Input**: List of machines, each with:
  - Indicator light diagram `[...]` - `.` = off, `#` = on (goal state)
  - Button wiring schematics `(...)` - each button toggles specific lights (0-indexed)
  - Joltage requirements `{...}` - ignored
- **Initial State**: All indicator lights are OFF
- **Goal**: Configure lights to match the diagram pattern
- **Mechanism**: Each button press toggles (XOR) the specified lights
- **Task**: Find minimum total button presses across all machines

## Example
```
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
```
- 4 lights, goal: OFF-ON-ON-OFF (binary: 0110)
- 6 buttons available
- Minimum: 2 presses (buttons `(0,2)` and `(0,1)` toggle lights 0,1,2 -> result is lights 1,2 ON)

## Algorithm Analysis

### Key Insight
This is a **XOR/GF(2) linear algebra problem**:
- Pressing a button twice cancels out (XOR property)
- Each button is pressed either 0 or 1 times
- Find minimum number of buttons to achieve target state

### Approach Options
1. **Brute Force**: Try all 2^n subsets of buttons (feasible for n <= ~20)
2. **Gaussian Elimination over GF(2)**: Find basis, then search for minimum weight solution
3. **BFS/Dijkstra**: Treat as shortest path problem (states = light configurations)

Given the input has machines with ~6-13 buttons, **brute force** (2^13 = 8192 max) is efficient enough.

### Algorithm
For each machine:
1. Parse target state from `[...]` diagram
2. Parse button effects from `(...)` schematics
3. For each subset of buttons (iterate 0 to 2^num_buttons - 1):
   - Apply XOR of selected button effects
   - If result matches target, track minimum button count
4. Return minimum presses found

### Complexity
- Per machine: O(2^b * n) where b = buttons, n = lights
- Total: O(m * 2^b * n) where m = machines

## Plan

### Part 1 Implementation

1. **Parse Input**
   - Extract pattern from `[...]`: convert `.#` to bits
   - Extract buttons from `(...)`: parse comma-separated indices
   - Ignore `{...}` joltage data

2. **Solve Single Machine**
   - Convert target pattern to bitmask
   - Convert each button to bitmask (which lights it toggles)
   - Brute force all 2^b button combinations
   - Find minimum count where XOR equals target

3. **Aggregate Results**
   - Sum minimum presses for all machines

### Data Structures
```rust
struct Machine {
    target: u64,         // Target light state as bitmask
    num_lights: usize,   // Number of lights
    buttons: Vec<u64>,   // Each button as bitmask of lights it toggles
}
```

### Functions
- `main()` - Entry point
- `solve_part1(input: &str) -> u64` - Sum of minimum presses
- `parse_machines(input: &str) -> Vec<Machine>` - Parse all machines
- `parse_machine(line: &str) -> Machine` - Parse single machine
- `min_presses(machine: &Machine) -> u64` - Find minimum presses for one machine

### Tests
- Example machines from problem (2 + 3 + 2 = 7)
- Single button needed
- All buttons needed
- Edge cases: single light, single button
- No solution case (if applicable)

## Part 2

### Problem
- Same machines, but now targets are joltage requirements `{...}`
- Buttons ADD to counters (not XOR toggle)
- Each button press adds 1 to each counter it affects
- Find minimum total presses to reach exact target values

### Algorithm: Gaussian Elimination
Based on Reddit discussion insights, this is an Integer Linear Programming problem:
- Set up matrix equation `Ax = b` (A = button effects, x = presses, b = targets)
- Use Gaussian elimination to get RREF (Reduced Row Echelon Form)
- At most **3 free variables** per machine (null space dimension ≤ 3)
- Brute force over free variables to find minimum non-negative integer solution

### Implementation
1. Build augmented matrix [A | b] where A[i][j] = 1 if button j affects counter i
2. Perform Gaussian elimination with partial pivoting
3. Identify pivot columns and free variables
4. Iterate all combinations of free variable values (0 to max_target)
5. Back-substitute to compute full solution
6. Return minimum sum among valid (non-negative integer) solutions

## Clarifications Needed
None - problem statement is clear

## File Structure
- `src/bin/day10/main.rs` - Solution code with tests
- `src/bin/day10/input.txt` - Puzzle input (already exists)
