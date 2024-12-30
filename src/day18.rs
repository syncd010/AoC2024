use aoc2024::*;
use itertools::Itertools;
use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<Pos> {
    let vals = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Pos {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect_vec();
    vals
}

fn _display_grid(grid: &[Vec<char>]) {
    grid.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}

struct State {
    pos: Pos,
    time: usize,
}

const DIRS: [Dir; 4] = [
    Dir { y: 1, x: 0 },
    Dir { y: 0, x: -1 },
    Dir { y: -1, x: 0 },
    Dir { y: 0, x: 1 },
];

fn bfs(grid: &[Vec<usize>], cutoff_time: usize) -> Option<State> {
    let dims = Pos {
        x: grid[0].len(),
        y: grid.len(),
    };
    let mut frontier = VecDeque::from([State {
        pos: Pos { x: 0, y: 0 },
        time: 0,
    }]);
    let mut visited = vec![vec![false; dims.x]; dims.y];

    while let Some(next) = frontier.pop_front() {
        let pos = next.pos;
        // Check if already visited
        if visited[pos.y][pos.x] {
            continue;
        }
        visited[pos.y][pos.x] = true;

        // Check for goal
        if pos.x == dims.x - 1 && pos.y == dims.y - 1 {
            return Some(next);
        }

        // Successors
        for d in DIRS {
            if pos.can_move_by(d, dims) {
                let new_pos = pos + d;
                if !visited[new_pos.y][new_pos.x] && grid[new_pos.y][new_pos.x] >= cutoff_time {
                    // Still not corrupted
                    frontier.push_back(State {
                        pos: new_pos,
                        time: next.time + 1,
                    });
                }
            }
        }
    }
    None
}

fn build_grid(dims: Pos, positions: &[Pos]) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![usize::MAX; dims.x]; dims.y];

    for (i, &pos) in positions.iter().enumerate() {
        if grid[pos.y][pos.x] > i {
            grid[pos.y][pos.x] = i;
        }
    }
    grid
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let corrupted = parse_input(input);
    let dims = Pos { x: 71, y: 71 };
    let cutoff_time = 1024;
    // let dims = Pos { x: 7, y: 7 };
    // let cutoff_time = 12;
    let grid = build_grid(dims, &corrupted);
    let res = bfs(&grid, cutoff_time).expect("Couldn't find a path").time;
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let corrupted = parse_input(input);
    let dims = Pos { x: 71, y: 71 };
    // let dims = Pos { x: 7, y: 7 };
    let grid = build_grid(dims, &corrupted);

    // Binary search on time
    let mut min_time = 0;
    let mut max_time = corrupted.len();
    while max_time - min_time > 1 {
        let half_time = min_time + (max_time - min_time) / 2;
        let end = bfs(&grid, half_time);
        if end.is_none() {
            max_time = half_time;
        } else {
            min_time = half_time;
        }
    }
    let limit_time = min_time;
    let res = format!("{},{}", corrupted[limit_time].x, corrupted[limit_time].y);
    AoCResult::Str(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input18Test"),
        include_str!("../data/input18"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [22, 286];
    const EXPECTED_PART_TWO: [&str; 2] = ["6,1", "20,64"];

    #[test]
    fn test_part_one() {
        let corrupted = parse_input(INPUT[0]);
        let dims = Pos { x: 7, y: 7 };
        let cutoff_time = 12;
        let grid = build_grid(dims, &corrupted);
        let res = bfs(&grid, cutoff_time).expect("Couldn't find a path").time;
        assert_eq!(res as i64, EXPECTED_PART_ONE[0]);

        let res = solve_part_one(INPUT[1]);
        match res {
            AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_ONE[1]),
            _ => panic!("Wrong result type returned"),
        }
    }

    #[test]
    fn test_part_two() {
        let res = solve_part_two(INPUT[1]);
        match res {
            AoCResult::Str(v) => assert_eq!(v, EXPECTED_PART_TWO[1]),
            _ => panic!("Wrong result type returned"),
        }
    }
}
