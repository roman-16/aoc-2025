fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    parse_problems(input)
        .iter()
        .map(|(numbers, operator)| solve_problem(numbers, *operator))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    parse_problems_part2(input)
        .iter()
        .map(|(numbers, operator)| solve_problem(numbers, *operator))
        .sum()
}

fn solve_problem(numbers: &[u64], operator: char) -> u64 {
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unknown operator: {}", operator),
    }
}

fn parse_problems_part2(input: &str) -> Vec<(Vec<u64>, char)> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // For each column, determine if it's a separator (all spaces)
    let is_separator: Vec<bool> = (0..max_len)
        .map(|col| {
            lines
                .iter()
                .all(|line| line.chars().nth(col).is_none_or(|c| c == ' '))
        })
        .collect();

    // Find problem regions (runs of non-separator columns)
    let mut problems = vec![];
    let mut start = None;

    for (col, &is_sep) in is_separator.iter().enumerate() {
        match (start, is_sep) {
            (None, false) => start = Some(col),
            (Some(s), true) => {
                problems.push(extract_problem_part2(&lines, s, col));
                start = None;
            }
            _ => {}
        }
    }

    // Handle last problem if input doesn't end with separator
    if let Some(s) = start {
        problems.push(extract_problem_part2(&lines, s, max_len));
    }

    problems
}

/// Extract a problem for part 2: each column is a number (digits top-to-bottom),
/// columns are read right-to-left
fn extract_problem_part2(lines: &[&str], start_col: usize, end_col: usize) -> (Vec<u64>, char) {
    let num_rows = lines.len() - 1;
    let operator_row = lines[num_rows];

    // Read columns right-to-left, each column forms a number (digits top-to-bottom)
    let mut numbers = vec![];
    for col in (start_col..end_col).rev() {
        let digits: String = lines
            .iter()
            .take(num_rows)
            .filter_map(|line| line.chars().nth(col))
            .filter(|c| c.is_ascii_digit())
            .collect();
        if !digits.is_empty() {
            numbers.push(digits.parse().expect("Invalid number in column"));
        }
    }

    let operator = operator_row
        .chars()
        .skip(start_col)
        .take(end_col - start_col)
        .find(|c| !c.is_whitespace())
        .expect("No operator found in problem");

    (numbers, operator)
}

fn parse_problems(input: &str) -> Vec<(Vec<u64>, char)> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // For each column, determine if it's a separator (all spaces)
    let is_separator: Vec<bool> = (0..max_len)
        .map(|col| {
            lines
                .iter()
                .all(|line| line.chars().nth(col).is_none_or(|c| c == ' '))
        })
        .collect();

    // Find problem regions (runs of non-separator columns)
    let mut problems = vec![];
    let mut start = None;

    for (col, &is_sep) in is_separator.iter().enumerate() {
        match (start, is_sep) {
            (None, false) => start = Some(col),
            (Some(s), true) => {
                problems.push(extract_problem(&lines, s, col));
                start = None;
            }
            _ => {}
        }
    }

    // Handle last problem if input doesn't end with separator
    if let Some(s) = start {
        problems.push(extract_problem(&lines, s, max_len));
    }

    problems
}

fn extract_problem(lines: &[&str], start_col: usize, end_col: usize) -> (Vec<u64>, char) {
    let num_rows = lines.len() - 1;
    let operator_row = lines[num_rows];

    let mut numbers = vec![];
    for line in lines.iter().take(num_rows) {
        let num_str: String = line
            .chars()
            .skip(start_col)
            .take(end_col - start_col)
            .filter(|c| !c.is_whitespace())
            .collect();
        if !num_str.is_empty() {
            numbers.push(num_str.parse().expect("Invalid number in problem"));
        }
    }

    let operator = operator_row
        .chars()
        .skip(start_col)
        .take(end_col - start_col)
        .find(|c| !c.is_whitespace())
        .expect("No operator found in problem");

    (numbers, operator)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_parse_problems_example() {
        let problems = parse_problems(EXAMPLE);
        assert_eq!(problems.len(), 4);
        assert_eq!(problems[0], (vec![123, 45, 6], '*'));
        assert_eq!(problems[1], (vec![328, 64, 98], '+'));
        assert_eq!(problems[2], (vec![51, 387, 215], '*'));
        assert_eq!(problems[3], (vec![64, 23, 314], '+'));
    }

    #[test]
    fn test_solve_problem_multiply() {
        assert_eq!(solve_problem(&[123, 45, 6], '*'), 33210);
    }

    #[test]
    fn test_solve_problem_add() {
        assert_eq!(solve_problem(&[328, 64, 98], '+'), 490);
    }

    #[test]
    fn test_single_problem_multiply() {
        let input = "10\n20\n*";
        assert_eq!(solve_part1(input), 200);
    }

    #[test]
    fn test_single_problem_add() {
        let input = "10\n20\n+";
        assert_eq!(solve_part1(input), 30);
    }

    #[test]
    fn test_single_number() {
        let input = "42\n*";
        assert_eq!(solve_part1(input), 42);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(parse_problems(""), vec![]);
    }

    #[test]
    #[should_panic(expected = "Unknown operator")]
    fn test_unknown_operator() {
        solve_problem(&[1, 2], '-');
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 3263827);
    }

    #[test]
    fn test_parse_problems_part2_example() {
        let problems = parse_problems_part2(EXAMPLE);
        assert_eq!(problems.len(), 4);
        // Leftmost problem read column-wise right-to-left: 356 * 24 * 1
        assert_eq!(problems[0], (vec![356, 24, 1], '*'));
        // Second from left: 8 + 248 + 369
        assert_eq!(problems[1], (vec![8, 248, 369], '+'));
        // Third from left: 175 * 581 * 32
        assert_eq!(problems[2], (vec![175, 581, 32], '*'));
        // Rightmost: 4 + 431 + 623
        assert_eq!(problems[3], (vec![4, 431, 623], '+'));
    }

    #[test]
    fn test_part2_single_column() {
        let input = "1\n2\n3\n+";
        assert_eq!(solve_part2(input), 123);
    }

    #[test]
    fn test_part2_empty_input() {
        assert_eq!(parse_problems_part2(""), vec![]);
    }

    #[test]
    fn test_main() {
        main();
    }
}
