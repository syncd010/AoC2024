
use std::collections::HashMap;

use aoc2024::AoCResult;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let vals = input.lines().filter(|line| !line.is_empty()).map(|line| {
        line.split(' ').filter(|v| !v.is_empty()).map(|v| { v.parse::<u64>().unwrap() }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let lst1 = vals.iter().map(|row| row[0]).collect::<Vec<_>>();
    let lst2 = vals.iter().map(|row| row[1]).collect::<Vec<_>>();
    return (lst1, lst2)
}

pub fn solve_part_one(input: &str) -> AoCResult {
    let (mut lst1, mut lst2) = parse_input(input);
    lst1.sort();
    lst2.sort();

    let diff: u64 = (0..lst1.len()).map(|i| {
        (lst1[i] as i64 - lst2[i] as i64).abs() as u64
    }).sum();
    AoCResult::Int(diff)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    let (lst1, lst2) = parse_input(input);
    // Build histogram of values in second list
    let mut histogram: HashMap<u64, u64> = HashMap::new();
    lst2.iter().for_each(|v| { *histogram.entry(*v).or_insert(0) += 1 });
    // Use histogram to calculate result
    let res = lst1.iter().map(|v| { v * histogram.get(&v).unwrap_or(&0) }).sum();
    return AoCResult::Int(res);
}
