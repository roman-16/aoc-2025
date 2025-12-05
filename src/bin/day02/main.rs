fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .flat_map(parse_range)
        .filter(|&n| is_doubled(n))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .flat_map(parse_range)
        .filter(|&n| is_repeated(n))
        .sum()
}

fn is_repeated(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len.is_multiple_of(pattern_len) {
            let pattern = &s[..pattern_len];
            if s.as_bytes()
                .chunks(pattern_len)
                .all(|chunk| chunk == pattern.as_bytes())
            {
                return true;
            }
        }
    }

    false
}

fn parse_range(range: &str) -> std::ops::RangeInclusive<u64> {
    let (start, end) = range
        .split_once('-')
        .expect("Range must contain a dash separator");
    let start: u64 = start.parse().expect("Start must be a valid number");
    let end: u64 = end.parse().expect("End must be a valid number");
    start..=end
}

fn is_doubled(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    s[..mid] == s[mid..]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_doubled_two_digit() {
        assert!(is_doubled(11));
        assert!(is_doubled(22));
        assert!(is_doubled(99));
    }

    #[test]
    fn test_is_doubled_four_digit() {
        assert!(is_doubled(1010));
        assert!(is_doubled(6464));
        assert!(is_doubled(1212));
    }

    #[test]
    fn test_is_doubled_six_digit() {
        assert!(is_doubled(123123));
        assert!(is_doubled(222222));
        assert!(is_doubled(446446));
    }

    #[test]
    fn test_is_doubled_large_numbers() {
        assert!(is_doubled(1188511885));
        assert!(is_doubled(38593859));
    }

    #[test]
    fn test_is_doubled_false_odd_length() {
        assert!(!is_doubled(1));
        assert!(!is_doubled(101));
        assert!(!is_doubled(12345));
    }

    #[test]
    fn test_is_doubled_false_even_length_not_doubled() {
        assert!(!is_doubled(12));
        assert!(!is_doubled(1234));
        assert!(!is_doubled(123456));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part1_single_range() {
        assert_eq!(solve_part1("11-22"), 11 + 22);
    }

    #[test]
    fn test_part1_no_invalid_ids() {
        assert_eq!(solve_part1("1698522-1698528"), 0);
    }

    #[test]
    fn test_is_repeated_twice() {
        assert!(is_repeated(11));
        assert!(is_repeated(1010));
        assert!(is_repeated(12341234));
    }

    #[test]
    fn test_is_repeated_three_times() {
        assert!(is_repeated(111));
        assert!(is_repeated(123123123));
        assert!(is_repeated(999));
    }

    #[test]
    fn test_is_repeated_many_times() {
        assert!(is_repeated(1111111)); // 1 seven times
        assert!(is_repeated(1212121212)); // 12 five times
        assert!(is_repeated(565656)); // 56 three times
        assert!(is_repeated(824824824)); // 824 three times
        assert!(is_repeated(2121212121)); // 21 five times
    }

    #[test]
    fn test_is_repeated_false() {
        assert!(!is_repeated(12));
        assert!(!is_repeated(123));
        assert!(!is_repeated(1234));
        assert!(!is_repeated(12345));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 4174379265);
    }

    #[test]
    #[should_panic(expected = "Range must contain a dash separator")]
    fn test_parse_range_no_dash() {
        solve_part1("1234");
    }

    #[test]
    #[should_panic(expected = "Start must be a valid number")]
    fn test_parse_range_invalid_start() {
        solve_part1("abc-123");
    }

    #[test]
    #[should_panic(expected = "End must be a valid number")]
    fn test_parse_range_invalid_end() {
        solve_part1("123-abc");
    }

    #[test]
    fn test_main() {
        main();
    }
}
