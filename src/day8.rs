use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc2024::AoCResult;

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>) {
    let map = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut locations = HashMap::<char, Vec<(usize, usize)>>::new();
    let (height, width) = (map.len(), map[0].len());

    for y in 0..height {
        for x in 0..width {
            if map[y][x] != '.' {
                locations
                    .entry(map[y][x])
                    .or_insert(Vec::new())
                    .push((y, x));
            }
        }
    }

    (map, locations)
}

pub fn inside_grid(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (map, locations) = parse_input(input);
    let (height, width) = (map.len(), map[0].len());

    let mut antinodes = HashSet::new();
    for loc in locations.iter() {
        for pair in loc.1.iter().combinations(2) {
            let x = pair[0].1 as i32;
            let y = pair[0].0 as i32;
            let dy = pair[1].0 as i32 - y;
            let dx = pair[1].1 as i32 - x;

            if inside_grid(x - dx, y - dy, width, height) {
                antinodes.insert((x - dx, y - dy));
            }

            if inside_grid(x + 2 * dx, y + 2 * dy, width, height) {
                antinodes.insert((x + 2 * dx, y + 2 * dy));
            }
        }
    }
    let res = antinodes.len();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (map, locations) = parse_input(input);
    let (height, width) = (map.len(), map[0].len());

    let mut antinodes = HashSet::new();
    for loc in locations.iter() {
        for pair in loc.1.iter().combinations(2) {
            let x = pair[0].1 as i32;
            let y = pair[0].0 as i32;
            let dy = pair[1].0 as i32 - y;
            let dx = pair[1].1 as i32 - x;

            let mut i = -1;
            while inside_grid(x + i * dx, y + i * dy, width, height) {
                antinodes.insert((x + i * dx, y + i * dy));
                i -= 1;
            }

            let mut i = 0;
            while inside_grid(x + i * dx, y + i * dy, width, height) {
                antinodes.insert((x + i * dx, y + i * dy));
                i += 1;
            }
        }
    }
    let res = antinodes.len();
    AoCResult::Int(res as i64)
}
