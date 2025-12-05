fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|bank| max_joltage_k(bank, 2))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|bank| max_joltage_k(bank, 12))
        .sum()
}

fn max_joltage_k(bank: &str, k: usize) -> u64 {
    let n = bank.len();
    assert!(
        n >= k,
        "Bank must have at least {} batteries, got {}: '{}'",
        k,
        n,
        bank
    );

    let digits: Vec<u8> = bank
        .bytes()
        .map(|b| {
            assert!(
                b.is_ascii_digit(),
                "Invalid character in bank: '{}'",
                b as char
            );
            b - b'0'
        })
        .collect();

    let mut drop = n - k;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for &d in &digits {
        while !stack.is_empty() && drop > 0 && *stack.last().unwrap() < d {
            stack.pop();
            drop -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);
    stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_example_1() {
        assert_eq!(max_joltage_k("987654321111111", 2), 98);
    }

    #[test]
    fn test_part1_example_2() {
        assert_eq!(max_joltage_k("811111111111119", 2), 89);
    }

    #[test]
    fn test_part1_example_3() {
        assert_eq!(max_joltage_k("234234234234278", 2), 78);
    }

    #[test]
    fn test_part1_example_4() {
        assert_eq!(max_joltage_k("818181911112111", 2), 92);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part1_two_digits() {
        assert_eq!(max_joltage_k("12", 2), 12);
        assert_eq!(max_joltage_k("91", 2), 91);
        assert_eq!(max_joltage_k("19", 2), 19);
    }

    #[test]
    fn test_part1_all_same() {
        assert_eq!(max_joltage_k("9999", 2), 99);
        assert_eq!(max_joltage_k("1111", 2), 11);
    }

    #[test]
    fn test_part1_descending() {
        assert_eq!(max_joltage_k("987654321", 2), 98);
    }

    #[test]
    fn test_part1_ascending() {
        assert_eq!(max_joltage_k("123456789", 2), 89);
    }

    #[test]
    #[should_panic(expected = "Bank must have at least 2 batteries")]
    fn test_part1_single_digit() {
        max_joltage_k("5", 2);
    }

    #[test]
    #[should_panic(expected = "Bank must have at least 2 batteries")]
    fn test_part1_empty() {
        max_joltage_k("", 2);
    }

    #[test]
    #[should_panic(expected = "Invalid character in bank")]
    fn test_invalid_char() {
        max_joltage_k("12a34", 2);
    }

    #[test]
    fn test_part2_example_1() {
        assert_eq!(max_joltage_k("987654321111111", 12), 987654321111);
    }

    #[test]
    fn test_part2_example_2() {
        assert_eq!(max_joltage_k("811111111111119", 12), 811111111119);
    }

    #[test]
    fn test_part2_example_3() {
        assert_eq!(max_joltage_k("234234234234278", 12), 434234234278);
    }

    #[test]
    fn test_part2_example_4() {
        assert_eq!(max_joltage_k("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn test_part2_exact_length() {
        assert_eq!(max_joltage_k("123456789012", 12), 123456789012);
    }

    #[test]
    #[should_panic(expected = "Bank must have at least 12 batteries")]
    fn test_part2_too_short() {
        max_joltage_k("12345678901", 12);
    }

    #[test]
    fn test_main() {
        main();
    }
}
