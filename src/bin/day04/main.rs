fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.trim().lines().map(|line| line.as_bytes()).collect();
    let cols = grid.first().map_or(0, |row| row.len());

    (0..grid.len())
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid[row][col] == b'@' && count_adjacent_rolls(&grid, row, col) < 4)
        .count()
}

fn solve_part2(input: &str) -> usize {
    let mut grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_rolls(&grid);
        if accessible.is_empty() {
            break;
        }

        for (row, col) in &accessible {
            grid[*row][*col] = b'.';
        }
        total_removed += accessible.len();
    }

    total_removed
}

fn find_accessible_rolls(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let cols = grid.first().map_or(0, |row| row.len());

    (0..grid.len())
        .flat_map(|row| (0..cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid[row][col] == b'@' && count_adjacent_rolls(grid, row, col) < 4)
        .collect()
}

fn count_adjacent_rolls<R: AsRef<[u8]>>(grid: &[R], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid.first().map_or(0, |r| r.as_ref().len());

    (-1i32..=1)
        .flat_map(|dr| (-1i32..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| dr != 0 || dc != 0)
        .filter_map(|(dr, dc)| {
            let new_row = row.checked_add_signed(dr as isize)?;
            let new_col = col.checked_add_signed(dc as isize)?;
            (new_row < rows && new_col < cols).then_some((new_row, new_col))
        })
        .filter(|&(r, c)| grid[r].as_ref()[c] == b'@')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 13);
    }

    #[test]
    fn test_count_adjacent_corner_top_left() {
        let grid: Vec<&[u8]> = vec![b"@.", b".."];
        assert_eq!(count_adjacent_rolls(&grid, 0, 0), 0);
    }

    #[test]
    fn test_count_adjacent_corner_with_neighbors() {
        let grid: Vec<&[u8]> = vec![b"@@", b"@."];
        assert_eq!(count_adjacent_rolls(&grid, 0, 0), 2);
    }

    #[test]
    fn test_count_adjacent_center_surrounded() {
        let grid: Vec<&[u8]> = vec![b"@@@", b"@@@", b"@@@"];
        assert_eq!(count_adjacent_rolls(&grid, 1, 1), 8);
    }

    #[test]
    fn test_count_adjacent_center_no_neighbors() {
        let grid: Vec<&[u8]> = vec![b"...", b".@.", b"..."];
        assert_eq!(count_adjacent_rolls(&grid, 1, 1), 0);
    }

    #[test]
    fn test_single_roll_accessible() {
        let input = "@";
        assert_eq!(solve_part1(input), 1);
    }

    #[test]
    fn test_all_empty() {
        let input = "...\n...\n...";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_roll_with_exactly_three_neighbors() {
        // Center roll has exactly 3 neighbors - should be accessible
        let input = ".@.\n@@.\n...";
        // Top-middle: 2 neighbors (accessible)
        // Middle-left: 2 neighbors (accessible)
        // Middle-middle: not a roll
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_roll_with_exactly_four_neighbors() {
        // Center roll has exactly 4 neighbors - NOT accessible
        let input = ".@.\n@@@\n.@.";
        // Center has 4 neighbors, not accessible
        // Each edge roll has 1 neighbor, accessible
        assert_eq!(solve_part1(input), 4);
    }

    #[test]
    fn test_edge_roll() {
        let grid: Vec<&[u8]> = vec![b".@.", b"..."];
        assert_eq!(count_adjacent_rolls(&grid, 0, 1), 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 43);
    }

    #[test]
    fn test_part2_single_roll() {
        assert_eq!(solve_part2("@"), 1);
    }

    #[test]
    fn test_part2_all_empty() {
        assert_eq!(solve_part2("...\n...\n..."), 0);
    }

    #[test]
    fn test_part2_chain_removal() {
        // A line of rolls - first the ends are accessible, then progressively inward
        let input = "@@@@@";
        // Initially: ends have 1 neighbor each (accessible)
        // After removing ends: new ends have 1 neighbor each
        // Continue until all removed
        assert_eq!(solve_part2(input), 5);
    }

    #[test]
    fn test_part2_dense_block_partial() {
        // 3x3 block - center has 8 neighbors, never accessible
        // But outer rolls have fewer neighbors
        let input = "@@@\n@@@\n@@@";
        // Corner: 3 neighbors (accessible)
        // Edge: 5 neighbors (not accessible initially)
        // Center: 8 neighbors (not accessible)
        // After corners removed, edges become accessible, then center
        assert_eq!(solve_part2(input), 9);
    }

    #[test]
    fn test_main() {
        main();
    }
}
