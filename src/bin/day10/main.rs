fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let machine = parse_machine(line);
            min_presses(&machine)
        })
        .sum()
}

struct Machine {
    target: u64,
    buttons: Vec<u64>,
}

fn parse_machine(line: &str) -> Machine {
    let target = parse_target(line);
    let buttons = parse_buttons(line);
    Machine { target, buttons }
}

fn parse_target(line: &str) -> u64 {
    let start = line.find('[').expect("missing [") + 1;
    let end = line.find(']').expect("missing ]");
    let pattern = &line[start..end];

    pattern
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(0u64, |acc, (i, _)| acc | (1 << i))
}

fn parse_buttons(line: &str) -> Vec<u64> {
    let mut buttons = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '(' {
            let mut content = String::new();
            for c in chars.by_ref() {
                if c == ')' {
                    break;
                }
                content.push(c);
            }
            buttons.push(parse_button(&content));
        }
    }

    buttons
}

fn parse_button(content: &str) -> u64 {
    content
        .split(',')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .fold(0u64, |acc, i| acc | (1 << i))
}

fn min_presses(machine: &Machine) -> u64 {
    let num_buttons = machine.buttons.len();

    // Try all 2^n subsets of buttons
    (0u64..(1 << num_buttons))
        .filter_map(|mask| {
            let result = apply_buttons(mask, &machine.buttons);
            if result == machine.target {
                Some(mask.count_ones() as u64)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(0)
}

fn apply_buttons(mask: u64, buttons: &[u64]) -> u64 {
    buttons
        .iter()
        .enumerate()
        .filter(|(i, _)| (mask & (1 << i)) != 0)
        .fold(0u64, |acc, (_, button)| acc ^ button)
}

// Part 2: Addition-based counter system using Gaussian elimination

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let buttons = parse_buttons_indices(line);
            let targets = parse_joltage(line);
            min_presses_gauss(&buttons, &targets)
        })
        .sum()
}

fn parse_buttons_indices(line: &str) -> Vec<Vec<usize>> {
    let mut buttons = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '(' {
            let mut content = String::new();
            for c in chars.by_ref() {
                if c == ')' {
                    break;
                }
                content.push(c);
            }
            let indices: Vec<usize> = content
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            buttons.push(indices);
        }
    }

    buttons
}

fn parse_joltage(line: &str) -> Vec<u64> {
    let start = line.find('{').expect("missing {") + 1;
    let end = line.find('}').expect("missing }");
    let content = &line[start..end];

    content
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

/// Solve using Gaussian elimination over rationals
/// Problem: Ax = b, minimize sum(x) where x >= 0 and x is integer
fn min_presses_gauss(buttons: &[Vec<usize>], targets: &[u64]) -> u64 {
    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    let n_buttons = buttons.len();
    let n_counters = targets.len();

    // Build augmented matrix [A | b] using rationals (as f64 for simplicity)
    // A[i][j] = 1 if button j affects counter i
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n_buttons + 1]; n_counters];

    for (j, button) in buttons.iter().enumerate() {
        for &counter in button {
            if counter < n_counters {
                matrix[counter][j] = 1.0;
            }
        }
    }

    // Set target column
    for (i, &target) in targets.iter().enumerate() {
        matrix[i][n_buttons] = target as f64;
    }

    // Gaussian elimination to get RREF
    let (rref, pivot_cols) = gaussian_elimination(&mut matrix, n_buttons);

    // Identify free variables (columns without pivots)
    let free_vars: Vec<usize> = (0..n_buttons)
        .filter(|col| !pivot_cols.contains(col))
        .collect();

    // Upper bound for free variables is max target value
    let max_target = *targets.iter().max().unwrap_or(&0) as i64;

    // Brute force over free variables
    let mut best = u64::MAX;
    brute_force_free_vars(
        &rref,
        &pivot_cols,
        &free_vars,
        n_buttons,
        max_target,
        &mut best,
    );

    best
}

fn gaussian_elimination(matrix: &mut [Vec<f64>], n_cols: usize) -> (Vec<Vec<f64>>, Vec<usize>) {
    let n_rows = matrix.len();
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..n_cols {
        // Find pivot in this column (row with max absolute value)
        let (max_row, max_val) = matrix
            .iter()
            .enumerate()
            .skip(pivot_row)
            .map(|(i, row)| (i, row[col].abs()))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap_or((pivot_row, 0.0));

        if max_val < 1e-10 {
            continue; // No pivot in this column
        }

        // Swap rows
        matrix.swap(pivot_row, max_row);

        // Scale pivot row
        let pivot_val = matrix[pivot_row][col];
        matrix[pivot_row]
            .iter_mut()
            .take(n_cols + 1)
            .for_each(|v| *v /= pivot_val);

        // Eliminate other rows
        for row in 0..n_rows {
            if row != pivot_row && matrix[row][col].abs() > 1e-10 {
                let factor = matrix[row][col];
                let pivot_row_copy: Vec<f64> =
                    matrix[pivot_row].iter().take(n_cols + 1).copied().collect();
                matrix[row]
                    .iter_mut()
                    .zip(pivot_row_copy.iter())
                    .for_each(|(v, &p)| *v -= factor * p);
            }
        }

        pivot_cols.push(col);
        pivot_row += 1;

        if pivot_row >= n_rows {
            break;
        }
    }

    (matrix.to_vec(), pivot_cols)
}

fn brute_force_free_vars(
    rref: &[Vec<f64>],
    pivot_cols: &[usize],
    free_vars: &[usize],
    n_buttons: usize,
    max_val: i64,
    best: &mut u64,
) {
    let n_free = free_vars.len();
    let total_combinations = (max_val + 1).pow(n_free as u32);

    for combo in 0..total_combinations {
        let free_vals: Vec<i64> = (0..n_free)
            .scan(combo, |temp, _| {
                let val = *temp % (max_val + 1);
                *temp /= max_val + 1;
                Some(val)
            })
            .collect();

        // Compute all button values from RREF
        if let Some(solution) = compute_solution(rref, pivot_cols, free_vars, &free_vals, n_buttons)
        {
            let total: u64 = solution.iter().sum();
            if total < *best {
                *best = total;
            }
        }
    }
}

fn compute_solution(
    rref: &[Vec<f64>],
    pivot_cols: &[usize],
    free_vars: &[usize],
    free_vals: &[i64],
    n_buttons: usize,
) -> Option<Vec<u64>> {
    let mut solution = vec![0.0; n_buttons];

    // Set free variables
    for (i, &col) in free_vars.iter().enumerate() {
        solution[col] = free_vals[i] as f64;
    }

    // Back-substitute to find pivot variables
    for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
        if row_idx >= rref.len() {
            break;
        }

        let mut val = rref[row_idx][n_buttons]; // RHS

        // Subtract contributions from free variables
        for &free_col in free_vars {
            val -= rref[row_idx][free_col] * solution[free_col];
        }

        // Subtract contributions from other pivot variables (already computed)
        for (other_row, &other_pivot) in pivot_cols.iter().enumerate() {
            if other_row > row_idx && other_pivot < n_buttons {
                val -= rref[row_idx][other_pivot] * solution[other_pivot];
            }
        }

        solution[pivot_col] = val;
    }

    // Check if solution is valid (non-negative integers)
    let mut result = Vec::with_capacity(n_buttons);
    for &val in &solution {
        let rounded = val.round();
        if (val - rounded).abs() > 1e-6 || rounded < 0.0 {
            return None;
        }
        result.push(rounded as u64);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 7);
    }

    #[test]
    fn test_parse_target_simple() {
        assert_eq!(parse_target("[.##.] (1) {1}"), 0b0110);
    }

    #[test]
    fn test_parse_target_first_on() {
        assert_eq!(parse_target("[#...] (1) {1}"), 0b0001);
    }

    #[test]
    fn test_parse_target_all_on() {
        assert_eq!(parse_target("[####] (1) {1}"), 0b1111);
    }

    #[test]
    fn test_parse_target_all_off() {
        assert_eq!(parse_target("[....] (1) {1}"), 0);
    }

    #[test]
    fn test_parse_target_mixed() {
        assert_eq!(parse_target("[...#.] (1) {1}"), 0b01000);
    }

    #[test]
    fn test_parse_target_six_lights() {
        assert_eq!(parse_target("[.###.#] (1) {1}"), 0b101110);
    }

    #[test]
    fn test_parse_buttons_single_index() {
        let buttons = parse_buttons("[.] (3) {1}");
        assert_eq!(buttons, vec![0b1000]);
    }

    #[test]
    fn test_parse_buttons_multiple_indices() {
        let buttons = parse_buttons("[.] (1,3) {1}");
        assert_eq!(buttons, vec![0b1010]);
    }

    #[test]
    fn test_parse_buttons_multiple_buttons() {
        let buttons = parse_buttons("[.] (3) (1,3) (2) {1}");
        assert_eq!(buttons, vec![0b1000, 0b1010, 0b0100]);
    }

    #[test]
    fn test_parse_button_content() {
        assert_eq!(parse_button("0,2"), 0b0101);
        assert_eq!(parse_button("1,3,4"), 0b11010);
        assert_eq!(parse_button("0"), 0b1);
    }

    #[test]
    fn test_apply_buttons_none() {
        let buttons = vec![0b0001, 0b0010, 0b0100];
        assert_eq!(apply_buttons(0b000, &buttons), 0);
    }

    #[test]
    fn test_apply_buttons_single() {
        let buttons = vec![0b0001, 0b0010, 0b0100];
        assert_eq!(apply_buttons(0b001, &buttons), 0b0001);
        assert_eq!(apply_buttons(0b010, &buttons), 0b0010);
        assert_eq!(apply_buttons(0b100, &buttons), 0b0100);
    }

    #[test]
    fn test_apply_buttons_multiple_xor() {
        let buttons = vec![0b0011, 0b0110];
        assert_eq!(apply_buttons(0b11, &buttons), 0b0101);
    }

    #[test]
    fn test_min_presses_example1() {
        let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(min_presses(&machine), 2);
    }

    #[test]
    fn test_min_presses_example2() {
        let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(min_presses(&machine), 3);
    }

    #[test]
    fn test_min_presses_example3() {
        let machine =
            parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(min_presses(&machine), 2);
    }

    #[test]
    fn test_min_presses_single_button() {
        let machine = Machine {
            target: 0b0110,
            buttons: vec![0b0110],
        };
        assert_eq!(min_presses(&machine), 1);
    }

    #[test]
    fn test_min_presses_all_off_target() {
        let machine = Machine {
            target: 0,
            buttons: vec![0b0001, 0b0010],
        };
        assert_eq!(min_presses(&machine), 0);
    }

    #[test]
    fn test_parse_machine_complete() {
        let machine = parse_machine("[.##.] (3) (1,3) {5}");
        assert_eq!(machine.target, 0b0110);
        assert_eq!(machine.buttons, vec![0b1000, 0b1010]);
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve_part1(""), 0);
    }

    #[test]
    fn test_solve_single_machine() {
        assert_eq!(
            solve_part1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            2
        );
    }

    #[test]
    fn test_main() {
        main();
    }

    // Part 2 tests

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 33);
    }

    #[test]
    fn test_parse_joltage() {
        assert_eq!(parse_joltage("[.] (1) {3,5,4,7}"), vec![3, 5, 4, 7]);
        assert_eq!(parse_joltage("[.] (1) {7,5,12,7,2}"), vec![7, 5, 12, 7, 2]);
    }

    #[test]
    fn test_parse_buttons_indices() {
        let buttons = parse_buttons_indices("[.] (3) (1,3) (0,2) {1}");
        assert_eq!(buttons, vec![vec![3], vec![1, 3], vec![0, 2]]);
    }

    #[test]
    fn test_min_presses_part2_example1() {
        let buttons = parse_buttons_indices("[.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let targets = parse_joltage("[.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(min_presses_gauss(&buttons, &targets), 10);
    }

    #[test]
    fn test_min_presses_part2_example2() {
        let line = "[.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let buttons = parse_buttons_indices(line);
        let targets = parse_joltage(line);
        assert_eq!(min_presses_gauss(&buttons, &targets), 12);
    }

    #[test]
    fn test_min_presses_part2_example3() {
        let line = "[.] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let buttons = parse_buttons_indices(line);
        let targets = parse_joltage(line);
        assert_eq!(min_presses_gauss(&buttons, &targets), 11);
    }

    #[test]
    fn test_min_presses_part2_single_button() {
        let buttons = vec![vec![0, 1]];
        let targets = vec![5, 5];
        assert_eq!(min_presses_gauss(&buttons, &targets), 5);
    }

    #[test]
    fn test_min_presses_part2_all_zeros() {
        let buttons = vec![vec![0], vec![1]];
        let targets = vec![0, 0];
        assert_eq!(min_presses_gauss(&buttons, &targets), 0);
    }

    #[test]
    fn test_solve_part2_empty() {
        assert_eq!(solve_part2(""), 0);
    }

    #[test]
    fn test_solve_part2_single_machine() {
        assert_eq!(
            solve_part2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            10
        );
    }
}
