fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
}

fn solve_part1(input: &str) -> usize {
    let (shape_sizes, regions) = parse_input(input);

    regions
        .iter()
        .filter(|region| {
            let total_cells: usize = region
                .counts
                .iter()
                .enumerate()
                .map(|(i, &count)| count * shape_sizes.get(i).unwrap_or(&0))
                .sum();
            let area = (region.width * region.height) as usize;
            total_cells < area
        })
        .count()
}

struct Region {
    width: i32,
    height: i32,
    counts: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Region>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut shape_sizes = Vec::new();
    let mut regions = Vec::new();

    for part in parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(first_line) = trimmed.lines().next() {
            if first_line.ends_with(':')
                && first_line[..first_line.len() - 1].parse::<usize>().is_ok()
            {
                let parsed = parse_shapes(part);
                for (i, size) in parsed.into_iter().enumerate() {
                    while shape_sizes.len() <= i {
                        shape_sizes.push(0);
                    }
                    if size > 0 {
                        shape_sizes[i] = size;
                    }
                }
            } else if first_line.contains('x') && first_line.contains(':') {
                regions.extend(parse_regions(part));
            }
        }
    }

    (shape_sizes, regions)
}

fn parse_shapes(input: &str) -> Vec<usize> {
    let mut shapes = Vec::new();
    let mut current_shape: Option<(usize, usize)> = None;

    for line in input.lines() {
        if let Some(idx_str) = line.strip_suffix(':') {
            if let Some((idx, count)) = current_shape.take() {
                while shapes.len() <= idx {
                    shapes.push(0);
                }
                shapes[idx] = count;
            }
            if let Ok(idx) = idx_str.parse::<usize>() {
                current_shape = Some((idx, 0));
            }
        } else if !line.is_empty() {
            if let Some((_, ref mut count)) = current_shape {
                *count += line.chars().filter(|&c| c == '#').count();
            }
        }
    }

    if let Some((idx, count)) = current_shape {
        while shapes.len() <= idx {
            shapes.push(0);
        }
        shapes[idx] = count;
    }

    shapes
}

fn parse_regions(input: &str) -> Vec<Region> {
    input.lines().filter_map(parse_region_line).collect()
}

fn parse_region_line(line: &str) -> Option<Region> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let dims: Vec<&str> = parts[0].trim().split('x').collect();
    if dims.len() != 2 {
        return None;
    }

    let width = dims[0].parse().ok()?;
    let height = dims[1].parse().ok()?;

    let counts: Vec<usize> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(Region {
        width,
        height,
        counts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shapes() {
        let input = "0:\n###\n##.\n##.";
        let shapes = parse_shapes(input);
        assert_eq!(shapes[0], 7);
    }

    #[test]
    fn test_parse_shapes_multiple() {
        let input = "0:\n###\n##.\n##.\n\n1:\n###\n##.\n.##";
        let shapes = parse_shapes(input);
        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0], 7);
        assert_eq!(shapes[1], 7);
    }

    #[test]
    fn test_parse_region_line() {
        let region = parse_region_line("4x4: 0 0 0 0 2 0").unwrap();
        assert_eq!(region.width, 4);
        assert_eq!(region.height, 4);
        assert_eq!(region.counts, vec![0, 0, 0, 0, 2, 0]);
    }

    #[test]
    fn test_parse_region_line_larger() {
        let region = parse_region_line("12x5: 1 0 1 0 2 2").unwrap();
        assert_eq!(region.width, 12);
        assert_eq!(region.height, 5);
        assert_eq!(region.counts, vec![1, 0, 1, 0, 2, 2]);
    }

    #[test]
    fn test_parse_region_line_empty() {
        assert!(parse_region_line("").is_none());
    }

    #[test]
    fn test_parse_region_line_invalid() {
        assert!(parse_region_line("invalid").is_none());
        assert!(parse_region_line("4x4").is_none());
    }

    #[test]
    fn test_solve_empty() {
        assert_eq!(solve_part1(""), 0);
    }

    #[test]
    fn test_area_check_fits() {
        // 4x4 = 16 cells, need 2*7 = 14 cells, 14 < 16 = true
        let input = "\
4:
###
#..
###

4x4: 0 0 0 0 2 0";
        assert_eq!(solve_part1(input), 1);
    }

    #[test]
    fn test_area_check_exact() {
        // 4x4 = 16 cells, need 16 cells exactly, 16 < 16 = false
        let input = "\
0:
####
####

4x4: 2";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_area_check_too_many() {
        // 4x4 = 16 cells, need 3*7 = 21 cells, 21 < 16 = false
        let input = "\
4:
###
#..
###

4x4: 0 0 0 0 3 0";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_main() {
        main();
    }
}
