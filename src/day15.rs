use std::collections::{HashMap, HashSet};

use aoc2024::*;

// Return the grid and the list of directions
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Dir>) {
    let mut grid = Vec::new();
    let mut directions = Vec::new();
    let mut in_grid = true;
    for line in input.lines() {
        if line.is_empty() {
            in_grid = false;
        } else if in_grid {
            grid.push(line.chars().collect());
        } else {
            directions.extend(line.chars().map(|c| match c {
                '<' => Dir { x: -1, y: 0 },
                '^' => Dir { x: 0, y: -1 },
                '>' => Dir { x: 1, y: 0 },
                'v' => Dir { x: 0, y: 1 },
                _ => panic!("Unkown direction"),
            }));
        }
    }

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
            // Backup to scracth
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
