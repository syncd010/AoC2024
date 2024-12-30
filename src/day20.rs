use aoc2024::*;
use itertools::Itertools;
use std::rc::Rc;

fn parse_input(input: &str) -> Vec<&[u8]> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .collect_vec();
    grid
}

fn find_in_grid(grid: &[&[u8]], sentinel: u8) -> Pos {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == sentinel {
                return Pos { x, y };
            }
        }
    }
    panic!("Couldn't find {sentinel}");
}

struct SearchState {
    pos: Pos,
    prev: Option<Rc<SearchState>>,
}

impl Drop for SearchState {
    fn drop(&mut self) {
        // From https://rust-unofficial.github.io/too-many-lists/third-drop.html
        let mut prev = self.prev.take();
        while let Some(node) = prev {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                prev = node.prev.take();
            } else {
                break;
            }
        }
    }
}

const DIRS: [Dir; 4] = [
    Dir { y: 1, x: 0 },
    Dir { y: 0, x: -1 },
    Dir { y: -1, x: 0 },
    Dir { y: 0, x: 1 },
];

fn find_path(grid: &[&[u8]], start: u8, end: u8) -> Vec<Pos> {
    let start_pos = find_in_grid(&grid, start);
    let mut frontier = Vec::from([Rc::new(SearchState {
        pos: start_pos,
        prev: None,
    })]);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut found = None;
    while let Some(state) = frontier.pop() {
        if visited[state.pos.y][state.pos.x] {
            continue;
        }
        visited[state.pos.y][state.pos.x] = true;
        for d in DIRS {
            let new_pos = state.pos + d;
            match grid[new_pos.y][new_pos.x] {
                b'.' => {
                    frontier.push(Rc::new(SearchState {
                        pos: new_pos,
                        prev: Some(Rc::clone(&state)),
                    }));
                }
                c if c == end => {
                    found = Some(Rc::new(SearchState {
                        pos: new_pos,
                        prev: Some(Rc::clone(&state)),
                    }));
                    break;
                }
                _ => (),
            }
        }
        if found.is_some() {
            break;
        }
    }
    if found.is_none() {
        panic!("Couldn't find a path");
    }
    let mut ptr = &found.unwrap();
    let mut path = Vec::from([ptr.pos]);
    while let Some(p) = ptr.prev.as_ref() {
        path.push(p.pos);
        ptr = ptr.prev.as_ref().unwrap();
    }
    path.reverse();
    path
}

fn build_grid_path(grid: &[&[u8]], path: &[Pos]) -> Vec<Vec<usize>> {
    let mut out = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    for (i, p) in path.iter().enumerate() {
        out[p.y][p.x] = i;
    }
    out
}

fn clamp(center: usize, spread: usize, min: usize, max: usize) -> (usize, usize) {
    let start = if center >= spread + min {
        center - spread
    } else {
        min
    };
    let end = if center + spread < max {
        center + spread + 1
    } else {
        max
    };
    (start, end)
}

fn count_cheats(grid: &[&[u8]], path: &[Pos], max_cheat_len: usize, min_save: usize) -> usize {
    assert!(min_save < path.len(), "Input is of the wrong format!");

    let grid_path = build_grid_path(&grid, &path);
    let mut res = 0;
    for start in &path[0..path.len() - min_save] {
        let i_start = grid_path[start.y][start.x];
        let (start_x, end_x) = clamp(start.x, max_cheat_len, 0, grid_path[0].len());
        let (start_y, end_y) = clamp(start.y, max_cheat_len, 0, grid_path.len());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let save = grid_path[y][x];
                if save == usize::MAX {
                    continue;
                }
                let dist = manhattan_dist(start, &Pos { y, x });
                if dist <= max_cheat_len
                    && save > i_start + dist
                    && save - i_start - dist >= min_save
                {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let grid = parse_input(input);
    let path = find_path(&grid, b'S', b'E');
    let res = count_cheats(&grid, &path, 2, 100);
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let grid = parse_input(input);
    let path = find_path(&grid, b'S', b'E');
    let res = count_cheats(&grid, &path, 20, 100);
    AoCResult::Int(res as i64)
}

fn manhattan_dist(p1: &Pos, p2: &Pos) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input20Test"),
        include_str!("../data/input20"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [0, 1296];
    const EXPECTED_PART_TWO: [i64; 2] = [0, 977665];

    #[test]
    fn test_part_one() {
        let grid = parse_input(INPUT[0]);
        let path = find_path(&grid, b'S', b'E');
        let res = count_cheats(&grid, &path, 2, 64);
        assert_eq!(res, 1);
        let res = count_cheats(&grid, &path, 2, 38);
        assert_eq!(res, 3);
        let res = count_cheats(&grid, &path, 2, 10);
        assert_eq!(res, 10);

        let res = solve_part_one(INPUT[1]);
        match res {
            AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_ONE[1]),
            _ => panic!("Wrong result type returned"),
        }
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input(INPUT[0]);
        let path = find_path(&grid, b'S', b'E');
        let res = count_cheats(&grid, &path, 20, 76);
        assert_eq!(res, 3);
        let res = count_cheats(&grid, &path, 20, 68);
        assert_eq!(res, 55);

        let res = solve_part_two(INPUT[1]);
        match res {
            AoCResult::Int(v) => assert_eq!(v, EXPECTED_PART_TWO[1]),
            _ => panic!("Wrong result type returned"),
        }
    }
}

// fn count_cheats(path: &[Pos], max_cheat_len: usize, min_save: usize) -> usize {
//     assert!(min_save < path.len());

//     let mut res = 0;
//     for (i_start, start) in path[0..path.len() - min_save].iter().enumerate() {
//         for (i_end, end) in path[i_start + min_save..].iter().enumerate() {
//             let dist = manhattan_dist(start, end);
//             if dist <= max_cheat_len {
//                 let save = i_end + min_save - dist;
//                 if save >= min_save {
//                     res += 1;
//                 }
//             }
//         }
//     }
//     res

//     // path[0..path.len() - min_save]
//     //     .iter()
//     //     .enumerate()
//     //     .flat_map(|(i_start, start)| {
//     //         path[i_start + min_save..]
//     //             .iter()
//     //             .enumerate()
//     //             .map(|(i_end, end)| {
//     //                 let dist = manhattan_dist(start, end);
//     //                 let save = i_end + min_save - dist;
//     //                 (dist, save)
//     //             })
//     //     })
//     //     .filter(|(dist, save)| *dist <= max_cheat_len && *save >= min_save)
//     //     .count()
// }

// pub fn solve_part_one(input: &str) -> AoCResult {
//     let grid = parse_input(input);
//     let path = find_path(&grid, b'S', b'E');
//     let res = count_cheats(&path, 2, 100);
//     AoCResult::Int(res as i64)
// }

// pub fn solve_part_two(input: &str) -> AoCResult {
//     let grid = parse_input(input);
//     let path = find_path(&grid, b'S', b'E');
//     let res = count_cheats(&path, 20, 100);
//     AoCResult::Int(res as i64)
// }
