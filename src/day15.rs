use aoc2024::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// Return the grid and the list of directions
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Dir>) {
    let (grid, directions) = input.trim().split_once("\n\n").unwrap();

    let grid = grid
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let directions = directions
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => Dir { x: -1, y: 0 },
                '^' => Dir { x: 0, y: -1 },
                '>' => Dir { x: 1, y: 0 },
                'v' => Dir { x: 0, y: 1 },
                _ => panic!("Unknown direction"),
            })
        })
        .collect_vec();
    (grid, directions)
}

fn _display_grid(grid: &[Vec<char>]) {
    grid.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}

fn find_start_pos(grid: &[Vec<char>]) -> Pos {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                return Pos { x, y };
            }
        }
    }
    panic!("Couldn't find starting position");
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (mut grid, directions) = parse_input(input);
    let mut pos = find_start_pos(&grid);

    for dir in directions {
        let mut next = pos + dir;
        // Find first free space or wall
        while grid[next.y][next.x] == 'O' {
            next += dir;
        }

        if grid[next.y][next.x] == '.' {
            // Move cells backwards
            while next != pos {
                let prev = next - dir;
                grid[next.y][next.x] = grid[prev.y][prev.x];
                next = prev;
            }
            grid[pos.y][pos.x] = '.';
            pos += dir;
        }
    }
    // _display_grid(&grid);

    let res: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| if c == 'O' { 100 * y + x } else { 0 })
        })
        .sum();
    AoCResult::Int(res as i64)
}

fn double_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec!['.'; width * 2]; height];
    let transform = HashMap::from([
        ('#', ['#', '#']),
        ('O', ['[', ']']),
        ('@', ['@', '.']),
        ('.', ['.', '.']),
    ]);

    for y in 0..height {
        for x in 0..width {
            let s = transform.get(&grid[y][x]).unwrap();
            new_grid[y][x * 2] = s[0];
            new_grid[y][x * 2 + 1] = s[1];
        }
    }
    new_grid
}

// From a position gets all connected cells in the given direction
// If a wall is found, returns None
fn expand_frontier(grid: &[Vec<char>], pos: Pos, dir: Dir) -> Option<HashSet<Pos>> {
    let mut frontier = Vec::from([pos]);
    let mut all = HashSet::from([pos]);
    let dir_left = Dir { x: -1, y: 0 };
    let dir_right = Dir { x: 1, y: 0 };

    while let Some(curr) = frontier.pop() {
        let next = curr + dir;
        if all.contains(&next) {
            continue;
        }
        let c = grid[next.y][next.x];
        match c {
            '.' => {} // Found edge
            ']' | '[' => {
                // Insert into frontier and all and keep going
                frontier.push(next);
                all.insert(next);
                if dir.y != 0 {
                    let other = next + if c == ']' { dir_left } else { dir_right };
                    frontier.push(other);
                    all.insert(other);
                }
            }
            _ => return None,
        }
    }
    Some(all)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (orig_grid, directions) = parse_input(input);
    let mut grid = double_grid(&orig_grid);
    let mut scratch_grid = grid.clone();
    let mut pos = find_start_pos(&grid);

    for d in directions {
        // Move all connected cells
        if let Some(frontier) = expand_frontier(&grid, pos, d) {
            // Backup to scratch
            for p in frontier.iter() {
                scratch_grid[p.y][p.x] = grid[p.y][p.x];
            }
            // Clear
            for p in frontier.iter() {
                grid[p.y][p.x] = '.';
            }
            // Move in dir
            for p in frontier.iter() {
                let new_p = *p + d;
                grid[new_p.y][new_p.x] = scratch_grid[p.y][p.x];
            }
            pos += d;
        }
    }

    let res: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| if c == '[' { 100 * y + x } else { 0 })
        })
        .sum();

    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input15Test"),
        include_str!("../data/input15"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [10092, 1451928];
    const EXPECTED_PART_TWO: [i64; 2] = [9021, 1462788];

    #[test]
    fn test_part_one() {
        for i in 0..2 {
            let res = solve_part_one(INPUT[i]);
            match res {
                AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_ONE[i]),
                _ => panic!("Wrong result type returned"),
            }
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
    }
}
