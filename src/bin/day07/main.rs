use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let (start_row, start_col) = find_start(&lines);
    simulate_beams(&lines, start_row, start_col)
}

fn solve_part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let (start_row, start_col) = find_start(&lines);
    simulate_timelines(&lines, start_row, start_col)
}

fn find_start(lines: &[&str]) -> (usize, usize) {
    for (row, line) in lines.iter().enumerate() {
        if let Some(col) = line.find('S') {
            return (row, col);
        }
    }
    panic!("No start position 'S' found in input");
}

fn simulate_beams(lines: &[&str], start_row: usize, start_col: usize) -> usize {
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut split_count = 0;
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for line in lines.iter().skip(start_row + 1) {
        let mut new_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            let char_at = line.chars().nth(col).unwrap_or('.');

            if char_at == '^' {
                split_count += 1;
                // Spawn beams to left and right
                if col > 0 {
                    new_beams.insert(col - 1);
                }
                if col + 1 < width {
                    new_beams.insert(col + 1);
                }
            } else {
                // Beam continues downward
                new_beams.insert(col);
            }
        }

        beams = new_beams;

        if beams.is_empty() {
            break;
        }
    }

    split_count
}

fn simulate_timelines(lines: &[&str], start_row: usize, start_col: usize) -> u64 {
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start_col, 1);

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for line in lines.iter().skip(start_row + 1) {
        let mut new_timelines: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &timelines {
            let char_at = line.chars().nth(col).unwrap_or('.');

            if char_at == '^' {
                // Split: each timeline at this position creates two new timelines
                if col > 0 {
                    *new_timelines.entry(col - 1).or_insert(0) += count;
                }
                if col + 1 < width {
                    *new_timelines.entry(col + 1).or_insert(0) += count;
                }
            } else {
                // Continue: timelines pass through
                *new_timelines.entry(col).or_insert(0) += count;
            }
        }

        timelines = new_timelines;

        if timelines.is_empty() {
            break;
        }
    }

    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 21);
    }

    #[test]
    fn test_find_start() {
        assert_eq!(find_start(&[".......S......."]), (0, 7));
        assert_eq!(find_start(&["...", ".S.", "..."]), (1, 1));
    }

    #[test]
    #[should_panic(expected = "No start position")]
    fn test_find_start_missing() {
        find_start(&["...", "...", "..."]);
    }

    #[test]
    fn test_single_splitter() {
        let input = "S\n.\n^\n.";
        assert_eq!(solve_part1(input), 1);
    }

    #[test]
    fn test_no_splitters() {
        let input = "S\n.\n.\n.";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_beam_merging() {
        // Two splitters side by side, beams merge in the middle
        let input = "\
..S..
.....
..^..
.....
.^.^.
.....";
        // First split at row 2, creates beams at col 1 and 3
        // Second row has splitters at col 1 and 3
        // Each beam hits a splitter = 2 more splits
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_beam_exits_edge() {
        // Beam goes off left edge
        let input = "\
S...
....
^...
....";
        // Splitter at col 0, beam splits to col -1 (exits) and col 1
        assert_eq!(solve_part1(input), 1);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(solve_part1(""), 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 40);
    }

    #[test]
    fn test_part2_single_splitter() {
        // One split = 2 timelines (need wider grid so beams don't exit)
        let input = ".S.\n...\n.^.\n...";
        assert_eq!(solve_part2(input), 2);
    }

    #[test]
    fn test_part2_no_splitters() {
        // No splits = 1 timeline
        let input = "S\n.\n.\n.";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_part2_chain_splits() {
        // Two sequential splitters on same path = 4 timelines
        let input = "\
..S..
.....
..^..
.....
.^.^.
.....";
        // First split: 2 timelines (col 1 and col 3)
        // Second split at col 1: 2 more (col 0 and col 2)
        // Second split at col 3: 2 more (col 2 and col 4)
        // Total: 4 timelines at cols 0, 2, 2, 4 = positions 0(1), 2(2), 4(1)
        assert_eq!(solve_part2(input), 4);
    }

    #[test]
    fn test_part2_empty_input() {
        assert_eq!(solve_part2(""), 0);
    }

    #[test]
    fn test_part2_edge_exit() {
        // Splitter at edge, one beam exits
        let input = "\
S...
....
^...
....";
        // Split at col 0: left exits (lost), right continues = 1 timeline
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_main() {
        main();
    }
}
