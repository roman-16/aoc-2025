fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter().filter(|&&id| is_fresh(id, &ranges)).count()
}

fn solve_part2(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);
    merged.iter().map(|&(start, end)| end - start + 1).sum()
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            // Overlapping or adjacent - extend current range
            current.1 = current.1.max(end);
        } else {
            // Gap - push current and start new
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut sections = input.trim().split("\n\n");

    let ranges_section = sections.next().expect("Missing ranges section");
    let ids_section = sections.next().expect("Missing IDs section");

    let ranges: Vec<(u64, u64)> = ranges_section.lines().map(parse_range).collect();

    let ids: Vec<u64> = ids_section
        .lines()
        .map(|line| line.parse().expect("Invalid ID"))
        .collect();

    (ranges, ids)
}

fn parse_range(line: &str) -> (u64, u64) {
    let (start, end) = line
        .split_once('-')
        .expect("Range must contain a dash separator");
    let start: u64 = start.parse().expect("Start must be a valid number");
    let end: u64 = end.parse().expect("End must be a valid number");
    (start, end)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 3);
    }

    #[test]
    fn test_is_fresh_in_range() {
        let ranges = vec![(3, 5), (10, 14)];
        assert!(is_fresh(3, &ranges));
        assert!(is_fresh(4, &ranges));
        assert!(is_fresh(5, &ranges));
        assert!(is_fresh(10, &ranges));
        assert!(is_fresh(14, &ranges));
    }

    #[test]
    fn test_is_fresh_not_in_range() {
        let ranges = vec![(3, 5), (10, 14)];
        assert!(!is_fresh(1, &ranges));
        assert!(!is_fresh(2, &ranges));
        assert!(!is_fresh(6, &ranges));
        assert!(!is_fresh(9, &ranges));
        assert!(!is_fresh(15, &ranges));
    }

    #[test]
    fn test_is_fresh_overlapping_ranges() {
        let ranges = vec![(10, 14), (12, 18)];
        assert!(is_fresh(13, &ranges)); // In both ranges
        assert!(is_fresh(17, &ranges)); // Only in second range
        assert!(is_fresh(11, &ranges)); // Only in first range
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("3-5"), (3, 5));
        assert_eq!(parse_range("10-14"), (10, 14));
        assert_eq!(parse_range("100-200"), (100, 200));
    }

    #[test]
    fn test_parse_input() {
        let (ranges, ids) = parse_input(EXAMPLE);
        assert_eq!(ranges, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_single_range_single_id() {
        let input = "5-10\n\n7";
        assert_eq!(solve_part1(input), 1);
    }

    #[test]
    fn test_single_range_id_outside() {
        let input = "5-10\n\n3";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_boundary_exact_match() {
        let input = "5-5\n\n5";
        assert_eq!(solve_part1(input), 1);
    }

    #[test]
    #[should_panic(expected = "Range must contain a dash separator")]
    fn test_invalid_range_no_dash() {
        parse_range("510");
    }

    #[test]
    #[should_panic(expected = "Start must be a valid number")]
    fn test_invalid_range_bad_start() {
        parse_range("abc-10");
    }

    #[test]
    #[should_panic(expected = "End must be a valid number")]
    fn test_invalid_range_bad_end() {
        parse_range("5-xyz");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 14);
    }

    #[test]
    fn test_merge_ranges_no_overlap() {
        let ranges = vec![(1, 3), (5, 7), (10, 12)];
        assert_eq!(merge_ranges(ranges), vec![(1, 3), (5, 7), (10, 12)]);
    }

    #[test]
    fn test_merge_ranges_overlap() {
        let ranges = vec![(1, 5), (3, 8)];
        assert_eq!(merge_ranges(ranges), vec![(1, 8)]);
    }

    #[test]
    fn test_merge_ranges_adjacent() {
        let ranges = vec![(1, 5), (6, 10)];
        assert_eq!(merge_ranges(ranges), vec![(1, 10)]);
    }

    #[test]
    fn test_merge_ranges_unsorted() {
        let ranges = vec![(10, 14), (3, 5), (12, 18), (16, 20)];
        assert_eq!(merge_ranges(ranges), vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn test_merge_ranges_contained() {
        let ranges = vec![(1, 10), (3, 5)];
        assert_eq!(merge_ranges(ranges), vec![(1, 10)]);
    }

    #[test]
    fn test_merge_ranges_empty() {
        let ranges: Vec<(u64, u64)> = vec![];
        assert_eq!(merge_ranges(ranges), vec![]);
    }

    #[test]
    fn test_part2_single_range() {
        let input = "5-10\n\n7";
        assert_eq!(solve_part2(input), 6);
    }

    #[test]
    fn test_main() {
        main();
    }
}
