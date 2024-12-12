use aoc2024::{AoCResult, Dir, Pos};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let vals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    vals
}

pub fn get_grid_dims<T>(grid: &[Vec<T>]) -> Pos {
    Pos {
        y: grid.len(),
        x: grid[0].len(),
    }
}

const DIRS: [Dir; 4] = [
    Dir { y: 1, x: 0 },
    Dir { y: 0, x: -1 },
    Dir { y: -1, x: 0 },
    Dir { y: 0, x: 1 },
];

pub fn solve(input: &str, part_two: bool) -> AoCResult {
    let grid = parse_input(input);
    let dims = get_grid_dims(&grid);
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
