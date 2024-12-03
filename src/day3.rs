use aoc2024::AoCResult;

use regex::Regex;

pub fn solve_part_one(input: &str) -> AoCResult {
    // Match mul and extract the numbers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res: i32 = re
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_inst, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum::<i32>();
    AoCResult::Int(res as i64)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    // Global regexp to match mul, do or don't
    let re_global = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    // Mul match to extract the numbers
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut flag = 1;
    let res: i32 = re_global
        .find_iter(input)
        .map(|m| {
            match m.as_str() {
                "do()" => {
                    flag = 1;
                    0
                },
                "don't()" => {
                    flag = 0;
                    0
                },
                mul_match  => {
                    let caps = re_mul.captures(mul_match).unwrap();
                    let a = caps.get(1).unwrap().as_str();
                    let b = caps.get(2).unwrap().as_str();
                    flag * a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
                } 
            }
        })
        .sum();
    AoCResult::Int(res as i64)
}
