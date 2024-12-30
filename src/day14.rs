use core::f64;

use aoc2024::*;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(Pos, Dir)> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (px, py) = p[2..].split_once(",").unwrap();
            let (vx, vy) = v[2..].split_once(",").unwrap();
            let pos = Pos {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            };
            let dir = Dir {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            };
            (pos, dir)
        })
        .collect::<Vec<_>>()
}

fn evolve(pos: usize, dir: isize, time: u32, limit: usize) -> usize {
    let aux = (pos as isize + time as isize * dir) % limit as isize;
    match aux < 0 {
        true => (limit as isize + aux) as usize,
        false => aux as usize,
    }
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let limits = (101, 103);
    let mid = (limits.0 / 2, limits.1 / 2);
    let t = 100;

    let res = parse_input(input)
        .iter()
        .filter_map(|(p, d)| {
            let new_x = evolve(p.x, d.x, t, limits.0);
            let new_y = evolve(p.y, d.y, t, limits.1);
            if new_x == mid.0 || new_y == mid.1 {
                None
            } else {
                Some((new_y > mid.1) as u8 * 2 + (new_x > mid.0) as u8)
            }
        })
        .counts()
        .iter()
        .fold(1, |acc, (&_k, &v)| acc * v);

    AoCResult::Int(res as i64)
}

// Variance of the given vector
fn variance(values: &[usize]) -> f64 {
    let mut mean = 0.0;
    let mut squared = 0.0;
    for &v in values {
        mean += v as f64;
        squared += (v * v) as f64;
    }
    mean /= values.len() as f64;
    squared /= values.len() as f64;
    squared - mean * mean
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let robots = &parse_input(input);
    let (limit_x, limit_y) = (101, 103);

    let mut pos_x = vec![0; robots.len()];
    let mut pos_y = vec![0; robots.len()];
    let (mut min_var_x, mut min_var_y) = (f64::MAX, f64::MAX);
    let (mut min_var_t_x, mut min_var_t_y) = (0, 0);

    // Find the minimum variance of points dispersion along each of the axis
    for t in 0..limit_x.max(limit_y) {
        for (i, (robot_pos, robot_dir)) in robots.iter().enumerate() {
            (pos_x[i], pos_y[i]) = (
                evolve(robot_pos.x, robot_dir.x, t as u32, limit_x),
                evolve(robot_pos.y, robot_dir.y, t as u32, limit_y),
            );
        }
        let var = (variance(&pos_x), variance(&pos_y));
        if var.0 < min_var_x {
            min_var_x = var.0;
            min_var_t_x = t;
        }
        if var.1 < min_var_y {
            min_var_y = var.1;
            min_var_t_y = t;
        }
    }

    // Poor man's chinese remainder theorem solution...
    let mut res = min_var_t_x;
    while !(res % limit_x == min_var_t_x && res % limit_y == min_var_t_y) {
        res += limit_x;
    }

    AoCResult::Int(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input14Test"),
        include_str!("../data/input14"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [21, 233709840];
    const EXPECTED_PART_TWO: [i64; 2] = [5253, 6620];

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
