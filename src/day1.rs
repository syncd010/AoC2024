use aoc2024::AoCResult;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let vals = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut it = line.split_whitespace();
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        });

    let lst1 = vals.clone().map(|row| row.0).collect_vec();
    let lst2 = vals.clone().map(|row| row.1).collect_vec();
    return (lst1, lst2);
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (mut lst1, mut lst2) = parse_input(input);
    lst1.sort();
    lst2.sort();

    let diff: i32 = (0..lst1.len()).map(|i| (lst1[i] - lst2[i]).abs()).sum();
    AoCResult::Int(diff as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (lst1, lst2) = parse_input(input);
    // Build histogram of values in second list
    let mut histogram: HashMap<i32, i32> = HashMap::new();
    lst2.iter()
        .for_each(|v| *histogram.entry(*v).or_insert(0) += 1);
    // Use histogram to calculate result
    let res: i32 = lst1
        .iter()
        .map(|v| v * histogram.get(&v).unwrap_or(&0))
        .sum();
    return AoCResult::Int(res as i64);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input1Test"),
        include_str!("../data/input1"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [11, 1722302];
    const EXPECTED_PART_TWO: [i64; 2] = [31, 20373490];

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
