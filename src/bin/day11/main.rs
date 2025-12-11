use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    let graph = parse_graph(input);
    count_paths(&graph, "you", &mut HashMap::new())
}

fn solve_part2(input: &str) -> u64 {
    let graph = parse_graph(input);
    count_paths_constrained(&graph, "svr", false, false, &mut HashMap::new())
}

fn count_paths_constrained<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    has_dac: bool,
    has_fft: bool,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    let has_dac = has_dac || node == "dac";
    let has_fft = has_fft || node == "fft";

    if node == "out" {
        return if has_dac && has_fft { 1 } else { 0 };
    }

    let state = (node, has_dac, has_fft);
    if let Some(&count) = memo.get(&state) {
        return count;
    }

    let count = graph
        .get(node)
        .map(|children| {
            children
                .iter()
                .map(|child| count_paths_constrained(graph, child, has_dac, has_fft, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(state, count);
    count
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.split(": ");
            let node = parts.next()?;
            let children: Vec<&str> = parts.next()?.split_whitespace().collect();
            Some((node, children))
        })
        .collect()
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if node == "out" {
        return 1;
    }

    if let Some(&count) = memo.get(node) {
        return count;
    }

    let count = graph
        .get(node)
        .map(|children| {
            children
                .iter()
                .map(|child| count_paths(graph, child, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(node, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 5);
    }

    #[test]
    fn test_parse_graph() {
        let graph = parse_graph("a: b c\nd: e");
        assert_eq!(graph.get("a"), Some(&vec!["b", "c"]));
        assert_eq!(graph.get("d"), Some(&vec!["e"]));
    }

    #[test]
    fn test_parse_graph_empty() {
        let graph = parse_graph("");
        assert!(graph.is_empty());
    }

    #[test]
    fn test_parse_graph_with_empty_lines() {
        let graph = parse_graph("a: b\n\nc: d");
        assert_eq!(graph.len(), 2);
    }

    #[test]
    fn test_count_paths_direct() {
        let graph = parse_graph("you: out");
        assert_eq!(count_paths(&graph, "you", &mut HashMap::new()), 1);
    }

    #[test]
    fn test_count_paths_two_paths() {
        let graph = parse_graph("you: a b\na: out\nb: out");
        assert_eq!(count_paths(&graph, "you", &mut HashMap::new()), 2);
    }

    #[test]
    fn test_count_paths_no_path() {
        let graph = parse_graph("you: a\na: b");
        assert_eq!(count_paths(&graph, "you", &mut HashMap::new()), 0);
    }

    #[test]
    fn test_count_paths_shared_node() {
        // you -> a -> c -> out
        // you -> b -> c -> out
        let graph = parse_graph("you: a b\na: c\nb: c\nc: out");
        assert_eq!(count_paths(&graph, "you", &mut HashMap::new()), 2);
    }

    #[test]
    fn test_count_paths_at_out() {
        let graph = parse_graph("");
        assert_eq!(count_paths(&graph, "out", &mut HashMap::new()), 1);
    }

    #[test]
    fn test_count_paths_unknown_node() {
        let graph = parse_graph("a: b");
        assert_eq!(count_paths(&graph, "unknown", &mut HashMap::new()), 0);
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve_part1(""), 0);
    }

    #[test]
    fn test_main() {
        main();
    }

    // Part 2 tests

    const EXAMPLE_PART2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_PART2), 2);
    }

    #[test]
    fn test_part2_empty() {
        assert_eq!(solve_part2(""), 0);
    }

    #[test]
    fn test_part2_direct_with_both() {
        // svr -> dac -> fft -> out
        let input = "svr: dac\ndac: fft\nfft: out";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_part2_direct_missing_dac() {
        // svr -> fft -> out (no dac)
        let input = "svr: fft\nfft: out";
        assert_eq!(solve_part2(input), 0);
    }

    #[test]
    fn test_part2_direct_missing_fft() {
        // svr -> dac -> out (no fft)
        let input = "svr: dac\ndac: out";
        assert_eq!(solve_part2(input), 0);
    }

    #[test]
    fn test_part2_two_paths_one_valid() {
        // Path 1: svr -> a -> out (invalid - missing both)
        // Path 2: svr -> dac -> fft -> out (valid)
        let input = "svr: a dac\na: out\ndac: fft\nfft: out";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_part2_fft_before_dac() {
        // svr -> fft -> dac -> out
        let input = "svr: fft\nfft: dac\ndac: out";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_count_paths_constrained_at_out_with_both() {
        let graph = parse_graph("");
        assert_eq!(
            count_paths_constrained(&graph, "out", true, true, &mut HashMap::new()),
            1
        );
    }

    #[test]
    fn test_count_paths_constrained_at_out_missing_one() {
        let graph = parse_graph("");
        assert_eq!(
            count_paths_constrained(&graph, "out", true, false, &mut HashMap::new()),
            0
        );
        assert_eq!(
            count_paths_constrained(&graph, "out", false, true, &mut HashMap::new()),
            0
        );
    }

    #[test]
    fn test_count_paths_constrained_unknown_node() {
        let graph = parse_graph("a: b");
        assert_eq!(
            count_paths_constrained(&graph, "unknown", true, true, &mut HashMap::new()),
            0
        );
    }
}
