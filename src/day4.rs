use std::collections::HashMap;

use aoc2024::AoCResult;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn in_bounds<T>(val: T, min: T, max: T) -> bool
where
    T: PartialOrd,
{
    val >= min && val < max
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let dirs = [
        (1 as i32, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let lines = parse_input(input);
    let height = lines.len();
    let width = lines[0].len();
    let search = "XMAS";
    let mut res = 0;
    // For each position on the board, try to find the search string in all of the 8 directions
    for y in 0..height {
        for x in 0..width {
            // Shortcut, not strictly necessary
            if lines[y][x] == 'X' {
                for (dy, dx) in dirs {
                    let found = search.chars().enumerate().all(|(i, c_search)| {
                        let x_search = x as i32 + (i as i32) * dx;
                        let y_search = y as i32 + (i as i32) * dy;
                        in_bounds(x_search, 0, width as i32)
                            && in_bounds(y_search, 0, height as i32)
                            && c_search == lines[y_search as usize][x_search as usize]
                    });
                    res += found as i32;
                }
            }
        }
    }
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let lines = parse_input(input);
    let height = lines.len();
    let width = lines[0].len();
    let valid_map = HashMap::from([('S', 'M'), ('M', 'S')]);
    let mut res = 0;
    for y in 0..height {
        for x in 0..width {
            let inside_board = y > 0 && y < height - 1 && x > 0 && x < width - 1;
            if inside_board && lines[y][x] == 'A' {
                let corners = [
                    (lines[y - 1][x - 1], lines[y + 1][x + 1]),
                    (lines[y - 1][x + 1], lines[y + 1][x - 1]),
                ];
                let found = corners.iter().all(|corner| {
                    valid_map.contains_key(&corner.0) && *valid_map.get(&corner.0).unwrap() == corner.1
                });
                res += found as i32;
            }
        }
    }
    AoCResult::Int(res as i64)
}
