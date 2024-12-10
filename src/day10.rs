use std::collections::HashSet;

use aoc2024::{AoCResult, Pos, Dir};


fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<Pos>) {
    let height_map = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let trailheads = height_map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, &c)| if c == 0 { Some(Pos { y, x }) } else { None })
        })
        .collect::<Vec<_>>();
    (height_map, trailheads)
}

const DIRS: [Dir; 4] = [
    Dir { y: 1, x: 0 },
    Dir { y: 0, x: -1 },
    Dir { y: -1, x: 0 },
    Dir { y: 0, x: 1 },
];

fn score(pos: Pos, height_map: &[Vec<u32>]) -> (u32, u32) {
    let mut unique_found: HashSet<Pos> = HashSet::new();
    let mut all_found = 0u32;
    let mut frontier = vec![pos];
    let dims = Pos {
        y: height_map.len(),
        x: height_map[0].len(),
    };

    while let Some(pos) = frontier.pop() {
        for d in DIRS {
            if pos.can_move_by(d, dims) {
                let new_pos = pos + d;

                if height_map[new_pos.y][new_pos.x] == height_map[pos.y][pos.x] + 1 {
                    if height_map[new_pos.y][new_pos.x] == 9 {
                        unique_found.insert(new_pos);
                        all_found += 1;
                    } else {
                        frontier.push(new_pos);
                    }
                }
            }
        }
    }

    return (unique_found.len() as u32, all_found);
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (height_map, trailheads) = parse_input(input);

    let res = trailheads
        .iter()
        .map(|t| score(*t, &height_map).0)
        .sum::<u32>();

    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (height_map, trailheads) = parse_input(input);

    let res = trailheads
        .iter()
        .map(|t| score(*t, &height_map).1)
        .sum::<u32>();

    AoCResult::Int(res as i64)
}
