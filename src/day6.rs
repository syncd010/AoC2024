use aoc2024::AoCResult;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let map = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    map
}

// Find the starting point
fn find_start(map: &[Vec<char>]) -> (i32, i32) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                return (y as i32, x as i32);
            }
        }
    }
    unreachable!()
}

// fn print_board(map: &[Vec<char>]) -> () {
//     map.iter().for_each(|line| {
//         println!("{}", line.iter().collect::<String>());
//     });
// }

// Rotate by 90 degrees
fn rotate((y, x): (i32, i32)) -> (i32, i32) {
    (x, y * -1)
}

// Walks through the map with the given start, returns whether a loop was found and
// the visited positions as a 1D bitmask of directions
fn walk_map(map: &[Vec<char>], (mut y, mut x): (i32, i32)) -> (bool, Vec<u8>) {
    let height = map.len();
    let width = map[0].len();

    // 1D bitmask of visited positions, which stores the directions it was visited
    let mut visited = vec![0u8; height * width];
    let mut dir = (-1, 0);
    // Direction to use with bitmask
    let mut dir_idx = 0;
    let mut curr_dir = 1u8 << dir_idx;
    // While inside the board
    while x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
        // Check if already passed through here in the same direction
        let visited_idx = (y * width as i32 + x) as usize;
        if (visited[visited_idx] & curr_dir) != 0 {
            return (true, visited);
        }

        let (y_idx, x_idx) = (y as usize, x as usize);
        if map[y_idx][x_idx] == '#' {
            // Turn
            y -= dir.0;
            x -= dir.1;
            dir = rotate(dir);
            dir_idx = (dir_idx + 1) % 4;
            curr_dir = 1 << dir_idx;
        } else {
            // Mark visited bitmask
            visited[visited_idx] = visited[visited_idx] | curr_dir;
        }
        x += dir.1;
        y += dir.0;
    }

    (false, visited)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let map = parse_input(input);
    let (_loops, visited) = walk_map(&map, find_start(&map));

    let res = visited.iter().map(|&v| (v != 0) as i64).sum();
    AoCResult::Int(res)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let mut map = parse_input(input);
    let start = find_start(&map);
    let (_, visited) = walk_map(&map, start);
    let (height, width) = (map.len(), map[0].len());

    let mut res = 0i64;
    for y in 0..height {
        for x in 0..width {
            if visited[y * width + x] != 0 {
                map[y][x] = '#';
                let (loops, _) = walk_map(&map, start);
                if loops {
                    res += 1
                };
                map[y][x] = '.';
            }
        }
    }

    AoCResult::Int(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input6Test"),
        include_str!("../data/input6"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [41, 4663];
    const EXPECTED_PART_TWO: [i64; 2] = [6, 1530];

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
