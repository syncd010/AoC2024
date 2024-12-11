use std::collections::HashMap;

use aoc2024::AoCResult;
use itertools::Itertools;

type CountsMaps = HashMap<u64, usize>;

fn parse_input(input: &str) -> CountsMaps {
    let vals = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .counts();
    vals
}

fn step_once(counts: CountsMaps) -> CountsMaps {
    let mut res = HashMap::new();
    for (&k, &v) in counts.iter() {
        let new_k1 ;
        let mut new_k2 = None;

        if k == 0 {
            new_k1 = 1;
        } else {
            let len = k.ilog10() + 1;
            if (len % 2) == 0 {
                new_k1 = k / 10u64.pow(len / 2);
                new_k2 = Some(k % 10u64.pow(len / 2));
            } else {
                new_k1 = k * 2024;
            }
        }
        *res.entry(new_k1).or_insert(0) += v;
        if let Some(k2) = new_k2 {
            *res.entry(k2).or_insert(0) += v;
        }
    }
    res
}

fn step_n(mut counts: CountsMaps, n: usize) -> CountsMaps {
    for _ in 0..n {
        counts = step_once(counts);
    }
    counts
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let steps = 25;
    let counts = step_n(parse_input(input), steps);
    let res: usize = counts.values().sum();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let steps = 75;
    let counts = step_n(parse_input(input), steps);
    let res: usize = counts.values().sum();
    AoCResult::Int(res as i64)
}
