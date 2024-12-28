use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc2024::AoCResult;

// Return the map as a 2d array of chars, and a hashmap with the locations for each antenna type.
fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>) {
    let map = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

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
    for (_, loc) in locations.iter() {
        for pair in loc.iter().combinations(2) {
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
    for (_, loc) in locations.iter() {
        for pair in loc.iter().combinations(2) {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input8Test"),
        include_str!("../data/input8"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [14, 240];
    const EXPECTED_PART_TWO: [i64; 2] = [34, 955];

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
