use aoc2024::AoCResult;

use regex::Regex;

pub fn solve_part_one(input: &str) -> AoCResult {
    // Match mul and extract the numbers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res = re
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_inst, [a, b])| a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap())
        .sum();
    AoCResult::Int(res)
}

pub fn solve_part_two(input: &str) -> AoCResult {
    // Global regexp to match mul, do or don't
    let re_global = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    // Mul match to extract the numbers
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut flag = 1;
    let res = re_global
        .find_iter(input)
        .map(|m| match m.as_str() {
            "do()" => {
                flag = 1;
                0
            }
            "don't()" => {
                flag = 0;
                0
            }
            mul_match => {
                let caps = re_mul.captures(mul_match).unwrap();
                let a = caps.get(1).unwrap().as_str();
                let b = caps.get(2).unwrap().as_str();
                flag * a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap()
            }
        })
        .sum();
    AoCResult::Int(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 2] = [
        include_str!("../data/input3Test"),
        include_str!("../data/input3"),
    ];
    const EXPECTED_PART_ONE: [i64; 2] = [161, 167090022];
    const EXPECTED_PART_TWO: [i64; 2] = [48, 89823704];

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
