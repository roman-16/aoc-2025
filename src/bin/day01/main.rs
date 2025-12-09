fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn parse_moves(input: &str) -> impl Iterator<Item = (bool, i32)> + '_ {
    input.lines().map(|line| {
        let (direction, distance) = line.split_at(1);
        let is_left = match direction {
            "L" => true,
            "R" => false,
            _ => panic!("Invalid direction: {direction}"),
        };
        (is_left, distance.parse().unwrap())
    })
}

fn apply_move(position: i32, distance: i32, is_left: bool) -> i32 {
    if is_left {
        (position - distance).rem_euclid(100)
    } else {
        (position + distance).rem_euclid(100)
    }
}

fn solve_part1(input: &str) -> usize {
    let mut position: i32 = 50;
    let mut count = 0;

    for (is_left, distance) in parse_moves(input) {
        position = apply_move(position, distance, is_left);
        if position == 0 {
            count += 1;
        }
    }

    count
}

fn solve_part2(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut count = 0;

    for (is_left, distance) in parse_moves(input) {
        count += count_zeros(position, distance, is_left);
        position = apply_move(position, distance, is_left);
    }

    count
}

fn count_zeros(position: i32, distance: i32, is_left: bool) -> i32 {
    let first_k = if is_left {
        if position == 0 {
            100
        } else {
            position
        }
    } else if position == 0 {
        100
    } else {
        100 - position
    };

    if first_k > distance {
        0
    } else {
        (distance - first_k) / 100 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 6);
    }

    #[test]
    fn test_part2_large_rotation() {
        assert_eq!(solve_part2("R1000"), 10);
    }

    #[test]
    #[should_panic(expected = "Invalid direction")]
    fn test_part1_invalid_direction() {
        solve_part1("X50");
    }

    #[test]
    #[should_panic(expected = "Invalid direction")]
    fn test_part2_invalid_direction() {
        solve_part2("X50");
    }

    #[test]
    fn test_main() {
        main();
    }
}
