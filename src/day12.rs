use aoc2024::{AoCResult, Dir, Pos};
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

const DIRS: [Dir; 4] = [
    Dir { y: 1, x: 0 },
    Dir { y: 0, x: -1 },
    Dir { y: -1, x: 0 },
    Dir { y: 0, x: 1 },
];

pub fn solve(input: &str, part_two: bool) -> AoCResult {
    let grid = parse_input(input);
    let dims = Pos {
        y: grid.len(),
        x: grid[0].len(),
    };
    // Whether each position has been visited
    let mut visited = vec![vec![false; dims.x]; dims.y];
    // For each position store a bitmask with bits set for the directions that are outside
    let mut outside = vec![vec![0u8; dims.x]; dims.y];
    let mut res = 0;

    // Iterate on grid
    for y in 0..dims.y {
        for x in 0..dims.x {
            if visited[y][x] {
                continue;
            }
            // Adjacent positions to be explored
            let mut frontier = vec![Pos { y, x }];
            let mut area = 0;
            let mut perimeter = 0;
            // DFS exploration
            while let Some(pos) = frontier.pop() {
                if visited[pos.y][pos.x] {
                    continue;
                }
                visited[pos.y][pos.x] = true;
                area += 1;
                let mut neighbors = Vec::new();
                for (i, &d) in DIRS.iter().enumerate() {
                    if pos.can_move_by(d, dims) {
                        let new_pos = pos + d;
                        if grid[new_pos.y][new_pos.x] == grid[y][x] {
                            // Same type as current, continue exploration and mark neighbor
                            frontier.push(new_pos);
                            neighbors.push(new_pos);
                            continue;
                        }
                    }
                    // Position on this direction is outside the grid or different, mark perimeter
                    outside[pos.y][pos.x] |= 1 << i;
                }

                perimeter += outside[pos.y][pos.x].count_ones();
                if part_two {
                    // Calculate common sides with neighbors and subtract
                    let mut common_sides = 0;
                    for neighbor in neighbors {
                        common_sides +=
                            (outside[pos.y][pos.x] & outside[neighbor.y][neighbor.x]).count_ones();
                    }
                    perimeter -= common_sides;
                }
            }
            res += area * perimeter;
        }
    }

    AoCResult::Int(res as i64)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    solve(input, false)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input12Test"),
        include_str!("../data/input12"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [1930, 1465112];
    const EXPECTED_PART_TWO: [i64; 2] = [1206, 893790];

    #[test]
    fn test_part_one() {
        for i in 0..2 {
            let res = solve_part_one(INPUT[i]);
            match res {
                AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_ONE[i]),
                _ => panic!("Wrong result type returned"),
            }
        }

        let input1 = "AAAA\nBBCD\nBBCC\nEEEC\n";
        let res = solve_part_one(input1);
        match res {
            AoCResult::Int(v) => assert_eq!(v, 140),
            _ => panic!("Wrong result type returned"),
        }
    }

    #[test]
    fn test_part_two() {
        for i in 0..2 {
            let res = solve_part_two(INPUT[i]);
            match res {
                AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_TWO[i]),
                _ => panic!("Wrong result type returned"),
            }
        }

        let input1 = "AAAA\nBBCD\nBBCC\nEEEC\n";
        let res = solve_part_two(input1);
        match res {
            AoCResult::Int(v) => assert_eq!(v, 80),
            _ => panic!("Wrong result type returned"),
        }
    }
}
