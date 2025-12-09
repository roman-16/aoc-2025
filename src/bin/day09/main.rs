use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> i64 {
    let coords = parse_coordinates(input);
    if coords.len() < 2 {
        return 0;
    }

    let mut max_area = 0;

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let area = rectangle_area(coords[i], coords[j]);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn parse_coordinates(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<i64> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            if parts.len() == 2 {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        })
        .collect()
}

fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let width = (p2.0 - p1.0).abs() + 1;
    let height = (p2.1 - p1.1).abs() + 1;
    width * height
}

fn solve_part2(input: &str) -> i64 {
    let red_tiles = parse_coordinates(input);
    if red_tiles.len() < 2 {
        return 0;
    }

    // Coordinate compression
    let (x_to_idx, idx_to_x) = compress_coords(red_tiles.iter().map(|p| p.0));
    let (y_to_idx, idx_to_y) = compress_coords(red_tiles.iter().map(|p| p.1));

    // Convert red tiles to compressed coordinates
    let compressed_red: Vec<(usize, usize)> = red_tiles
        .iter()
        .map(|p| (x_to_idx[&p.0], y_to_idx[&p.1]))
        .collect();

    // Build colored tiles in compressed space
    let colored = build_colored_compressed(&compressed_red, idx_to_x.len(), idx_to_y.len());

    // Find largest valid rectangle
    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let p1_comp = compressed_red[i];
            let p2_comp = compressed_red[j];

            if is_valid_rectangle_compressed(&colored, p1_comp, p2_comp) {
                let area = rectangle_area(red_tiles[i], red_tiles[j]);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn compress_coords<I: Iterator<Item = i64>>(coords: I) -> (HashMap<i64, usize>, Vec<i64>) {
    let mut unique: Vec<i64> = coords.collect();
    unique.sort();
    unique.dedup();

    let to_idx: HashMap<i64, usize> = unique.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    (to_idx, unique)
}

fn build_colored_compressed(
    red_tiles: &[(usize, usize)],
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut boundary: HashSet<(usize, usize)> = HashSet::new();

    // Add all red tiles
    for &tile in red_tiles {
        boundary.insert(tile);
    }

    // Connect consecutive red tiles with lines (wrapping)
    for i in 0..red_tiles.len() {
        let p1 = red_tiles[i];
        let p2 = red_tiles[(i + 1) % red_tiles.len()];
        add_line_compressed(&mut boundary, p1, p2);
    }

    // Fill interior using flood fill from outside
    fill_interior_compressed(&boundary, width, height)
}

fn add_line_compressed(
    colored: &mut HashSet<(usize, usize)>,
    p1: (usize, usize),
    p2: (usize, usize),
) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    if x1 == x2 {
        // Vertical line
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        for y in min_y..=max_y {
            colored.insert((x1, y));
        }
    } else {
        // Horizontal line
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        for x in min_x..=max_x {
            colored.insert((x, y1));
        }
    }
}

fn fill_interior_compressed(
    boundary: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    // Flood fill from outside (using padded grid)
    let mut outside: HashSet<(i32, i32)> = HashSet::new();
    let mut stack = vec![(-1_i32, -1_i32)];

    let w = width as i32;
    let h = height as i32;

    while let Some((x, y)) = stack.pop() {
        if x < -1 || x > w || y < -1 || y > h {
            continue;
        }
        if outside.contains(&(x, y)) {
            continue;
        }
        // Check if this is a boundary point (only for valid indices)
        if x >= 0 && x < w && y >= 0 && y < h && boundary.contains(&(x as usize, y as usize)) {
            continue;
        }
        outside.insert((x, y));
        stack.push((x - 1, y));
        stack.push((x + 1, y));
        stack.push((x, y - 1));
        stack.push((x, y + 1));
    }

    // All tiles in grid that are not outside are colored (boundary or interior)
    let mut colored: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            if !outside.contains(&(x as i32, y as i32)) {
                colored.insert((x, y));
            }
        }
    }
    colored
}

fn is_valid_rectangle_compressed(
    colored: &HashSet<(usize, usize)>,
    p1: (usize, usize),
    p2: (usize, usize),
) -> bool {
    let min_x = p1.0.min(p2.0);
    let max_x = p1.0.max(p2.0);
    let min_y = p1.1.min(p2.1);
    let max_y = p1.1.max(p2.1);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !colored.contains(&(x, y)) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 50);
    }

    #[test]
    fn test_rectangle_area_example_50() {
        assert_eq!(rectangle_area((2, 5), (11, 1)), 50);
    }

    #[test]
    fn test_rectangle_area_example_35() {
        assert_eq!(rectangle_area((7, 1), (11, 7)), 35);
    }

    #[test]
    fn test_rectangle_area_example_24() {
        assert_eq!(rectangle_area((2, 5), (9, 7)), 24);
    }

    #[test]
    fn test_rectangle_area_example_6() {
        assert_eq!(rectangle_area((7, 3), (2, 3)), 6);
    }

    #[test]
    fn test_rectangle_area_same_point() {
        assert_eq!(rectangle_area((5, 5), (5, 5)), 1);
    }

    #[test]
    fn test_rectangle_area_same_column() {
        assert_eq!(rectangle_area((3, 1), (3, 5)), 5);
    }

    #[test]
    fn test_parse_coordinates() {
        let coords = parse_coordinates("1,2\n3,4");
        assert_eq!(coords, vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn test_parse_coordinates_empty() {
        let coords = parse_coordinates("");
        assert_eq!(coords, vec![]);
    }

    #[test]
    fn test_parse_coordinates_with_empty_lines() {
        let coords = parse_coordinates("1,2\n\n3,4\n");
        assert_eq!(coords, vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn test_parse_coordinates_invalid() {
        let coords = parse_coordinates("1,2\n3\n4,5");
        assert_eq!(coords, vec![(1, 2), (4, 5)]);
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve_part1(""), 0);
    }

    #[test]
    fn test_solve_single_tile() {
        assert_eq!(solve_part1("5,5"), 0);
    }

    #[test]
    fn test_solve_two_tiles() {
        assert_eq!(solve_part1("0,0\n2,3"), 12);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 24);
    }

    #[test]
    fn test_part2_empty() {
        assert_eq!(solve_part2(""), 0);
    }

    #[test]
    fn test_part2_single_tile() {
        assert_eq!(solve_part2("5,5"), 0);
    }

    #[test]
    fn test_compress_coords() {
        let coords = vec![100, 50, 200, 50];
        let (to_idx, to_val) = compress_coords(coords.into_iter());
        assert_eq!(to_idx[&50], 0);
        assert_eq!(to_idx[&100], 1);
        assert_eq!(to_idx[&200], 2);
        assert_eq!(to_val, vec![50, 100, 200]);
    }

    #[test]
    fn test_add_line_compressed_horizontal() {
        let mut colored = HashSet::new();
        add_line_compressed(&mut colored, (1, 2), (4, 2));
        assert_eq!(colored.len(), 4);
        for x in 1..=4 {
            assert!(colored.contains(&(x, 2)));
        }
    }

    #[test]
    fn test_add_line_compressed_vertical() {
        let mut colored = HashSet::new();
        add_line_compressed(&mut colored, (2, 1), (2, 3));
        assert_eq!(colored.len(), 3);
        for y in 1..=3 {
            assert!(colored.contains(&(2, y)));
        }
    }

    #[test]
    fn test_is_valid_rectangle_compressed() {
        let mut colored = HashSet::new();
        for x in 0..=3 {
            for y in 0..=2 {
                colored.insert((x, y));
            }
        }
        assert!(is_valid_rectangle_compressed(&colored, (0, 0), (3, 2)));
        assert!(!is_valid_rectangle_compressed(&colored, (0, 0), (4, 2)));
    }

    #[test]
    fn test_main() {
        main();
    }
}
