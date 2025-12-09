fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    solve_with_connections(input, 1000)
}

fn solve_part2(input: &str) -> i64 {
    let coords = parse_coordinates(input);
    if coords.len() < 2 {
        return 0;
    }

    let pairs = generate_sorted_pairs(&coords);
    let mut uf = UnionFind::new(coords.len());

    let mut last_i = 0;
    let mut last_j = 0;

    for (i, j, _dist) in pairs {
        // Only track connections that actually merge two circuits
        if uf.union(i, j) {
            last_i = i;
            last_j = j;
        }

        // Stop when all boxes are in one circuit
        if uf.circuit_count() == 1 {
            break;
        }
    }

    coords[last_i].0 * coords[last_j].0
}

fn solve_with_connections(input: &str, num_connections: usize) -> u64 {
    let coords = parse_coordinates(input);
    if coords.len() < 2 {
        return if coords.len() == 1 { 1 } else { 0 };
    }

    let pairs = generate_sorted_pairs(&coords);
    let mut uf = UnionFind::new(coords.len());

    for (i, j, _dist) in pairs.into_iter().take(num_connections) {
        // Attempt connection (may be redundant if already in same circuit)
        uf.union(i, j);
    }

    let sizes = uf.circuit_sizes();
    let mut sorted_sizes: Vec<usize> = sizes.into_iter().collect();
    sorted_sizes.sort_unstable_by(|a, b| b.cmp(a));

    sorted_sizes.iter().take(3).map(|&s| s as u64).product()
}

fn parse_coordinates(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<i64> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            if parts.len() == 3 {
                Some((parts[0], parts[1], parts[2]))
            } else {
                None
            }
        })
        .collect()
}

fn distance_squared(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

fn generate_sorted_pairs(coords: &[(i64, i64, i64)]) -> Vec<(usize, usize, i64)> {
    let mut pairs = Vec::new();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dist = distance_squared(coords[i], coords[j]);
            pairs.push((i, j, dist));
        }
    }
    pairs.sort_unstable_by_key(|&(_, _, dist)| dist);
    pairs
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }
        true
    }

    fn circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }

    fn circuit_count(&mut self) -> usize {
        let n = self.parent.len();
        (0..n).filter(|&i| self.find(i) == i).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_with_connections(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_parse_coordinates() {
        let coords = parse_coordinates("1,2,3\n4,5,6");
        assert_eq!(coords, vec![(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn test_parse_coordinates_empty() {
        let coords = parse_coordinates("");
        assert_eq!(coords, vec![]);
    }

    #[test]
    fn test_parse_coordinates_with_empty_lines() {
        let coords = parse_coordinates("1,2,3\n\n4,5,6\n");
        assert_eq!(coords, vec![(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn test_parse_coordinates_invalid() {
        // Only 2 values - should be skipped
        let coords = parse_coordinates("1,2,3\n1,2\n4,5,6");
        assert_eq!(coords, vec![(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn test_distance_squared() {
        assert_eq!(distance_squared((0, 0, 0), (1, 0, 0)), 1);
        assert_eq!(distance_squared((0, 0, 0), (1, 1, 1)), 3);
        assert_eq!(distance_squared((0, 0, 0), (3, 4, 0)), 25);
    }

    #[test]
    fn test_distance_squared_negative() {
        assert_eq!(distance_squared((5, 5, 5), (2, 1, 5)), 25);
    }

    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert_eq!(uf.find(0), uf.find(1));
        assert_ne!(uf.find(0), uf.find(2));
    }

    #[test]
    fn test_union_find_same_circuit() {
        let mut uf = UnionFind::new(3);
        assert!(uf.union(0, 1));
        assert!(uf.union(1, 2));
        assert!(!uf.union(0, 2)); // Already connected
    }

    #[test]
    fn test_union_find_sizes() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        let sizes = uf.circuit_sizes();
        assert!(sizes.contains(&3));
        assert!(sizes.contains(&1));
        assert_eq!(sizes.len(), 3);
    }

    #[test]
    fn test_generate_sorted_pairs() {
        let coords = vec![(0, 0, 0), (1, 0, 0), (10, 0, 0)];
        let pairs = generate_sorted_pairs(&coords);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0], (0, 1, 1)); // Closest pair first
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve_with_connections("", 10), 0);
    }

    #[test]
    fn test_solve_single_box() {
        assert_eq!(solve_with_connections("1,2,3", 10), 1);
    }

    #[test]
    fn test_solve_two_boxes() {
        assert_eq!(solve_with_connections("0,0,0\n1,1,1", 1), 2);
    }

    #[test]
    fn test_solve_no_connections() {
        // 3 boxes, 0 connections = 3 circuits of size 1 = 1*1*1 = 1
        assert_eq!(solve_with_connections("0,0,0\n1,0,0\n2,0,0", 0), 1);
    }

    #[test]
    fn test_solve_all_connected() {
        // 3 boxes, 2+ connections = 1 circuit of size 3
        // Product of top 3 sizes: 3 * 1 * 1 (padding with 1s) = 3
        // Actually only 1 circuit exists, so it's just 3
        let result = solve_with_connections("0,0,0\n1,0,0\n2,0,0", 3);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_example() {
        // Last connection is between 216,146,977 and 117,168,530
        // Product of X coords: 216 * 117 = 25272
        assert_eq!(solve_part2(EXAMPLE), 25272);
    }

    #[test]
    fn test_part2_two_boxes() {
        // Two boxes: first connection unites them
        // X coords: 5 * 10 = 50
        assert_eq!(solve_part2("5,0,0\n10,0,0"), 50);
    }

    #[test]
    fn test_part2_empty() {
        assert_eq!(solve_part2(""), 0);
    }

    #[test]
    fn test_part2_single_box() {
        assert_eq!(solve_part2("1,2,3"), 0);
    }

    #[test]
    fn test_circuit_count() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.circuit_count(), 5);
        uf.union(0, 1);
        assert_eq!(uf.circuit_count(), 4);
        uf.union(2, 3);
        assert_eq!(uf.circuit_count(), 3);
        uf.union(0, 2);
        assert_eq!(uf.circuit_count(), 2);
    }

    #[test]
    fn test_main() {
        main();
    }
}
