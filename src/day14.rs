use core::f64;

use aoc2024::*;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(Pos, Dir)> {
    let vals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut ps = line[line.find("p=").unwrap() + 2..line.find(" ").unwrap()].split(",");
            let pos = Pos {
                x: ps.next().unwrap().parse().unwrap(),
                y: ps.next().unwrap().parse().unwrap(),
            };

            let mut ds = line[line.find("v=").unwrap() + 2..].split(",");
            let dir = Dir {
                x: ds.next().unwrap().parse().unwrap(),
                y: ds.next().unwrap().parse().unwrap(),
            };
            (pos, dir)
        })
        .collect::<Vec<_>>();
    vals
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
    let limits = (101, 103);

    let mut pos_x = vec![0; robots.len()];
    let mut pos_y = vec![0; robots.len()];
    let mut min_var = (f64::MAX, f64::MAX);
    let mut min_var_idx = (0, 0);

    // Find the minimum variance of points dispersion along each of the axis
    for t in 0..limits.0.max(limits.1) {
        for (i, &r) in robots.iter().enumerate() {
            (pos_x[i], pos_y[i]) = (
                evolve(r.0.x, r.1.x, t as u32, limits.0),
                evolve(r.0.y, r.1.y, t as u32, limits.1),
            );
        }
        let var = (variance(&pos_x), variance(&pos_y));
        if var.0 < min_var.0 {
            min_var.0 = var.0;
            min_var_idx.0 = t;
        }
        if var.1 < min_var.1 {
            min_var.1 = var.1;
            min_var_idx.1 = t;
        }
    }

    // Poor man's chinese remainder theorem solution...
    let mut res = min_var_idx.0;
    while !(res % limits.0 == min_var_idx.0 && res % limits.1 == min_var_idx.1) {
        res += limits.0;
    }

    AoCResult::Int(res as i64)
}
