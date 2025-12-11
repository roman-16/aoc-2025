# Day 11: Reactor - Path Counting

## Problem Summary
- **Input**: A directed graph where each line `node: child1 child2 ...` defines edges from `node` to its children
- **Task (Part 1)**: Count all distinct paths from `you` to `out`
- **Graph Type**: Directed Acyclic Graph (DAG) - data flows one way only

## Example
```
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
```

Paths from `you` to `out`:
1. you → bbb → ddd → ggg → out
2. you → bbb → eee → out
3. you → ccc → ddd → ggg → out
4. you → ccc → eee → out
5. you → ccc → fff → out

**Answer**: 5 paths

## Implementation Plan

### Step 1: Parse Input
- Parse each line into a mapping: `node -> Vec<children>`
- Build a `HashMap<String, Vec<String>>` representing the adjacency list

### Step 2: Count Paths (Part 1)
- Use DFS with memoization to count paths from `you` to `out`
- Memoization: `memo[node]` = number of paths from `node` to `out`
- Base case: `memo["out"] = 1`
- Recursive case: `memo[node] = sum(memo[child] for child in children[node])`

### Step 3: Tests
- Test with example input (expected: 5)
- Test edge cases (empty input, direct path, single node)
- Test parsing functions
- Test main function execution

## Technical Decisions
- Use `HashMap` for adjacency list (efficient lookup)
- Use recursive DFS with memoization for path counting (efficient for DAGs)
- Return type: `u64` for potentially large path counts

## Solutions
- **Part 1**: 470
- **Part 2**: 384151614084875

## Quality Gates
1. `just build` - compile
2. `just lint` - clippy + formatting
3. `just test` - 100% coverage

## Part 2: Constrained Path Counting

- **Start**: `svr` (server rack)
- **End**: `out`
- **Constraint**: Path must visit BOTH `dac` AND `fft` (in any order)

### Approach
State-based memoization with 4 states per node:
- `memo[(node, has_dac, has_fft)]` = count of paths to `out` with constraint satisfied

### Example
8 total paths from `svr` to `out`, but only 2 visit both `dac` and `fft`.

## Clarifications
- **Q: Part 2?** A: Implement Part 1 only, wait for Part 2 requirements
