use std::collections::HashMap;

use aoc2024::AoCResult;
use itertools::Itertools;

type CountsMap = HashMap<u64, usize>;

fn parse_input(input: &str) -> CountsMap {
    input
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .counts()
}

fn step_once(counts: CountsMap) -> CountsMap {
    let mut res = HashMap::new();
    for (&k, &v) in counts.iter() {
        let new_k1;
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

fn step_n(mut counts: CountsMap, n: usize) -> CountsMap {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input11Test"),
        include_str!("../data/input11"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [55312, 218956];
    const EXPECTED_PART_TWO: [i64; 2] = [65601038650482, 259593838049805];

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
